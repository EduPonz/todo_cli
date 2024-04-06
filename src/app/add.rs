use std::fs::OpenOptions;
use std::io::Seek;
use std::time::SystemTime;

use serde::Serialize;

use crate::app::parser::Parser;

pub struct Adder();

impl Parser for Adder {
    fn print_help() -> () {
        println!(
            "Usage: {} add [options] \"<description>\"",
            env!("CARGO_PKG_NAME")
        );
        println!("");
        println!("Options:");
        println!("  -h, --help    Print this help message");
    }

    fn parse(args: Vec<String>) -> Result<(), String> {
        let mut ret = Ok(());

        // Check if there are enough arguments
        if args.len() == 0 {
            Self::print_help();
            ret = Err("Not enough arguments".to_string());
        }

        if !ret.is_err() {
            // Check add input
            match args[0].as_str() {
                "-h" | "--help" => {
                    Self::print_help();
                    ret = Ok(());
                }
                _ => {
                    ret = CSVAdder::add(args[0].as_str().to_string());
                }
            }
        }

        ret
    }
}

struct CSVAdder();

#[derive(Serialize)]
struct Record<'a> {
    id: &'a str,
    description: &'a str,
    status: &'a str,
    last_updated: &'a str,
}

impl CSVAdder {
    fn add(description: String) -> Result<(), String> {
        let mut ret = Ok(());
        println!("Adding todo item with description '{}'", description);

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
            let timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string();

            let record = Record {
                id: &timestamp,
                status: "Pending",
                description: &description,
                last_updated: &timestamp,
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
}
