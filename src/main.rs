use std::process;
use clap::{Parser, Subcommand, ArgAction};
use std::error::Error;

mod task;

/// Default database path
const DB_PATH: &str = "./data/db.json";

#[derive(Parser,Default,Debug)]
#[clap(name="ZTask", author="Tom Zakrajsek", version, about)]
/// A very simple Task Manager
struct Arguments {

    #[command(subcommand)]
    command: Option<SubCommand>,

    /// Database file of tasks
    #[clap(long, default_value = DB_PATH)]
    db: String,

    /// Edit one or more tasks
    #[clap(short, long, num_args(0..), action=ArgAction::Append)]
    edit: Option<Vec<String>>,

    /// Increase logging verbosity
    #[clap(short, long, action=ArgAction::Count)]
    verbose: u8,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// List existing tasks
    List {
        // /// List all tasks
        // /// Increase logging verbosity
        // #[clap(short, long, action=ArgAction::Count)]
        // verbose: u8,
    },
    /// Add one or more new tasks
    Add {
        /// Name of task(s) to add
        #[clap(num_args(0..), action=ArgAction::Append)]
        task_names: Option<Vec<String>>,
    },
    /// Del one or more tasks
    Del {
        /// Name of task(s) to delete
        #[clap(num_args(0..), action=ArgAction::Append)]
        task_ids: Option<Vec<String>>,
    },
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args: Arguments = Arguments::parse();

    // // Print arguments for debugging
    // println!("{:?}", args);

    let mut task_list = task::TaskList::new(args.db);

    // Check if user requested to edit any tasks with --edit
    if let Some(task_ids) = args.edit {
        if task_ids.is_empty() {
            // --edit was provided without id(s)
            println!("--edit arg list is empty, which is not allowed")
        } else {
            // --edit was provided with id(s)
            // Edit selected tasks
            for id in task_ids {
                task_list.edit_task(id);
            }
        }
    }

    if let Some(subcmd) = args.command {
        match subcmd {
            SubCommand::List { } => match process_list(&mut task_list) {
                Ok(c) => if args.verbose > 0 {
                    println!("{} task(s) found", c)
                },
                Err(e) => eprintln!("error in processing : {}", e),
            },
            SubCommand::Add { task_names } => match process_add(&mut task_list, task_names.unwrap_or_default()) {
                Ok(c) => if args.verbose > 0 {
                    println!("{} task(s) added", c)
                },
                Err(e) => eprintln!("error in processing : {}", e),
            },
            SubCommand::Del { task_ids } => match process_del(&mut task_list, task_ids.unwrap_or_default()) {
                Ok(c) => if args.verbose > 0 {
                    println!("{} task(s) removed", c)
                },
                Err(e) => eprintln!("error in processing : {}", e),
            },
        }
    }

    Ok(())
}

fn process_list(task_list: &mut task::TaskList) -> Result<usize, Box<dyn Error>> {
    task_list.print_list();
    Ok(task_list.tasks.len())  // TODO: return number of tasks listed
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

fn process_add(task_list: &mut task::TaskList, new_task_names: Vec<String>) -> Result<usize, Box<dyn Error>> {
    let prior_task_count = task_list.tasks.len();
    if new_task_names.is_empty() {
        // Create default task with default name
        let default_task_name = format!("New task #{count}", count=task_list.num_tasks() + 1);
        let new_task = task::Task::new(default_task_name, "quick".to_string());
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
                task_list.add_task(new_task);
            } else {
                // Some task names are multi-word
                // Create multiple tasks with those task names
                for name in new_task_names {
                    let new_task = task::Task::new(name, "quick".to_string());
                    task_list.add_task(new_task);
                }
            }
        } else {
            // Create single task with that task name
            let new_task = task::Task::new(new_task_names[0].clone(), "quick".to_string());
            task_list.add_task(new_task);
        }
    }
    // return number of tasks added
    Ok(task_list.tasks.len() - prior_task_count)

}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use std::fs;

    #[test]
    fn invoke_main() {
        super::main();
    }

    #[test]
    fn invoke_main_1() {
        let test_db = "data/test.json";
        let _ = fs::remove_file(test_db);

        let mut cmd = Command::cargo_bin("ztask").unwrap();
        let assert = cmd
            .arg("--db").arg(test_db)
            .arg("-v")
            .arg("list")
            // .arg("-a")
            .assert();
        assert
            .success()
            .code(0);
            //.stdout(predicate::str::contains("task count: "));
    }

    // #[test]
    // fn test_main() {
    //     let mut cmd = Command::cargo_bin("ztask").unwrap();
    //     cmd.assert().success();
    // }

    // #[test]
    // fn test_version() {
    //     let mut cmd = Command::cargo_bin("ztask").unwrap();
    //     cmd.arg("--version").assert().success();
    // }

    // #[test]
    // fn test_help() {
    //     let mut cmd = Command::cargo_bin("ztask").unwrap();
    //     cmd.arg("--help").assert().success();
    // }

    // #[test]
    // fn test_main_1() {
    //     let mut cmd = assert_cmd::cargo::cargo_bin("ztask");
    //     println!("{:?}", cmd.as_os_str());
    //     let assert = cmd
    //         .arg("-A")
    //         .env("stdout", "hello")
    //         .env("exit", "42")
    //         .write_stdin("42")
    //         .assert();
    //     assert
    //         .failure()
    //         .code(42)
    //         .stdout("hello\n");

    // }
}
