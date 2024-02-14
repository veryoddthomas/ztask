use chrono::{DateTime, Local};
use std::fs::File;
use std::io::{self, Read, Write};

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
