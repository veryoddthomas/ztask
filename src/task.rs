use chrono::{DateTime, Local};
use std::fs::File;
use std::io::Write;
use std::fs;
use std::io;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use colored::Colorize;
use std::collections::VecDeque;


// I'm not sure yet if I want to implement
// a TaskType enum or if that would be
// too inflexible.

// #[derive(Serialize, Deserialize, Clone)]
// // #[serde(tag = "type")]
// pub enum TaskType {
//     #[serde(rename = "quick")]  Quick,
//     #[serde(rename = "learning")]   Learning,
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
// #[serde(tag = "type")]
pub enum TaskStatus {
    #[serde(rename = "active")]    Active,
    #[serde(rename = "backlog")]   Backlog,
    #[serde(rename = "blocked")]   Blocked,
    #[serde(rename = "completed")] Completed,
    #[serde(rename = "sleeping")]  Sleeping,  // Would Snoozed be better?
}

/// Task structure
#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub category: String,
    pub created_at: DateTime<Local>,
    pub status: TaskStatus,
}

impl Task {
    pub fn new(name: String, category: String) -> Self {
        Task {
            id: Uuid::new_v4().simple().to_string(),
            name,
            category,
            created_at: Local::now(),
            status: TaskStatus::Active,
        }
    }
    pub fn to_string(&self, colorized: bool) -> String {
        let id=&self.id[0..9];
        // let created = self.created_at.format("%Y-%m-%d %H:%M").to_string();
        if colorized {
            format!("{}  {}  {}", id.bright_green(), self.name, format!("{:?}",self.status).yellow())
        } else {
            format!("{}  {}  {}", id, self.name, format!("{:?}",self.status))
        }
    }
}

pub struct TaskList {
    pub tasks: VecDeque<Task>,
    pub db_path: String,
}

impl Drop for TaskList {
    fn drop(&mut self) {
        self.save().unwrap();
        self.tasks.clear();
    }
}

impl TaskList {
    /// Create a new task list.
    pub fn new(db_path: String) -> Self {
        let result = TaskList::load(db_path.clone());

        match result {
            Ok(tasks) => TaskList { tasks, db_path },
            Err(_) => {
                TaskList { tasks: VecDeque::new(), db_path }
            }
        }
    }

    /// Print the task list.
    pub fn print_list(&self) {
        for task in &self.tasks {
            println!("{}", task.to_string(true));
        }
    }

    /// Return the number of tasks in the list.
    pub fn num_tasks(&self) -> usize {
        self.tasks.len()
    }

    // #[cfg(test)]
    // /// Clears the TaskList, removing all Tasks.
    // pub fn clear(&mut self) {
    //     self.tasks.clear()
    // }

    /// Save the task list to the database file.
    pub fn save(&self) -> Result<(), io::Error> {
        let serialized = serde_json::to_string_pretty(&self.tasks)?;
        let mut file = File::create(&self.db_path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    /// Load the task list from the database file.
    pub fn load(db_path: String) -> Result<VecDeque<Task>, io::Error> {
        let contents = fs::read_to_string(db_path)?;
        let tasks: VecDeque<Task> = serde_json::from_str(&contents)?;
        Ok(tasks)
    }

    /// Add a task to the list.
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push_back(task)
    }

    /// Remove the task whose id starts with the id string passed in.
    pub fn remove_task(&mut self, id: String) {
        // If we don't find exactly one task that starts with 'id',
        // print a warning and return
        let match_count = self.tasks.iter().filter(|task| &task.id[0..id.len()] == id).count();
        if match_count !=1 {
            println!("Id '{}' does not uniquely match one task.  It matches {}", id, match_count);
            return;
        }
        self.tasks.retain(|task| &task.id[0..id.len()] != id)
    }

    // pub fn get_task(&self, id: Uuid) -> Option<&Task> {
    //     self.tasks.iter().find(|task| task.id == id)
    // }

    // pub fn get_tasks_by_category(&self, category: &str) -> VecDeque<&Task> {
    //     self.tasks.iter().filter(|task| task.category == category).collect()
    // }

    // pub fn get_tasks_by_name(&self, name: &str) -> VecDeque<&Task> {
    //     self.tasks.iter().filter(|task| task.name == name).collect()
    // }

    // pub fn get_tasks_by_name_and_category(&self, name: &str, category: &str) -> VecDeque<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.name == name && task.category == category)
    //        .collect()
    // }

    // pub fn get_tasks_by_category_and_name(&self, name: &str, category: &str) -> VecDeque<&Task> {
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
    // ) -> VecDeque<&Task> {
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
    // ) -> VecDeque<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.name == name && task.category == category && task.created_at == date)
    //        .collect()
    // }

    // pub fn get_tasks_by_name_and_date(&self, name: &str, date: &DateTime<Utc>) -> VecDeque<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.name == name && task.created_at == date)
    //        .collect()
    // }

    // pub fn get_tasks_by_category_and_date(&self, category: &str, date: &DateTime<Utc>) -> VecDeque<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.category == category && task.created_at == date)
    //        .collect()
    // }

    // pub fn get_tasks_by_date(&self, date: &DateTime<Utc>) -> VecDeque<&Task> {
    //     self.tasks.iter().filter(|task| task.created_at == date).collect()
    // }

    // pub fn get_tasks_by_name_and_category_and_date_range(
    //     &self,
    //     name: &str,
    //     category: &str,
    //     start_date: &DateTime<Utc>,
    //     end_date: &DateTime<Utc>,
    // ) -> VecDeque<&Task> {
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
    // ) -> VecDeque<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.name == name && task.category == category && task.created_at >= start_date && task.created_at <= end_date)
    //        .collect()
    // }

    // pub fn get_tasks_by_name_and_date_range(&self, name: &str, start_date: &DateTime<Utc>, end_date: &DateTime<Utc>) -> VecDeque<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.name == name && task.created_at >= start_date && task.created_at <= end_date)
    //        .collect()
    // }

    // pub fn get_tasks_by_category_and_date_range(&self, category: &str, start_date: &DateTime<Utc>, end_date: &DateTime<Utc>) -> VecDeque<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.category == category && task.created_at >= start_date && task.created_at <= end_date)
    //        .collect()
    // }

    // pub fn get_tasks_by_date_range(&self, start_date: &DateTime<Utc>, end_date: &DateTime<Utc>) -> VecDeque<&Task> {
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
    // ) -> VecDeque<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.name == name && task.category == category && task.created_at >= start_date && task.created_at <= end_date && task.created_at >= start_time && task.created_at <= end_time)
    //        .collect()
    // }
}


// xref: /usr/local/develop/rust-commandline-example/src/main.rs

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify default task settings
    #[test]
    fn check_task_defaults() {
        let task = Task::new("Get shit done".to_string(), "Category".to_string());

        assert_eq!(task.name, "Get stuff done".to_string());
        assert_eq!(task.category, "Category".to_string());
        assert_eq!(task.status, TaskStatus::Active);
        assert_eq!(task.id.len(), 32);
    }

}
