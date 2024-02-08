use chrono::{DateTime, Local};
use std::fs::File;
use std::io::Write;
use std::fs;
use std::io;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use colored::Colorize;
use std::collections::VecDeque;
// use std::fmt;


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
pub enum TaskStatus {
    #[serde(rename = "active")]    Active,
    #[serde(rename = "backlog")]   Backlog,
    #[serde(rename = "blocked")]   Blocked,
    #[serde(rename = "completed")] Completed,
    #[serde(rename = "sleeping")]  Sleeping,  // Would Snoozed be better?
}

impl TaskStatus {
    fn label(&self) -> String {
        let label = match self {
            TaskStatus::Active => "active",
            TaskStatus::Backlog => "backlog",
            TaskStatus::Blocked => "blocked",
            TaskStatus::Completed => "completed",
            TaskStatus::Sleeping => "sleeping",
        };
        label.to_string()
    }
}

/// Task structure
#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub summary: String,
    pub category: String,
    pub created_at: DateTime<Local>,
    pub status: TaskStatus,
}

impl Task {
    pub fn new(summary: String, category: String) -> Self {
        Task {
            id: Uuid::new_v4().simple().to_string(),
            summary,
            category,
            created_at: Local::now(),
            status: TaskStatus::Active,
        }
    }
    pub fn to_string(&self, colorized: bool) -> String {
        let id=&self.id[0..9];
        // let created = self.created_at.format("%Y-%m-%d %H:%M").to_string();
        if colorized {
            format!("{}  {}  {}", id.bright_green(), self.summary, self.status.label().bright_white())
        } else {
            format!("{}  {}  {}", id, self.summary, self.status.label())
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
        let match_count = self.tasks.iter().filter(|task| task.id[0..id.len()] == id).count();
        if match_count !=1 {
            println!("Id '{}' does not uniquely match one task.  It matches {}", id, match_count);
            return;
        }
        self.tasks.retain(|task| task.id[0..id.len()] != id)
    }

    /// Edit the task whose id starts with the id string passed in.
    pub fn edit_task(&mut self, id: String) {
        let tasks = self.tasks.iter().filter(|task| task.id[0..id.len()] == id);
        let match_count = tasks.count();
        if match_count !=1 {
            println!("Id '{}' does not uniquely match one task.  It matches {}", id, match_count);
            return;
        }

        let result = self.tasks.iter().find(|task| task.id[0..id.len()] == id);
        if let Some(task) = result {
            println!("TBD: implement edit for {}", task.to_string(true));
        } else {
            println!("Task {id} not found")
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
mod tests {
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
