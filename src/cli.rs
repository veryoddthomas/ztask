use std::error::Error;
use crate::task;
use clap::{Parser, Subcommand, ArgAction};

/// Default database path
const DB_PATH: &str = "./data/db.json";

#[derive(Parser,Default,Debug)]
#[clap(name="ZTask", author="Tom Zakrajsek", version, about)]
/// A very simple Task Manager
pub struct Arguments {

    #[command(subcommand)]
    command: Option<Command>,

    /// Database file of tasks
    #[clap(long, default_value = DB_PATH)]
    db: String,

    /// Increase logging verbosity
    #[clap(short, long, action=ArgAction::Count)]
    verbose: u8,
}

#[derive(Subcommand, Debug)]
enum Command {

    /// List existing tasks
    List {
    },

    /// Add one or more new tasks
    Add {
        /// Name of task(s) to add
        #[clap(num_args(0..), action=ArgAction::Append)]
        task_names: Option<Vec<String>>,
    },

    /// Del one or more tasks
    Del {
        /// Id(s) of task(s) to delete
        #[clap(num_args(0..), action=ArgAction::Append)]
        task_ids: Option<Vec<String>>,
    },

    /// Edit one or more tasks
    Edit {
        /// Id(s) of task(s) to edit
        #[clap(num_args(0..), action=ArgAction::Append)]
        task_ids: Option<Vec<String>>,
    },
}

pub fn run(arg_overrides:Option<Arguments>) -> Result<(), Box<dyn Error>> {
    let args = arg_overrides.unwrap_or(Arguments::parse());

    // // Print arguments for debugging
    // println!("{:?}", args);

    let mut task_list = task::TaskList::new(args.db);

    if let Some(subcmd) = args.command {
        match subcmd {
            Command::List { } => match process_list(&mut task_list) {
                Ok(c) => if args.verbose > 0 {
                    println!("{} task(s) found", c)
                },
                Err(e) => eprintln!("error in processing : {}", e),
            },
            Command::Add { task_names } => match process_add(&mut task_list, task_names.unwrap_or_default()) {
                Ok(ids) => if args.verbose > 0 {
                    println!("created task(s) {:?}", ids)
                },
                Err(e) => eprintln!("error in processing : {}", e),
            },
            Command::Del { task_ids } => match process_del(&mut task_list, task_ids.unwrap_or_default()) {
                Ok(c) => if args.verbose > 0 {
                    println!("{} task(s) removed", c)
                },
                Err(e) => eprintln!("error in processing : {}", e),
            },
            Command::Edit { task_ids } => match process_edit(&mut task_list, task_ids.unwrap_or_default()) {
                Ok(c) => if args.verbose > 0 {
                    println!("{} task(s) updated", c)
                },
                Err(e) => eprintln!("error in processing : {}", e),
            },
        }
    }

    Ok(())
}

fn process_list(task_list: &mut task::TaskList) -> Result<usize, Box<dyn Error>> {
    task_list.print_list();
    Ok(task_list.tasks.len())
}

fn process_edit(task_list: &mut task::TaskList, task_ids: Vec<String>) -> Result<usize, Box<dyn Error>> {
    let mut edit_count = 0;
    if task_ids.is_empty() {
        // TODO: Should this edit the most recent task?
        println!("edit arg list is empty, which is not currently allowed");
    } else {
        // Edit selected tasks
        for id in task_ids {
            edit_count += task_list.edit_task(id);
        }
    }
    Ok(edit_count)
}

fn process_del(task_list: &mut task::TaskList, task_ids: Vec<String>) -> Result<usize, Box<dyn Error>> {
    let prior_task_count = task_list.tasks.len();
    if task_ids.is_empty() {
        // Remove last task
        let final_length = task_list.tasks.len().saturating_sub(1);  // remove last task
        task_list.tasks.truncate(final_length);
    } else {
        // Remove selected tasks
        for id in task_ids {
            task_list.remove_task(id);
        }
    }
    Ok(prior_task_count - task_list.tasks.len())
}

