use chrono::{DateTime, Local};
use std::fs::File;
use std::io::Write;
use std::fs;
use std::io;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use colored::Colorize;

/// Task structure
#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub category: String,
    pub created_at: DateTime<Local>,
}

impl Task {
    pub fn new(name: String, category: String) -> Self {
        Task {
            id: Uuid::new_v4().simple().to_string(),
            name,
            category,
            created_at: Local::now(),
        }
    }
    pub fn to_string(&self, colorized: bool) -> String {
        let id=&self.id[0..9];
        // let created = self.created_at.format("%Y-%m-%d %H:%M").to_string();
        if colorized {
            format!("{}  {}  {}", id.bright_green(), self.name, self.category.yellow())
        } else {
            format!("{}  {}  {}", id, self.name, self.category)
        }
    }
}

pub struct TaskList {
    pub tasks: Vec<Task>,
    pub db_path: String,
}

impl Drop for TaskList {
    fn drop(&mut self) {
        self.save().unwrap();
        self.tasks.clear();
    }
}

impl TaskList {
    pub fn new(db_path: String) -> Self {
        // let db_file = DB_PATH.to_string();
        let tasks = TaskList::load(db_path.clone()).unwrap();
        TaskList { tasks, db_path }
    }

    pub fn print_list(&self) {
        for task in &self.tasks {
            println!("{}", task.to_string(true));
        }
    }

    pub fn num_tasks(&self) -> usize {
        self.tasks.len()
    }

    pub fn save(&self) -> Result<(), io::Error> {
        let serialized = serde_json::to_string_pretty(&self.tasks)?;
        let mut file = File::create(&self.db_path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn load(db_path: String) -> Result<Vec<Task>, io::Error> {
        let contents = fs::read_to_string(db_path)?;
        let tasks: Vec<Task> = serde_json::from_str(&contents)?;
        Ok(tasks)
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task)
    }

    pub fn remove_task(&mut self, id: String) {
        if id.len() < 4 {
            // TODO-001
            // This is for safety of usability.  We could remove this check
            // if we instead make sure the the prefix only matches one task.
            println!("Please specify a task id (at least 4 digits) to remove");
            return;
        }
        self.tasks.retain(|task| &task.id[0..id.len()] != id)
    }

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


// xref: /usr/local/develop/rust-commandline-example/src/main.rs

