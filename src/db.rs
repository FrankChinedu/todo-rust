use serde_json::{from_str, json, to_string_pretty, Value};
use std::fs::OpenOptions;
use std::io::{Read, Write};

use crate::task::Task;

#[allow(dead_code)]
pub struct DB;

impl DB {
    pub fn save(tasks: Vec<Task>) -> Result<String, String> {
        let file_option = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open("data.json");

        match file_option {
            Ok(mut file) => {
                let mut contents = String::new();
                match file.read_to_string(&mut contents) {
                    Ok(_) => {
                        let mut data: Value = from_str(&contents).unwrap_or_else(|_| json!([]));

                        if let Some(array) = data.as_array_mut() {
                            for task in tasks {
                                let item = serde_json::to_value(task).unwrap();
                                array.push(item);
                            }
                        } else {
                            for task in tasks {
                                let item = serde_json::to_value(task).unwrap();
                                data = Value::Array(vec![data, item]);
                            }
                        }
                        let updated_json = to_string_pretty(&data).unwrap();
                        return match file.write_all(updated_json.as_bytes()) {
                            Ok(_) => {
                                println!("created");
                                Ok("created".to_owned())
                            }
                            Err(e) => {
                                eprintln!("lasr e {:?}", e);
                                Err("Last erro".to_owned())
                            }
                        };
                    }
                    Err(e) => {
                        eprintln!("e {:?}", e);
                        Err("FAILED".to_owned())
                    }
                }
            }
            Err(e) => {
                println!("unaable to read {:?}", e);
                Err("error occured".to_owned())
            }
        }
    }
}