fn process_add(task_list: &mut task::TaskList, new_task_names: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
    let mut created_task_ids: Vec<String> = Vec::new();
    if new_task_names.is_empty() {
        // Create default task with default name
        let default_task_name = format!("New task #{count}", count=task_list.num_tasks() + 1);
        let new_task = task::Task::new(default_task_name, "quick".to_string());
        created_task_ids.push(new_task.id.clone());
        task_list.add_task(new_task);
    } else {
        // Create new tasks with provided names
        if new_task_names.len() > 1 {
            // If they are all single word, consider this as a single task
            let count_multi_word = new_task_names.iter().filter(|name| name.contains(' ')).count();
            if count_multi_word == 0 {
                // All task names are single word
                // Create single task with those task names
                let name = new_task_names.join(" ");
                let new_task = task::Task::new(name, "quick".to_string());
                created_task_ids.push(new_task.id.clone());
                task_list.add_task(new_task);
            } else {
                // Some task names are multi-word
                // Create multiple tasks with those task names
                for name in new_task_names {
                    let new_task = task::Task::new(name, "quick".to_string());
                    created_task_ids.push(new_task.id.clone());
                    task_list.add_task(new_task);
                }
            }
        } else {
            // Create single task with that task name
            let new_task = task::Task::new(new_task_names[0].clone(), "quick".to_string());
            created_task_ids.push(new_task.id.clone());
            task_list.add_task(new_task);
        }
    }
    // return number of tasks added
    Ok(created_task_ids)

}

#[cfg(test)]
mod tests {
    use super::*;

    fn _setup_empty_db(override_db: Option<&str>) -> String {
        use std::fs;
        let test_db = override_db.unwrap_or("data/test.json");
        let _ = fs::remove_file(test_db);
        test_db.to_string()
    }

    // Tests for "list""

    #[test]
    fn verify_list() {
        let db = _setup_empty_db(None);
        let args: Arguments = Arguments::parse_from(
            ["ztask", "--db", &db, "-v", "list"]
        );
        println!("args: {:?}", args);
        run(Some(args)).unwrap();
    }

    // Tests for "add"

    #[test]
    fn verify_add_default() {
        let db = _setup_empty_db(None);
        let args: Arguments = Arguments::parse_from(
            ["ztask", "--db", &db, "-v", "add"]
        );
        println!("args: {:?}", args);
        run(Some(args)).unwrap();
    }

    #[test]
    fn verify_add_single() {
        let db = _setup_empty_db(None);
        let args: Arguments = Arguments::parse_from(
            ["ztask", "--db", &db, "-v", "add", "test task"]
        );
        // Should create 1 task with name "test task"
        println!("args: {:?}", args);
        run(Some(args)).unwrap();
    }

    #[test]
    fn verify_add_multiple() {
        let db = _setup_empty_db(None);
        let args: Arguments = Arguments::parse_from(
            ["ztask", "--db", &db, "-v", "add", "test task #1", "test task #2", "task3", "task4"]
        );
        // Should create 4 tasks with names "test task #1", "test task #2", "task3", "task4"
        println!("args: {:?}", args);
        run(Some(args)).unwrap();
    }

    #[test]
    fn verify_add_with_word_merge() {
        let db = _setup_empty_db(None);
        let args: Arguments = Arguments::parse_from(
            ["ztask", "--db", &db, "-v", "add", "create", "single", "task"]
        );
        // Should create 1 task with name "create single task"
        println!("args: {:?}", args);
        run(Some(args)).unwrap();
    }

    // Tests for "del"

    #[test]
    fn verify_delete_default() {
        let db = _setup_empty_db(None);
        let args: Arguments = Arguments::parse_from(
            ["ztask", "--db", &db, "-v", "del"]
        );
        println!("args: {:?}", args);
        run(Some(args)).unwrap();
    }
}
