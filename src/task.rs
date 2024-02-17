use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process::Command;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Task {
    pub id: String,
    pub summary: String,
    pub details: String,
    pub priority: u8,
    pub category: String,
    pub created_at: DateTime<Local>,
    pub status: TaskStatus,
    pub blocked_by: VecDeque<String>,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        // In case of a priority tie we compare created_at - this step
        // is necessary to make implementations of `PartialEq` and
        // `Ord` consistent.
        // other.priority.cmp(&self.priority).reverse()
        // .then_with(|| self.created_at.cmp(&other.created_at))

        other.status.cmp(&self.status).reverse()
            .then_with(|| self.priority.cmp(&other.priority))
            .then_with(|| self.created_at.cmp(&other.created_at))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Task {
    pub fn new(summary: String, category: String) -> Self {
        Task {
            id: Uuid::new_v4().simple().to_string(),
            summary,
            details: "".to_string(),
            priority: 3,
            category,
            created_at: Local::now(),
            status: TaskStatus::Active,
            // blocked_by: VecDeque::from(["9d8607f24".to_string(), "c1ed178b5".to_string()]),
            blocked_by: VecDeque::new(),
        }
    }

    fn update_from(&mut self, other: &Task) {
        assert_eq!(self.id, other.id);
        self.priority = other.priority;
        self.summary = other.summary.clone();
        self.details = other.details.clone();
        self.category = other.category.clone();
        self.status = other.status.clone();
        self.blocked_by = other.blocked_by.clone();
    }

    // pub fn to_string(&self) -> String {
    //     let id = &self.id[0..9];
    //     // let created = self.created_at.format("%Y-%m-%d %H:%M").to_string();

    //     let summary = self.summary.to_string();
    //     let status = self.status.to_string();
    //     let blocked = if self.blocked_by.is_empty() {
    //         "".to_string()
    //     } else {
    //         self
    //             .blocked_by
    //             .iter()
    //             .map(|s| &s[..9])
    //             .collect::<Vec<_>>()
    //             .join(", ")
    //     };

    //     format!("{}  {}  {}  {}", id, summary, status, blocked)
    // }

    /// Invoke the default editor to edit the task
    pub fn invoke_editor(&mut self) -> Result<(), io::Error> {
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



// xref: /usr/local/develop/rust-commandline-example/src/main.rs

#[cfg(test)]
pub mod tests {
    use super::*;

    /// Verify default task settings
    #[test]
    fn check_task_defaults() {
        let task = Task::new("Get stuff done".to_string(), "Category".to_string());

        assert_eq!(task.summary, "Get stuff done".to_string());
        assert_eq!(task.category, "Category".to_string());
        assert_eq!(task.status, TaskStatus::Active);
        assert_eq!(task.id.len(), 32);
    }
}
