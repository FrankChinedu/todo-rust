use serde_json::{from_str, from_value, json, to_string_pretty, Value};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

use crate::task::Task;

#[allow(dead_code)]
pub struct DB;
const FILE_PATH: &str = "data.json";
impl DB {
    pub fn save(tasks: Vec<Task>) -> Result<String, Box<dyn std::error::Error>> {
        let mut file_value = Self::read_json_from_file()?;

        let data = DB::update_tasks(tasks, &mut file_value);

        Self::write_json_to_file(FILE_PATH, data)?;
        Ok("created".to_owned())
    }

    fn read_json_from_file() -> Result<Value, Box<dyn std::error::Error>> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(FILE_PATH)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(from_str(&contents).unwrap_or_else(|_| json!([])))
    }

    pub fn get_all_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        let values = DB::read_json_from_file()?;
        Ok(from_value(values).unwrap())
    }

    fn update_tasks(tasks: Vec<Task>, data: &mut Value) -> &mut Vec<Value> {
        let array = data.as_array_mut().unwrap();
        for task in tasks {
            let item = serde_json::to_value(task).unwrap();
            array.push(item);
        }
        array
    }

    fn write_json_to_file<P: AsRef<Path>>(
        path: P,
        data: &mut Vec<Value>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(path)?;
        let json_string = to_string_pretty(data)?;
        file.write_all(json_string.as_bytes())?;
        Ok(())
    }
}
