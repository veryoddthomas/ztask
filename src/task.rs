use chrono::prelude::*;
use serde_json;
use std::fs::File;
use std::io::Write;
use std::fs;
use std::io;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const DB_PATH: &str = "./data/db.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub category: String,
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(name: String, category: String) -> Self {
        Task {
            id: Uuid::new_v4(),
            name,
            category,
            created_at: Utc::now(),
        }
    }
}

pub struct TaskList {
    pub tasks: Vec<Task>,
    pub db_file: String,
}

impl Drop for TaskList {
    fn drop(&mut self) {
        // println!("Destroy TaskList");
        self.save().unwrap();
        self.tasks.clear();
    }
}

impl TaskList {
    pub fn new() -> Self {
        let db_file = DB_PATH.to_string();
        let tasks = TaskList::load().unwrap();
        TaskList { tasks: tasks, db_file: db_file }
    }

    pub fn print_list(&self) {
        for task in &self.tasks {
            println!("Task: {} {}", task.id, task.name);
        }
    }

    pub fn save(&self) -> Result<(), io::Error> {
        let serialized = serde_json::to_string_pretty(&self.tasks)?;
        let mut file = File::create(DB_PATH)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn load() -> Result<Vec<Task>, io::Error> {
        let contents = fs::read_to_string(DB_PATH)?;
        let tasks: Vec<Task> = serde_json::from_str(&contents)?;
        Ok(tasks)
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task)
    }

    // pub fn remove_task(&mut self, id: Uuid) {
    //     self.tasks.retain(|task| task.id!= id)
    // }

    // pub fn get_task(&self, id: Uuid) -> Option<&Task> {
    //     self.tasks.iter().find(|task| task.id == id)
    // }

    // pub fn get_tasks_by_category(&self, category: &str) -> Vec<&Task> {
    //     self.tasks.iter().filter(|task| task.category == category).collect()
    // }

    // pub fn get_tasks_by_name(&self, name: &str) -> Vec<&Task> {
    //     self.tasks.iter().filter(|task| task.name == name).collect()
    // }

    // pub fn get_tasks_by_name_and_category(&self, name: &str, category: &str) -> Vec<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.name == name && task.category == category)
    //        .collect()
    // }

    // pub fn get_tasks_by_category_and_name(&self, name: &str, category: &str) -> Vec<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.name == name && task.category == category)
    //        .collect()
    // }

    // pub fn get_tasks_by_name_and_category_and_date(
    //     &self,
    //     name: &str,
    //     category: &str,
    //     date: &DateTime<Utc>,
    // ) -> Vec<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.name == name && task.category == category && task.created_at == date)
    //        .collect()
    // }

    // pub fn get_tasks_by_category_and_name_and_date(
    //     &self,
    //     name: &str,
    //     category: &str,
    //     date: &DateTime<Utc>,
    // ) -> Vec<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.name == name && task.category == category && task.created_at == date)
    //        .collect()
    // }

    // pub fn get_tasks_by_name_and_date(&self, name: &str, date: &DateTime<Utc>) -> Vec<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.name == name && task.created_at == date)
    //        .collect()
    // }

    // pub fn get_tasks_by_category_and_date(&self, category: &str, date: &DateTime<Utc>) -> Vec<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.category == category && task.created_at == date)
    //        .collect()
    // }

    // pub fn get_tasks_by_date(&self, date: &DateTime<Utc>) -> Vec<&Task> {
    //     self.tasks.iter().filter(|task| task.created_at == date).collect()
    // }

    // pub fn get_tasks_by_name_and_category_and_date_range(
    //     &self,
    //     name: &str,
    //     category: &str,
    //     start_date: &DateTime<Utc>,
    //     end_date: &DateTime<Utc>,
    // ) -> Vec<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.name == name && task.category == category && task.created_at >= start_date && task.created_at <= end_date)
    //        .collect()
    // }

    // pub fn get_tasks_by_category_and_name_and_date_range(
    //     &self,
    //     name: &str,
    //     category: &str,
    //     start_date: &DateTime<Utc>,
    //     end_date: &DateTime<Utc>,
    // ) -> Vec<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.name == name && task.category == category && task.created_at >= start_date && task.created_at <= end_date)
    //        .collect()
    // }

    // pub fn get_tasks_by_name_and_date_range(&self, name: &str, start_date: &DateTime<Utc>, end_date: &DateTime<Utc>) -> Vec<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.name == name && task.created_at >= start_date && task.created_at <= end_date)
    //        .collect()
    // }

    // pub fn get_tasks_by_category_and_date_range(&self, category: &str, start_date: &DateTime<Utc>, end_date: &DateTime<Utc>) -> Vec<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.category == category && task.created_at >= start_date && task.created_at <= end_date)
    //        .collect()
    // }

    // pub fn get_tasks_by_date_range(&self, start_date: &DateTime<Utc>, end_date: &DateTime<Utc>) -> Vec<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.created_at >= start_date && task.created_at <= end_date)
    //        .collect()
    // }

    // pub fn get_tasks_by_name_and_category_and_date_range_and_time_range(
    //     &self,
    //     name: &str,
    //     category: &str,
    //     start_date: &DateTime<Utc>,
    //     end_date: &DateTime<Utc>,
    //     start_time: &DateTime<Utc>,
    //     end_time: &DateTime<Utc>,
    // ) -> Vec<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.name == name && task.category == category && task.created_at >= start_date && task.created_at <= end_date && task.created_at >= start_time && task.created_at <= end_time)
    //        .collect()
    // }
}



// pub fn read_db() -> Result<Vec<Task>, Error> {
//     let db_content = fs::read_to_string(DB_PATH)?;
//     let parsed: Vec<Task> = serde_json::from_str(&db_content)?;
//     Ok(parsed)
// }

// /// Write tasks to DB (json file)
// pub fn write_db(tasks: &Vec<Task>) {
//     let json = serde_json::to_string(tasks).unwrap();

//     let mut file = File::create(DB_PATH).unwrap();
//     file.write_all(json.as_bytes()).unwrap();
// }

// xref: /usr/local/develop/rust-commandline-example/src/main.rs
