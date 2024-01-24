use clap::{Parser, ArgAction};
use std::error::Error;
mod task;

/// Default database path
const DB_PATH: &str = "./data/db.json";

#[derive(Parser,Default,Debug)]
#[clap(name="ZTask", author="Tom Zakrajsek", version, about)]
/// A very simple Task Manager
struct Arguments {
    /// Database file of tasks
    #[clap(long, default_value = DB_PATH)]
    db: String,

    /// Add one or more new tasks
    #[clap(short, long, num_args(0..), action=ArgAction::Append)]
    add: Option<Vec<String>>,

    /// Delete one or more tasks
    #[clap(short, long, num_args(0..), action=ArgAction::Append)]
    del: Option<Vec<String>>,

    /// List all tasks
    #[clap(short, long, action=ArgAction::SetTrue )]
    list: bool,

    /// Increase logging verbosity
    #[clap(short, long, action=ArgAction::Count)]
    verbose: u8,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args: Arguments = Arguments::parse();

    // Print arguments for debugging
    // println!("{:?}", args);

    let mut task_list = task::TaskList::new(args.db);

    // Check if user requested to add new tasks with --add
    if let Some(new_task_names) = args.add {
        if new_task_names.is_empty() {
            // --add was provided without name(s)
            // Create default task with default name
            let default_task_name = format!("New task #{count}", count=task_list.num_tasks() + 1);
            let new_task = task::Task::new(default_task_name, "quick".to_string());
            task_list.add_task(new_task);
        } else {
            // --add was provided with name(s)
            // Create new tasks with those names
            for name in new_task_names {
                let new_task = task::Task::new(name, "quick".to_string());
                task_list.add_task(new_task);
            }
        }
    }

    // Check if user requested to delete any tasks with --del
    if let Some(task_ids) = args.del {
        if task_ids.is_empty() {
            // --del was provided without id(s)
            // Remove last task
            let final_length = task_list.tasks.len().saturating_sub(1);  // remove last task
            task_list.tasks.truncate(final_length);
        } else {
            // --del was provided with id(s)
            // Remove selected tasks
            for id in task_ids {
                let uuid = uuid::Uuid::parse_str(&id).unwrap();
                task_list.remove_task(uuid);
            }
        }
    }

    // Check if user requested to list tasks
    if args.list {
        task_list.print_list();
    }

    Ok(())
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     /// Test that expected arguments return an appropriate Config struct
//     #[test]
//     fn config_1() {
//         let test_args = vec!["ztask".to_string(), "Who".to_string(), "poem.txt".to_string()];
//         let config = Config::build(&test_args).unwrap();

//         assert_eq!(config.query, "Who".to_string());
//         assert_eq!(config.file_path, "poem.txt".to_string());
//     }

//     /// Test that invalid arguments return an error
//     #[test]
//     #[should_panic]
//     fn invalid_args() {
//         let test_args = vec!["ztask".to_string()];
//         Config::build(&test_args).unwrap();
//     }

//     /// Test that the case-sensitive query is found in the contents
//     #[test]
//     fn case_sensitive() {
//         // contstraint: This test depends on the contents of poem.txt!
//         let test_args = vec!["ztask".to_string(), "Who".to_string(), "poem.txt".to_string()];
//         let config = Config::build(&test_args).unwrap();

//         run(config).unwrap();
//     }

// }