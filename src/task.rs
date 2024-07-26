use crate::db::DB;
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
}
