use std::error::Error;
use crate::task;
use crate::tasklist;
use clap::{Parser, Subcommand, ArgAction};
use colored::Colorize;

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

/// Subcommands for the application
#[derive(Subcommand, Debug)]
enum Command {

    /// List existing tasks
    List {
        /// Increase logging verbosity
        #[clap(short, long, action=ArgAction::Count)]
        verbose: u8,
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

    let mut task_list = tasklist::TaskList::new(args.db);

    if let Some(subcmd) = args.command {
        match subcmd {
            Command::List { verbose } => match process_list(&mut task_list, std::cmp::max(args.verbose, verbose)) {
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

/// Print all tasks
pub fn print_task_list(task_list: &tasklist::TaskList, verbosity: u8) {
    for task in task_list.tasks.iter() {
        if verbosity > 0 {
            print_task_multiline(task, true);
        } else {
            print_task_oneline(task, true);
        }
    }
}

pub fn print_task_oneline(task: &task::Task, colorized: bool) {
    let id = &task.id[0..9];
    // let created = self.created_at.format("%Y-%m-%d %H:%M").to_string();

    let summary = task.summary.to_string();
    let status = task.status.to_string();
    // let details = task.details.to_string();
    let blocked = if task.blocked_by.is_empty() {
        "".to_string()
    } else {
        task
            .blocked_by
            .iter()
            .map(|s| &s[..9])
            .collect::<Vec<_>>()
            .join(", ")
    };

    if colorized {
        let id = id.bright_green();
        let summary = summary.bright_white();
        let status = status.bright_black();
        let blocked = blocked.bright_red();
        // let details = details.bright_black();
        println!("{}  {}  {}  {}", id, summary, status, blocked);
    } else {
        println!("{}  {}  {}  {}", id, summary, status, blocked);
    }
}

pub fn print_task_multiline(task: &task::Task, colorized: bool) {
    let id = &task.id[0..9];
    // let created = self.created_at.format("%Y-%m-%d %H:%M").to_string();

    let summary = task.summary.to_string();
    let status = task.status.to_string();
    let details = task.details.to_string();
    let blocked = if task.blocked_by.is_empty() {
        "".to_string()
    } else {
        task
            .blocked_by
            .iter()
            .map(|s| &s[..9])
            .collect::<Vec<_>>()
            .join(", ")
    };

    if colorized {
        let id = id.bright_green();
        let summary = summary.bright_white();
        let status = status.bright_black();
        let blocked = blocked.bright_red();
        let details = details.bright_black();
        println!("——————————————————————————————————————————————————————");
        println!("summary: {}", summary);
        println!("id: {}", id);
        println!("status: {}", status);
        println!("blocked by: {}", blocked);
        println!("details: {}", details);
    } else {
        println!("——————————————————————————————————————————————————————");
        println!("summary: {}", summary);
        println!("id: {}", id);
        println!("status: {}", status);
        println!("blocked by: {}", blocked);
        println!("details: {}", details);
    }
}


fn process_list(task_list: &mut tasklist::TaskList, verbosity: u8) -> Result<usize, Box<dyn Error>> {
    print_task_list(task_list, verbosity);
    Ok(task_list.tasks.len())
}

fn process_edit(task_list: &mut tasklist::TaskList, task_ids: Vec<String>) -> Result<usize, Box<dyn Error>> {
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

fn process_del(task_list: &mut tasklist::TaskList, task_ids: Vec<String>) -> Result<usize, Box<dyn Error>> {
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

fn process_add(task_list: &mut tasklist::TaskList, new_task_names: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
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
    use tasklist::tests::__create_temp_db;
    use tasklist::tests::__destroy_temp_db;

    #[test]
    #[should_panic]
    fn test_invalid_args() {
        if let Err(_err) = Arguments::try_parse_from(["ztask", "--undefined-flag-guaranteed"]) {
            panic!("--undefined-flag-guaranteed failed as expected");
        }
    }

    // Tests for "list""

    #[test]
    fn verify_command_list() {
        let db = __create_temp_db(5);

        let args: Arguments = Arguments::parse_from(
            ["ztask", "--db", &db, "list"]
        );
        println!("args: {:?}", args);
        run(Some(args)).unwrap();

        __destroy_temp_db(db);
    }

    // Tests for "add"

    #[test]
    fn verify_add_default() {
        let db = __create_temp_db(0);

        let args: Arguments = Arguments::parse_from(
            ["ztask", "--db", &db, "-v", "add"]
        );
        println!("args: {:?}", args);
        run(Some(args)).unwrap();

        __destroy_temp_db(db);
    }

    #[test]
    fn verify_add_single() {
        let db = __create_temp_db(0);

        let args: Arguments = Arguments::parse_from(
            ["ztask", "--db", &db, "-v", "add", "test task"]
        );
        // Should create 1 task with name "test task"
        println!("args: {:?}", args);
        run(Some(args)).unwrap();

        __destroy_temp_db(db);
    }

    #[test]
    fn verify_add_multiple() {
        let db = __create_temp_db(0);

        let args: Arguments = Arguments::parse_from(
            ["ztask", "--db", &db, "-v", "add", "test task #1", "test task #2", "task3", "task4"]
        );
        // Should create 4 tasks with names "test task #1", "test task #2", "task3", "task4"
        println!("args: {:?}", args);
        run(Some(args)).unwrap();

        __destroy_temp_db(db);
    }

    #[test]
    fn verify_add_with_word_merge() {
        let db = __create_temp_db(0);
        let args: Arguments = Arguments::parse_from(
            ["ztask", "--db", &db, "-v", "add", "create", "single", "task"]
        );
        // Should create 1 task with name "create single task"
        println!("args: {:?}", args);
        run(Some(args)).unwrap();
        __destroy_temp_db(db);
    }

    // Tests for "del"

    #[test]
    fn verify_delete_default() {
        let db = __create_temp_db(0);
        let args: Arguments = Arguments::parse_from(
            ["ztask", "--db", &db, "-v", "del"]
        );
        println!("args: {:?}", args);
        run(Some(args)).unwrap();
        __destroy_temp_db(db);
    }

    #[test]
    fn verify_delete_single() {
        let db = __create_temp_db(2);
        let task_list = tasklist::TaskList::new(db.clone());
        let id = task_list.tasks.get(1).unwrap().id.clone();
        let args: Arguments = Arguments::parse_from(
            ["ztask", "--db", &db, "-v", "del", &id]
        );
        drop(task_list);
        println!("args: {:?}", args);
        run(Some(args)).unwrap();
        __destroy_temp_db(db);
    }

    #[test]
    fn verify_delete_nonexisting() {
        let db = __create_temp_db(0);
        let id = "invalid";
        let args: Arguments = Arguments::parse_from(
            ["ztask", "--db", &db, "-v", "del", &id]
        );
        println!("args: {:?}", args);
        run(Some(args)).unwrap();
        __destroy_temp_db(db);
    }

    // Tests for "edit"

    #[test]
    fn verify_edit_default() {
        let db = __create_temp_db(0);
        let args: Arguments = Arguments::parse_from(
            ["ztask", "--db", &db, "-v", "edit"]
        );
        println!("args: {:?}", args);
        run(Some(args)).unwrap();
        __destroy_temp_db(db);
    }

    #[test]
    fn verify_edit_single() {
        let db = __create_temp_db(2);
        let task_list = tasklist::TaskList::new(db.clone());
        let id = task_list.tasks.get(1).unwrap().id.clone();
        let args: Arguments = Arguments::parse_from(
            ["ztask", "--db", &db, "-v", "edit", &id]
        );
        drop(task_list);
        println!("args: {:?}", args);
        run(Some(args)).unwrap();
        __destroy_temp_db(db);
    }
}