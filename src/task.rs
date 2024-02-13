use chrono::{DateTime, Local};
use std::fs::File;
use std::io::{self, Read, Write};

use std::fs;
use std::env;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use colored::Colorize;
use std::collections::VecDeque;

use std::process::Command;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TaskStatus {
    #[serde(rename = "active")]    Active,
    #[serde(rename = "backlog")]   Backlog,
    #[serde(rename = "blocked")]   Blocked,
    #[serde(rename = "completed")] Completed,
    #[serde(rename = "sleeping")]  Sleeping,  // Would Snoozed be better?
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Active => write!(f, "active"),
            TaskStatus::Backlog => write!(f, "backlog"),
            TaskStatus::Blocked => write!(f, "blocked"),
            TaskStatus::Completed => write!(f, "completed"),
            TaskStatus::Sleeping => write!(f, "sleeping"),
        }
    }
}

/// Task structure
#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub summary: String,
    pub details: String,
    pub category: String,
    pub created_at: DateTime<Local>,
    pub status: TaskStatus,
    pub blocked_by: VecDeque<String>,
}

impl Task {
    pub fn new(summary: String, category: String) -> Self {
        Task {
            id: Uuid::new_v4().simple().to_string(),
            summary,
            details: "".to_string(),
            category,
            created_at: Local::now(),
            status: TaskStatus::Active,
            // blocked_by: VecDeque::from(["9d8607f24".to_string(), "c1ed178b5".to_string()]),
            blocked_by: VecDeque::new(),
        }
    }

    fn update_from(&mut self, other: &Task) {
        assert_eq!(self.id, other.id);
        self.summary = other.summary.clone();
        self.details = other.details.clone();
        self.category = other.category.clone();
        self.status = other.status.clone();
        self.blocked_by = other.blocked_by.clone();
    }

    pub fn to_string(&self, colorized: bool) -> String {
        let id = &self.id[0..9];
        // let created = self.created_at.format("%Y-%m-%d %H:%M").to_string();

        let summary = self.summary.to_string();
        let status = self.status.to_string();
        let blocked = if self.blocked_by.is_empty() {
            "".to_string()
        } else {
            self
                .blocked_by
                .iter()
                .map(|s| &s[..9])
                .collect::<Vec<_>>()
                .join(", ")
        };

        if colorized {
            let id = id.bright_green();
            let summary = summary.bright_black();
            let status = status.bright_white();
            let blocked = blocked.bright_red();
            format!("{}  {}  {}  {}", id, summary, status, blocked)
        } else {
            format!("{}  {}  {}  {}", id, summary, status, blocked)
        }
    }

    /// Invoke the default editor to edit the task
    fn invoke_editor(&mut self) -> Result<(), io::Error> {
        let serialized = serde_json::to_string_pretty(&self)?;

        // Create a temporary file
        let mut temp_file = tempfile::Builder::new().suffix(".json").tempfile()?;

        // Write some content to the temporary file
        writeln!(temp_file, "{}", serialized)?;

        // Get the path to the temporary file
        let file_path = temp_file.path();

        // Determine the default editor based on the environment variables
        let editor = env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());

        if !cfg!(test) {
            // Invoke the default editor to open the temporary file,
            // as long as we're not running tests
            Command::new(editor)
                .arg(file_path)
                .status()
                .expect("Failed to open the editor");
            }

        // Reopen the temporary file for reading
        let file_path = temp_file.path();
        let mut file = File::open(file_path)?;

        // Read the entire contents into a buffer
        let mut updates = String::new();
        file.read_to_string(&mut updates)?;

        // Deserialize the buffer into a Task.  If it can't be parsed,
        // default to the original task values
        let updated_task: Task = serde_json::from_str(&updates).unwrap_or(self.clone());
        self.update_from(&updated_task);

        Ok(())
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
    pub fn add_task(&mut self, task: Task) -> String {
        let id = task.id.clone();
        self.tasks.push_back(task);
        id
    }

    /// Remove the task whose id starts with the id string passed in.
    pub fn remove_task(&mut self, id: String) {
        // If we don't find exactly one task that starts with 'id',
        // print a warning and return
        let match_count = self.tasks.iter().filter(|task| task.id[0..id.len()] == id).count();
        if match_count !=1 {
            println!("Id '{}' does not uniquely match one task.  It matches {}", id, match_count);
            return;
        }
        self.tasks.retain(|task| task.id[0..id.len()] != id)
    }

    /// Edit the task whose id starts with the id string passed in.
    pub fn edit_task(&mut self, id: String) -> usize{
        let tasks = self.tasks.iter().filter(|task| task.id[0..id.len()] == id);
        let match_count = tasks.count();
        if match_count !=1 {
            println!("Id '{}' does not uniquely match one task.  It matches {}", id, match_count);
            return 0;
        }

        let result = self.tasks.iter_mut().find(|task| task.id[0..id.len()] == id);

        if let Some(task) = result {
            task.invoke_editor().unwrap_or_default();
            1
        } else {
            println!("Task {id} not found");
            0
        }
    }


    // pub fn get_task(&self, id: String) -> Option<&Task> {
    //     self.tasks.iter().find(|task| task.id == id)
    // }

