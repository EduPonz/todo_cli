use std::fs::OpenOptions;
use std::io::Seek;

use chrono::{SecondsFormat, Utc};

use serde::Deserialize;
use serde::Serialize;

pub struct CSVManager();

#[derive(Serialize, Deserialize)]
struct CSVRecord {
    id: u64,
    description: String,
    status: String,
    last_updated: String,
}

impl CSVManager {
    pub fn add(description: String) -> Result<(), String> {
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
            let timestamp: String = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);

            let record = CSVRecord {
                id: last_id,
                status: "Pending".to_string(),
                description: description,
                last_updated: timestamp,
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

    pub fn list() -> Result<(), String> {
        let ret = Ok(());

        // Read the CSV file
        let mut rdr = match csv::Reader::from_path("todo.csv") {
            Ok(r) => r,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        let mut first_record: bool = true;

        for result in rdr.deserialize() {
            if first_record {
                first_record = false;
                println!(" ID | Status | Description | Last Updated");
                println!("----|--------|-------------|-------------");
            }
            let record: CSVRecord = result.unwrap();
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

    fn get_last_id() -> u64 {
        let mut last_id: u64 = 0;
        let increment: u64 = 1;

        // Read the CSV file
        let mut rdr = match csv::Reader::from_path("todo.csv") {
            Ok(r) => r,
            Err(_e) => {
                return last_id;
            }
        };

        let last_record: CSVRecord = match rdr.records().last() {
            Some(r) => r.unwrap().deserialize(None).unwrap(),
            None => {
                return last_id;
            }
        };

        last_id = last_record.id + increment;
        last_id
    }
}
