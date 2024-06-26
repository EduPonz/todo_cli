use std::fs::OpenOptions;
use std::io::Seek;

use chrono::{SecondsFormat, Utc};
use serde::Deserialize;
use serde::Serialize;

use crate::database::DatabaseManager;

pub struct CSVManager();

#[derive(Serialize, Deserialize)]
struct CSVRecord {
    id: u64,
    description: String,
    status: String,
    last_updated: String,
}

impl DatabaseManager for CSVManager {
    fn add(description: String) -> Result<(), String> {
        let mut ret = Ok(());
        println!("Adding todo item with description '{}'", description);

        let last_id = CSVManager::get_last_id();

        let mut csv_file = match OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open("todo.csv")
        {
            Ok(f) => f,
            Err(e) => {
                ret = Err(e.to_string());
                return ret;
            }
        };

        // Headers are needed only if the file is empty
        let needs_headers = match csv_file.seek(std::io::SeekFrom::End(0)) {
            Ok(0) => true,
            Ok(_) => false,
            Err(e) => {
                ret = Err(e.to_string());
                return ret;
            }
        };

        // Create a CSV writer
        let mut csv_writer = csv::WriterBuilder::new()
            .has_headers(needs_headers)
            .from_writer(csv_file);

        if !ret.is_err() {
            let record = CSVRecord {
                id: last_id,
                status: "pending".to_string(),
                description: description,
                last_updated: CSVManager::now_string(),
            };

            match csv_writer.serialize(record) {
                Ok(_) => (),
                Err(_e) => {
                    ret = Err("Failed to write to CSV file 'todo.csv'".to_string());
                }
            };

            match csv_writer.flush() {
                Ok(_) => (),
                Err(_e) => {
                    ret = Err("Failed to flush CSV file 'todo.csv'".to_string());
                }
            };
        }

        ret
    }

    fn list() -> Result<(), String> {
        let mut ret = Ok(());

        // Read the CSV file
        let mut csv_reader = match csv::Reader::from_path("todo.csv") {
            Ok(r) => r,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        let mut first_record: bool = true;

        for result in csv_reader.deserialize() {
            if first_record {
                first_record = false;
                println!(" ID | Status | Description | Last Updated");
                println!("----|--------|-------------|-------------");
            }

            let record: CSVRecord = match result {
                Ok(r) => r,
                Err(_e) => {
                    ret = Err("Incorrect record format found".to_string());
                    break;
                }
            };

            println!(
                " {} | {} | {} | {}",
                record.id, record.status, record.description, record.last_updated
            );
        }

        if first_record {
            println!("No todo items found");
        }

        ret
    }

    fn update_description(id: u64, description: String) -> Result<(), String> {
        CSVManager::update_field(id, "description".to_string(), description)
    }

    fn update_status(id: u64, status: String) -> Result<(), String> {
        CSVManager::update_field(id, "status".to_string(), status)
    }

    fn remove(id: u64) -> Result<(), String> {
        let mut ret;
        let mut records: Vec<CSVRecord> = Vec::new();

        let remove_record = |record: CSVRecord| {
            if id != record.id {
                records.push(record);
            }
        };

        ret = CSVManager::read_csv("todo.csv".to_string(), remove_record);

        if !ret.is_err() {
            ret = CSVManager::write_csv("todo.csv".to_string(), records);
        }

        ret
    }
}

impl CSVManager {
    fn read_csv<Predicate>(csv_file: String, mut predicate: Predicate) -> Result<(), String>
    where
        Predicate: FnMut(CSVRecord),
    {
        let mut ret = Ok(());

        let mut csv_reader = match csv::Reader::from_path(csv_file) {
            Ok(r) => r,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        for result in csv_reader.deserialize::<CSVRecord>() {
            let record: CSVRecord = match result {
                Ok(r) => r,
                Err(_e) => {
                    ret = Err("Incorrect record format found".to_string());
                    break;
                }
            };

            predicate(record);
        }

        ret
    }

    fn write_csv(csv_file: String, records: Vec<CSVRecord>) -> Result<(), String> {
        let mut ret = Ok(());

        // Open the CSV file for writing
        let csv_file = match OpenOptions::new()
            .write(true)
            .create(true)
            .append(false)
            .truncate(true)
            .open(csv_file)
        {
            Ok(f) => f,
            Err(e) => {
                ret = Err(e.to_string());
                return ret;
            }
        };

        let mut csv_writer = csv::WriterBuilder::new()
            .has_headers(true)
            .from_writer(csv_file);

        for record in records {
            match csv_writer.serialize(record) {
                Ok(_) => (),
                Err(_e) => {
                    ret = Err("Failed serialize record".to_string());
                }
            };
        }

        match csv_writer.flush() {
            Ok(_) => (),
            Err(_e) => {
                ret = Err("Failed to flush CSV file 'todo.csv'".to_string());
            }
        };

        ret
    }

    fn update_field(id: u64, field: String, value: String) -> Result<(), String> {
        let mut ret;
        let mut records: Vec<CSVRecord> = Vec::new();

        // Read the CSV file
        let push_record = |mut record: CSVRecord| {
            if id == record.id {
                match field.as_str() {
                    "description" => {
                        record.description = value.clone();
                    }
                    "status" => {
                        record.status = value.clone();
                    }
                    _ => (),
                }
                record.last_updated = CSVManager::now_string();
            }
            records.push(record);
        };

        ret = CSVManager::read_csv("todo.csv".to_string(), push_record);

        if !ret.is_err() {
            ret = CSVManager::write_csv("todo.csv".to_string(), records);
        }

        ret
    }

    fn get_last_id() -> u64 {
        let mut last_id: u64 = 0;
        let increment: u64 = 1;

        // Read the CSV file
        let mut csv_reader = match csv::Reader::from_path("todo.csv") {
            Ok(r) => r,
            Err(_e) => {
                return last_id;
            }
        };

        let last_record: CSVRecord = match csv_reader.records().last() {
            Some(r) => r.unwrap().deserialize(None).unwrap(),
            None => {
                return last_id;
            }
        };

        last_id = last_record.id + increment;
        last_id
    }

    fn now_string() -> String {
        Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)
    }
}
