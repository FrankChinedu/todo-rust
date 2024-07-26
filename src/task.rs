use crate::db::DB;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    id: u8,
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String) -> Self {
        let mut rng = rand::thread_rng();
        let id = rng.gen_range(0..=std::u8::MAX);
        Task {
            id,
            description,
            completed: false,
        }
    }

    pub fn create(descriptions: Vec<String>) {
        let mut tasks: Vec<Task> = vec![];
        for val in descriptions {
            tasks.push(Self::new(val));
        }
        match DB::save(tasks) {
            Ok(res) => {
                println!("res : {res}")
            }
            Err(e) => {
                println!("DB create : {:?}", e)
            }
        };
    }

    pub fn list() {
        let tasks = DB::get_all_tasks().unwrap();
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(80)
            .set_header(vec![
                Cell::new("Index").fg(Color::Green),
                Cell::new("Description").fg(Color::Green),
                Cell::new("Completed").fg(Color::Green),
            ]);

        for task in tasks {
            table.add_row(vec![
                task.id.to_string(),
                task.description.to_string(),
                task.completed.to_string(),
            ]);
        }
        println!("{table}");
    }
}
