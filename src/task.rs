use chrono::prelude::*;
use serde_json;
use std::fs::File;
use std::io::Write;
use std::fs;
use std::io;
use thiserror::Error;
use serde::{Deserialize, Serialize};

const DB_PATH: &str = "./data/db.json";

// ref: https://docs.rs/thiserror/latest/thiserror/

//TODO: messages used below don't seem to be working as expected
#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the Task DB file: {0}")]
    ReadTaskDBError(#[from] io::Error),

    #[error("error parsing the Task DB file: {0}")]
    ParseTaskDBError(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: usize,
    pub name: String,
    pub category: String,
    pub created_at: DateTime<Utc>,
}

pub fn read_db() -> Result<Vec<Task>, Error> {
    let db_content = fs::read_to_string(DB_PATH)?;
    let parsed: Vec<Task> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

/// Write tasks to DB (json file)
pub fn write_db(tasks: &Vec<Task>) {
    let json = serde_json::to_string(tasks).unwrap();

    let mut file = File::create(DB_PATH).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}


// xref: /usr/local/develop/rust-commandline-example/src/main.rs

// fn add_task_to_db() -> Result<Vec<Task>, Error> {
//     let mut rng = rand::thread_rng();
//     let db_content = fs::read_to_string(DB_PATH)?;
//     let mut parsed: Vec<Task> = serde_json::from_str(&db_content)?;
//     let catsdogs = match rng.gen_range(0, 1) {
//         0 => "cats",
//         _ => "dogs",
//     };

//     let random_pet = Task {
//         id: rng.gen_range(0, 9999999),
//         name: rng.sample_iter(Alphanumeric).take(10).collect(),
//         category: catsdogs.to_owned(),
//         age: rng.gen_range(1, 15),
//         created_at: Utc::now(),
//     };

//     parsed.push(random_pet);
//     fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
//     Ok(parsed)
// }

// fn remove_pet_at_index(pet_list_state: &mut ListState) -> Result<(), Error> {
//     if let Some(selected) = pet_list_state.selected() {
//         let db_content = fs::read_to_string(DB_PATH)?;
//         let mut parsed: Vec<Pet> = serde_json::from_str(&db_content)?;
//         parsed.remove(selected);
//         fs::write(DB_PATH, &serde_json::to_vec(&parsed)?)?;
//         let amount_pets = read_db().expect("can fetch pet list").len();
//         if selected > 0 {
//             pet_list_state.select(Some(selected - 1));
//         } else {
//             pet_list_state.select(Some(0));
//         }
//     }
//     Ok(())
// }
