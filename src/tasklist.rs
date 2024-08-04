use crate::simple_duration;
use crate::task::{Status, Task};
use chrono::Local;
use std::collections::{BTreeSet, BinaryHeap};
use std::fs;
use std::fs::File;
use std::io::{self, Write};

/// Task list data structure, includeing a priority queue of tasks
/// and a database path.
pub struct TaskList {
    // pub active_tasks: VecDeque<Task>,
    // pub backlog_tasks: VecDeque<Task>,
    // pub blocked_tasks: VecDeque<Task>,
    // pub completed_tasks: VecDeque<Task>,
    // pub sleeping_tasks: VecDeque<Task>,
    pub tasks: BinaryHeap<Task>,
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
            Ok(tasks) => {
                let mut task_list = TaskList { tasks, db_path };
                let awakened = task_list.wake_tasks();
                if awakened > 0 {
                    println!("Awakened {awakened} task(s)");
                }
                let unblocked = task_list.unblock_tasks();
                if unblocked > 0 {
                    println!("Unblocked {unblocked} task(s)");
                }
                task_list
            }
            Err(_) => TaskList {
                tasks: BinaryHeap::new(),
                db_path,
            },
        }
    }

    /// Save the task list to the database file.
    pub fn save(&self) -> Result<(), io::Error> {
        let serialized = serde_json::to_string_pretty(&self.tasks)?;
        let mut file = File::create(&self.db_path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    /// Load the task list from the database file.
    pub fn load(db_path: String) -> Result<BinaryHeap<Task>, io::Error> {
        let contents = fs::read_to_string(db_path)?;
        let tasks: BinaryHeap<Task> = serde_json::from_str(&contents)?;
        Ok(tasks)
    }

    /// Return the number of tasks in the list.
    pub fn num_tasks(&self) -> usize {
        self.tasks.len()
    }

    /// Wake any tasks whose snooze timer has expired
    pub fn wake_tasks(&mut self) -> usize {
        let mut num_woken = 0;
        let now = Local::now();

        // let updated_tasks = self.tasks.clone().into_sorted_vec();
        let mut updated_tasks: BinaryHeap<Task> = BinaryHeap::new();

        // Process every node in the BinaryHeap
        while let Some(mut task) = self.tasks.pop() {
            if task.status == Status::Sleeping && task.wake_at.unwrap() <= now {
                task.status = Status::Backlog;
                task.wake_at = None;
                num_woken += 1;
            }
            updated_tasks.push(task);
        }
        self.tasks = updated_tasks;
        num_woken
    }

    /// Check for tasks that are blocked on other tasks that have been completed
    /// or deleted and unblock them.
    /// Returns the number of tasks unblocked.
    pub fn unblock_tasks(&mut self) -> usize {
        let mut num_unblocked = 0;

        let blocking_capable_ids: BTreeSet<String> = self
            .tasks
            .iter()
            .filter(|task| task.status != Status::Completed)
            .map(|task| task.id.clone())
            .collect();

        // let updated_tasks = self.tasks.clone().into_sorted_vec();
        let mut updated_tasks: BinaryHeap<Task> = BinaryHeap::new();

        // Process every node in the BinaryHeap
        while let Some(mut task) = self.tasks.pop() {
            if task.status == Status::Blocked {
                let intersection: BTreeSet<_> = task
                    .blocked_by
                    .intersection(&blocking_capable_ids)
                    .cloned()
                    .collect();
                task.blocked_by = intersection;
                if task.blocked_by.is_empty() {
                    task.status = Status::Backlog;
                    num_unblocked += 1;
                }
            }
            updated_tasks.push(task);
        }
        self.tasks = updated_tasks;
        num_unblocked
    }

    /// Clone a task
    pub fn copy_task(&mut self, id: &str) -> Option<Task> {
        let tasks = self.tasks.iter().filter(|task| task.id[0..id.len()] == *id);
        let match_count = tasks.count();
        if match_count != 1 {
            println!("Id '{id}' does not uniquely match one task.  It matches {match_count}");
            return None;
        }

        // There will be only one match, so unwrap is safe
        let task = self
            .tasks
            .iter()
            .find(|task| task.id[0..id.len()] == *id)
            .unwrap();

        Some(task.clone())
    }

    /// Add a task to the list.
    pub fn add_task(&mut self, task: Task) -> String {
        let id = task.id.clone();
        self.tasks.push(task);
        id
    }

    /// Remove the task whose id starts with the id string passed in.
    pub fn remove_task(&mut self, id: &str) {
        // If we don't find exactly one task that starts with 'id',
        // print a warning and return
        let match_count = self
            .tasks
            .iter()
            .filter(|task| task.id[0..id.len()] == *id)
            .count();
        if match_count != 1 {
            println!("Id '{id}' does not uniquely match one task.  It matches {match_count}");
            return;
        }
        self.tasks.retain(|task| task.id[0..id.len()] != *id);
    }

    /// Block the blockee on the blocker(s)
    #[allow(clippy::similar_names)]
    pub fn block_task_on(&mut self, blockee_id: &String, blocker_id: &String) -> usize {
        // If we don't find exactly one task that starts with 'id',
        // print a warning and return
        let blockee_match_count = self
            .tasks
            .iter()
            .filter(|task| &task.id[0..blockee_id.len()] == blockee_id)
            .count();
        if blockee_match_count != 1 {
            println!(
                "Blockee Id '{blockee_id}' does not uniquely match one task.  It matches {blockee_match_count}");
            return 0;
        }
        let blocker_match_count = self
            .tasks
            .iter()
            .filter(|task| &task.id[0..blocker_id.len()] == blocker_id)
            .count();
        if blocker_match_count != 1 {
            println!(
                "Blocker Id '{blocker_id}' does not uniquely match one task.  It matches {blocker_match_count}");
            return 0;
        }
        // There will be only one match, so unwrap is safe
        let blockee = self
            .tasks
            .iter()
            .find(|task| &task.id[0..blockee_id.len()] == blockee_id)
            .unwrap();
        let blocker = self
            .tasks
            .iter()
            .find(|task| &task.id[0..blocker_id.len()] == blocker_id)
            .unwrap();

        let mut updated_task = blockee.clone();
        updated_task.block_on(blocker.id.clone());
        // updated_task.invoke_editor().unwrap_or_default();  // TODO: Handle errors
        let id = blockee.id.clone();
        self.tasks.retain(|task| task.id != id);
        self.tasks.push(updated_task);

        1
    }

    /// Edit the task whose id starts with the id string passed in.
    pub fn edit_task(&mut self, id: &str) -> usize {
        let tasks = self.tasks.iter().filter(|task| task.id[0..id.len()] == *id);
        let match_count = tasks.count();
        if match_count != 1 {
            println!("Id '{id}' does not uniquely match one task.  It matches {match_count}");
            return 0;
        }

        // There will be only one match, so unwrap is safe
        let task = self
            .tasks
            .iter()
            .find(|task| task.id[0..id.len()] == *id)
            .unwrap();
        let mut updated_task = task.clone();
        updated_task.invoke_editor().unwrap_or_default(); // TODO: Handle errors
        let id = task.id.clone();
        self.tasks.retain(|task| task.id != id);
        self.tasks.push(updated_task);
        1
    }

    /// Edit the details for the task whose id starts with the id string passed in.
    pub fn edit_task_details(&mut self, id: &str) -> usize {
        let tasks = self.tasks.iter().filter(|task| task.id[0..id.len()] == *id);
        let match_count = tasks.count();
        if match_count != 1 {
            println!("Id '{id}' does not uniquely match one task.  It matches {match_count}");
            return 0;
        }

        // There will be only one match, so unwrap is safe
        let task = self
            .tasks
            .iter()
            .find(|task| task.id[0..id.len()] == *id)
            .unwrap();
        let mut updated_task = task.clone();
        updated_task.invoke_editor_for_details().unwrap_or_default(); // TODO: Handle errors
        let id = task.id.clone();
        self.tasks.retain(|task| task.id != id);
        self.tasks.push(updated_task);
        1
    }

    /// Complete the task whose id starts with the id string passed in.
    pub fn complete_task(&mut self, id: &str) -> usize {
        let tasks = self.tasks.iter().filter(|task| task.id[0..id.len()] == *id);
        let match_count = tasks.count();
        if match_count != 1 {
            println!("Id '{id}' does not uniquely match one task.  It matches {match_count}");
            return 0;
        }

        // There will be only one match, so unwrap is safe
        let task = self
            .tasks
            .iter()
            .find(|task| task.id[0..id.len()] == *id)
            .unwrap();
        let mut updated_task = task.clone();
        updated_task.status = Status::Completed;
        let id = task.id.clone();
        self.tasks.retain(|task| task.id != id);
        self.tasks.push(updated_task);
        1
    }

    /// Start the task whose id starts with the id string passed in.
    pub fn start_task(&mut self, id: &str) -> usize {
        let tasks = self.tasks.iter().filter(|task| task.id[0..id.len()] == *id);
        let match_count = tasks.count();
        if match_count != 1 {
            println!("Id '{id}' does not uniquely match one task.  It matches {match_count}");
            return 0;
        }

        // There will be only one match, so unwrap is safe
        let task = self
            .tasks
            .iter()
            .find(|task| task.id[0..id.len()] == *id)
            .unwrap();
        let mut updated_task = task.clone();
        updated_task.status = Status::Active;
        let id = task.id.clone();
        self.tasks.retain(|task| task.id != id);
        self.tasks.push(updated_task);
        1
    }

    /// Suspend the task whose id starts with the id string passed in.
    pub fn suspend_task(&mut self, id: &str, duration: &str) -> usize {
        let tasks = self.tasks.iter().filter(|task| task.id[0..id.len()] == *id);
        let match_count = tasks.count();
        if match_count != 1 {
            println!("Id '{id}' does not uniquely match one task.  It matches {match_count}");
            return 0;
        }

        // There will be only one match, so unwrap is safe
        let task = self
            .tasks
            .iter()
            .find(|task| task.id[0..id.len()] == *id)
            .unwrap();
        let mut updated_task = task.clone();
        updated_task.status = Status::Sleeping;
        let time_delta = simple_duration::parse(duration).unwrap();
        println!("Sleeping for {} seconds", time_delta.num_seconds());
        updated_task.wake_at = Some(Local::now() + time_delta);
        let id = task.id.clone();
        self.tasks.retain(|task| task.id != id);
        self.tasks.push(updated_task);
        1
    }
}

// xref: /usr/local/develop/rust-commandline-example/src/main.rs

#[cfg(test)]
pub mod tests {
    use super::*;
    use uuid::Uuid;

    /// Create a temporary test database with the given number of tasks
    pub fn __create_temp_db(initial_task_count: i32) -> String {
        use std::path::Path;
        fs::create_dir_all("data/temp").unwrap();
        let db = format!("data/temp/{}-test.json", Uuid::new_v4().simple());
        assert!(
            !Path::new(&db).exists(),
            "Temporary test database already exists: {db}"
        );

        let mut task_list = TaskList::new(db.clone());
        for i in 0..initial_task_count {
            task_list.add_task(Task::new(
                format!("test task {i}").to_string(),
                "quick".to_string(),
                true,
            ));
        }
        db
    }

    /// Remove the named test database
    pub fn __destroy_temp_db(test_db: &str) -> String {
        use std::path::Path;

        if test_db.starts_with("data/temp")
            && test_db.ends_with("-test.json")
            && Path::new(test_db).exists()
        {
            let _ = fs::remove_file(test_db);
        }
        test_db.to_string()
    }

    #[test]
    fn verify_remove_single() {
        let db = __create_temp_db(2);
        let mut task_list = TaskList::new(db.clone());

        let mut iter = task_list.tasks.iter().skip(1);
        let id = iter.next().unwrap().id.clone();

        task_list.remove_task(&id);
        assert_eq!(task_list.tasks.len(), 1);

        drop(task_list);
        __destroy_temp_db(&db);
    }

    #[test]
    fn verify_edit_single() {
        let db = __create_temp_db(2);
        let mut task_list = TaskList::new(db.clone());

        let mut iter = task_list.tasks.iter().skip(1);
        let id = iter.next().unwrap().id.clone();

        task_list.edit_task(&id);
        assert_eq!(task_list.tasks.len(), 2);

        drop(task_list);
        __destroy_temp_db(&db);
    }
}
