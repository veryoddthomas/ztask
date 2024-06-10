use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process::Command;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TaskStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "backlog")]
    Backlog,
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "sleeping")]
    Sleeping, // Would Snoozed be better?
    #[serde(rename = "completed")]
    Completed,
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
    pub blocked_by: BTreeSet<String>,
    pub wake_at: Option<DateTime<Local>>,
}

impl Ord for Task {
    // Note, we have to reverse the ordering since the BinaryHeap is a max-heap
    // (descending order) and we want to sort in ascending order.

    fn cmp(&self, other: &Self) -> Ordering {
        if self.status == TaskStatus::Active && other.status == TaskStatus::Active {
            // These should be sorted in descending order by date (only)
            return self.created_at.cmp(&other.created_at);
        }
        // In case of a priority tie we compare created_at - this step
        // is necessary to make implementations of `PartialEq` and
        // `Ord` consistent.
        other
            .status
            .cmp(&self.status)
            .then_with(|| other.priority.cmp(&self.priority))
            .then_with(|| other.created_at.cmp(&self.created_at))
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(self))
    }
}

impl Task {
    pub fn new(summary: String, category: String, is_interrupt: bool) -> Self {
        Task {
            id: Uuid::new_v4().simple().to_string(),
            summary,
            details: "".to_string(),
            priority: 3,
            category,
            created_at: Local::now(),
            status: match is_interrupt {
                true => TaskStatus::Active,
                false => TaskStatus::Backlog,
            },
            // blocked_by: VecDeque::from(["9d8607f24".to_string(), "c1ed178b5".to_string()]),
            blocked_by: BTreeSet::new(),
            wake_at: None,
        }
    }

    fn update_from(&mut self, other: &Task) {
        assert_eq!(self.id, other.id);
        self.priority = other.priority;
        self.summary.clone_from(&other.summary);
        self.details.clone_from(&other.details);
        self.category.clone_from(&other.category);
        self.status.clone_from(&other.status);
        self.blocked_by.clone_from(&other.blocked_by);
        self.wake_at.clone_from(&other.wake_at);
    }

    pub fn block_on(&mut self, blocker_id: String) {
        self.blocked_by.insert(blocker_id);
        self.status = TaskStatus::Blocked;
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

    /// Invoke the default editor to edit the task
    pub fn invoke_editor_for_details(&mut self) -> Result<(), io::Error> {
        // let serialized = serde_json::to_string_pretty(&self)?;

        // Create a temporary file
        let mut temp_file = tempfile::Builder::new().suffix(".txt").tempfile()?;

        // Write some content to the temporary file
        writeln!(temp_file, "{}", self.details)?;

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
        self.details = updates.trim().to_string();

        // Deserialize the buffer into a Task.  If it can't be parsed,
        // default to the original task values
        // let updated_descripiton: Task = serde_json::from_str(&updates).unwrap_or(self.clone());
        // self.update_from(&updated_task);

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
        let task = Task::new("Get stuff done".to_string(), "Category".to_string(), true);

        assert_eq!(task.summary, "Get stuff done".to_string());
        assert_eq!(task.category, "Category".to_string());
        assert_eq!(task.status, TaskStatus::Active);
        assert_eq!(task.id.len(), 32);
    }
}