    // pub fn get_tasks_by_category(&self, category: &str) -> VecDeque<&Task> {
    //     self.tasks.iter().filter(|task| task.category == category).collect()
    // }

    // pub fn get_tasks_by_summary(&self, summary: &str) -> VecDeque<&Task> {
    //     self.tasks.iter().filter(|task| task.summary == summary).collect()
    // }

    // pub fn get_tasks_by_summary_and_category(&self, summary: &str, category: &str) -> VecDeque<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.summary == summary && task.category == category)
    //        .collect()
    // }

    // pub fn get_tasks_by_category_and_summary(&self, summary: &str, category: &str) -> VecDeque<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.summary == summary && task.category == category)
    //        .collect()
    // }

    // pub fn get_tasks_by_summary_and_category_and_date(
    //     &self,
    //     summary: &str,
    //     category: &str,
    //     date: &DateTime<Utc>,
    // ) -> VecDeque<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.summary == summary && task.category == category && task.created_at == date)
    //        .collect()
    // }

    // pub fn get_tasks_by_category_and_summary_and_date(
    //     &self,
    //     summary: &str,
    //     category: &str,
    //     date: &DateTime<Utc>,
    // ) -> VecDeque<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.summary == summary && task.category == category && task.created_at == date)
    //        .collect()
    // }

    // pub fn get_tasks_by_summary_and_date(&self, summary: &str, date: &DateTime<Utc>) -> VecDeque<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.summary == summary && task.created_at == date)
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

    // pub fn get_tasks_by_summary_and_category_and_date_range(
    //     &self,
    //     summary: &str,
    //     category: &str,
    //     start_date: &DateTime<Utc>,
    //     end_date: &DateTime<Utc>,
    // ) -> VecDeque<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.summary == summary && task.category == category && task.created_at >= start_date && task.created_at <= end_date)
    //        .collect()
    // }

    // pub fn get_tasks_by_category_and_summary_and_date_range(
    //     &self,
    //     summary: &str,
    //     category: &str,
    //     start_date: &DateTime<Utc>,
    //     end_date: &DateTime<Utc>,
    // ) -> VecDeque<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.summary == summary && task.category == category && task.created_at >= start_date && task.created_at <= end_date)
    //        .collect()
    // }

    // pub fn get_tasks_by_summary_and_date_range(&self, summary: &str, start_date: &DateTime<Utc>, end_date: &DateTime<Utc>) -> VecDeque<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.summary == summary && task.created_at >= start_date && task.created_at <= end_date)
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

    // pub fn get_tasks_by_summary_and_category_and_date_range_and_time_range(
    //     &self,
    //     summary: &str,
    //     category: &str,
    //     start_date: &DateTime<Utc>,
    //     end_date: &DateTime<Utc>,
    //     start_time: &DateTime<Utc>,
    //     end_time: &DateTime<Utc>,
    // ) -> VecDeque<&Task> {
    //     self.tasks
    //        .iter()
    //        .filter(|task| task.summary == summary && task.category == category && task.created_at >= start_date && task.created_at <= end_date && task.created_at >= start_time && task.created_at <= end_time)
    //        .collect()
    // }
}


// xref: /usr/local/develop/rust-commandline-example/src/main.rs

#[cfg(test)]
pub mod tests {
    use super::*;

    /// Create a temporary test database with the given number of tasks
    pub fn __create_temp_db(initial_task_count: i32) -> String {
        use std::path::Path;
        fs::create_dir_all("data/temp").unwrap();
        let db = format!("data/temp/{}-test.json",Uuid::new_v4().simple());
        if Path::new(&db).exists() {
            panic!("Temporary test database already exists: {}", db);
        }

        let mut task_list = TaskList::new(db.clone());
        for i in 0..initial_task_count {
            task_list.add_task(Task::new(format!("test task {i}").to_string(), "quick".to_string()));
        }
        db
    }

    /// Remove the named test database
    pub fn __destroy_temp_db(test_db: String) -> String {
        use std::path::Path;

        if test_db.starts_with("data/temp") && test_db.ends_with("-test.json") && Path::new(&test_db).exists() {
            let _ = fs::remove_file(&test_db);
        }
        test_db.to_string()
    }

    /// Verify default task settings
    #[test]
    fn check_task_defaults() {
        let task = Task::new("Get stuff done".to_string(), "Category".to_string());

        assert_eq!(task.summary, "Get stuff done".to_string());
        assert_eq!(task.category, "Category".to_string());
        assert_eq!(task.status, TaskStatus::Active);
        assert_eq!(task.id.len(), 32);
    }

    #[test]
    fn verify_remove_single() {
        let db = __create_temp_db(2);
        let mut task_list = TaskList::new(db.clone());
        let id = task_list.tasks.get(1).unwrap().id.clone();

        task_list.remove_task(id);
        assert_eq!(task_list.tasks.len(), 1);

        drop(task_list);
        __destroy_temp_db(db);
    }

    #[test]
    fn verify_edit_single() {
        let db = __create_temp_db(2);
        let mut task_list = TaskList::new(db.clone());
        let id = task_list.tasks.get(1).unwrap().id.clone();

        task_list.edit_task(id);
        assert_eq!(task_list.tasks.len(), 2);

        drop(task_list);
        __destroy_temp_db(db);
    }
}
