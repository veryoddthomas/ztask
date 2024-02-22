use std::error::Error;
use crate::task::{Task, TaskStatus};
use crate::tasklist;
use clap::{Parser, Subcommand, ArgAction};
use colored::{ColoredString, Colorize};

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

    #[clap(short='?', action=ArgAction::Help, help="Print help (alias for --help)")]
    help_short: Option<bool>,
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

        /// Indicate that the task(s) should be added as active (interrupt(s))
        #[clap(short, long, action=ArgAction::SetTrue)]
        is_interrupt: bool,
    },
    /// Start work on a task
    Start {
        /// Id(s) of task(s) to delete
        #[clap(num_args(0..), action=ArgAction::Append)]
        task_ids: Option<Vec<String>>,
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
    /// Block a task on one or more other tasks
    Block {
        /// Id(s) of task(s) to delete
        #[clap(num_args(2..), action=ArgAction::Append)]
        task_ids: Option<Vec<String>>,
    },
    /// Complete one or more tasks
    Complete {
        /// Id(s) of task(s) to delete
        #[clap(num_args(0..), action=ArgAction::Append)]
        task_ids: Option<Vec<String>>,
    },
}

pub fn run(arg_overrides:Option<Arguments>) -> Result<(), Box<dyn Error>> {
    let args = arg_overrides.unwrap_or(Arguments::parse());
    let mut task_list = tasklist::TaskList::new(args.db);

    if let Some(subcmd) = args.command {
        match subcmd {
            Command::List { verbose } => match process_list(&mut task_list, std::cmp::max(args.verbose, verbose), true) {
                Ok(c) => if args.verbose > 0 {
                    println!("{} task(s) found", c)
                },
                Err(e) => eprintln!("error in processing : {}", e),
            },
            Command::Add { task_names, is_interrupt } =>
                match process_add(&mut task_list, task_names.unwrap_or_default(), is_interrupt) {
                    Ok(ids) => if args.verbose > 0 {
                        println!("created task(s) {:?}", ids)
                    },
                    Err(e) => eprintln!("error in processing : {}", e),
            },
            Command::Start { task_ids } => match process_start(&mut task_list, task_ids.unwrap_or_default()) {
                Ok(c) => if args.verbose > 0 {
                    println!("{} task started", c)
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
            Command::Block { task_ids } => match process_block_on(&mut task_list, task_ids.unwrap_or_default()) {
                Ok(c) => if args.verbose > 0 {
                    println!("{} task(s) updated", c)
                },
                Err(e) => eprintln!("error in processing : {}", e),
            },
            Command::Complete { task_ids } => match process_complete(&mut task_list, task_ids.unwrap_or_default()) {
                Ok(c) => if args.verbose > 0 {
                    println!("{} task(s) updated", c)
                },
                Err(e) => eprintln!("error in processing : {}", e),
            },
        }
    } else {
        // No subcommand, so just list the active task
        match process_list(&mut task_list, args.verbose, false) {
            Ok(_) => (),
            Err(e) => eprintln!("error in processing : {}", e),
        }
    }

    Ok(())
}

fn process_list(task_list: &mut tasklist::TaskList, verbosity: u8, show_all: bool) -> Result<usize, Box<dyn Error>> {
    if show_all {
        print_categorized_task_list(task_list, verbosity);
    } else {
        let mut tasks = task_list.tasks.clone();
        tasks.retain(|task| task.status == TaskStatus::Active);
        let mut tasks = tasks.into_sorted_vec();
        let task = tasks.remove(0);

        if verbosity > 0 {
            print_task_detailed(&task);
        } else {
            print_task_oneline(&task, true);
        }
    }
    Ok(task_list.tasks.len())
}

/// Print all tasks
fn print_categorized_task_list(task_list: &tasklist::TaskList, verbosity: u8) {
    show_list("Active Tasks", TaskStatus::Active, task_list, verbosity);
    show_list("Backlog Tasks", TaskStatus::Backlog, task_list, verbosity);
    show_list("Blocked Tasks", TaskStatus::Blocked, task_list, verbosity);
    show_list("Sleeping Tasks", TaskStatus::Sleeping, task_list, verbosity);
    show_list("Completed Tasks", TaskStatus::Completed, task_list, verbosity);

    fn show_list(heading: &str, status: TaskStatus, task_list: &tasklist::TaskList, _verbosity: u8) {
        let mut tasks = task_list.tasks.clone();
        tasks.retain(|task| task.status == status);
        let mut tasks = tasks.into_sorted_vec();

        if !tasks.is_empty() {
            println!("{}:", heading.bright_white());

            if status == TaskStatus::Active {
                // Print the first active task normally
                let task = tasks.remove(0);
                print_task_oneline(&task, false);
            }
        }
        let fn_format = match status {
            TaskStatus::Active => |s: &str| s.bright_black(),
            TaskStatus::Backlog => |s: &str| s.white(),
            TaskStatus::Blocked => |s: &str| s.bright_black(),
            TaskStatus::Sleeping => |s: &str| s.bright_black(),
            TaskStatus::Completed => |s: &str| s.bright_black().strikethrough(),
        };

        if !tasks.is_empty() {
            for task in tasks {
                print_task_oneline_with_format_override(
                    &task, fn_format);
            }
        }
    }
}

// fn red(s: &str) -> ColoredString { s.red() }

fn print_task_oneline_with_format_override(task: &Task, set_color: fn(&str) -> ColoredString) {
    let id = set_color(&task.id[..9]);
    let priority = set_color(&task.priority.to_string());

    print!("  {}  {}", id, priority);
    print!("  {}", set_color(&task.created_at.format("%F").to_string()));

    let summary = set_color(&task.summary.to_string());
    let blocked = if task.blocked_by.is_empty() {
        set_color("")  // bright_red()
    } else {
        set_color(&format!("[{}]",
            task.blocked_by
                    .iter()
                    .map(|s| &s[..9])
                    .collect::<Vec<_>>()
                    .join(", ")))
    };

    print!("  {}  {}", summary, blocked);
    println!();
}

fn print_task_oneline(task: &Task, show_status: bool) {
    let show_date = true;
    // See specifiers at https://docs.rs/chrono/latest/chrono/format/strftime/index.html
    // "%F@%T%.3f" example: 2024-02-15@22:38:39.439

    let id = &task.id[..9];
    let id = match task.status {
        TaskStatus::Active => id.bright_green(),
        TaskStatus::Backlog => id.white(),
        TaskStatus::Blocked => id.truecolor(238,105,105),  // bright_red(),
        TaskStatus::Sleeping => id.bright_black(),
        TaskStatus::Completed => id.bright_black(),
    };
    let priority = task.priority.to_string().bright_black();

    print!("  {}", id);
    print!("  {}", priority);
    if show_status { print!("  {}", task.status.to_string().bright_black()); }
    if show_date { print!("  {}", task.created_at.format("%F").to_string().bright_black()); }

    // let summary = task.summary.to_string().bright_black();
    let blocked = if task.blocked_by.is_empty() {
        "".truecolor(238,105,105)  // bright_red()
    } else {
        format!("[{}]",
            task.blocked_by
                .iter()
                .map(|s| &s[..9])
                .collect::<Vec<_>>()
                .join(", ")).truecolor(238,105,105)  // bright_red()
    };

    print!("  {}  {}", task.summary.to_string().white(), blocked);
    println!();
}

pub fn print_task_detailed(task: &Task) {
    let blocked = if task.blocked_by.is_empty() {
        "".to_string().truecolor(26, 126, 165)
    } else {
        task
            .blocked_by
            .iter()
            .map(|s| &s[..9])
            .collect::<Vec<_>>()
            .join(", ").truecolor(26, 126, 165)
    };

    let width = 11;
    println!("  {:width$} {}", "summary:", task.summary.to_string().truecolor(40, 194, 254));
    println!("  {:width$} {}", "id:", &task.id[0..9].to_string().truecolor(0x1a, 0x7e, 0xa5));
    println!("  {:width$} {}", "priority:", task.priority.to_string().truecolor(0x1a, 0x7e, 0xa5));
    println!("  {:width$} {}", "status:", task.status.to_string().truecolor(0x1a, 0x7e, 0xa5));
    println!("  {:width$} {}", "created:", task.created_at.format("%F %T").to_string().truecolor(0x1a, 0x7e, 0xa5));
    if task.status == TaskStatus::Blocked {
        println!("  {:width$} {}", "blocked by:".bright_white(), blocked);
    }
    if !task.details.is_empty() {
        println!("  {}", "details:".bright_white());
        println!("  {}", task.details.to_string().truecolor(0x1a, 0x7e, 0xa5));
    }
}


fn process_block_on(task_list: &mut tasklist::TaskList, task_ids: Vec<String>) -> Result<usize, Box<dyn Error>> {
    let mut blocker_count = 0;
    if task_ids.is_empty() {
        // TODO: Should this prompt for which to block on?
        println!("block_on arg list is empty, which is not currently allowed");
    } else {
        // Edit selected tasks
        let blockee = task_ids.first().unwrap();
        let mut task_ids = task_ids.clone();
        task_ids.remove(0);
        for id in task_ids.clone() {
            blocker_count += task_list.block_task_on(blockee, &id);
        }
    }
    Ok(blocker_count)
}

fn process_complete(task_list: &mut tasklist::TaskList, task_ids: Vec<String>) -> Result<usize, Box<dyn Error>> {
    let mut completed_count = 0;
    if task_ids.is_empty() {
        let mut tasks = task_list.tasks.clone();
        tasks.retain(|task| task.status == TaskStatus::Active);
        let mut tasks = tasks.into_sorted_vec();
        let task = tasks.remove(0);
        task_list.complete_task(task.id);
        completed_count = 1;
    } else {
        // Complete selected tasks
        for id in task_ids {
            completed_count += task_list.complete_task(id);
        }
    }
    Ok(completed_count)
}

fn process_start(task_list: &mut tasklist::TaskList, task_ids: Vec<String>) -> Result<usize, Box<dyn Error>> {
    let mut completed_count = 0;
    if task_ids.is_empty() {
        let count_active = task_list.tasks.iter().filter(|task| task.status == TaskStatus::Active).count();

        if count_active == 0 {
            let mut tasks = task_list.tasks.clone();
            tasks.retain(|task| task.status == TaskStatus::Backlog);
            let mut tasks = tasks.into_sorted_vec();
            let task = tasks.remove(0);
            task_list.start_task(task.id);
            completed_count = 1;
        } else {
            println!("Can't activate default backlog task when there are active tasks");
            println!("Clear your active tasks or use the start command with a task id");
        }
    } else {
        task_list.start_task(task_ids.first().unwrap().clone());
        completed_count = 1;
    }
    Ok(completed_count)
}

fn process_edit(task_list: &mut tasklist::TaskList, task_ids: Vec<String>) -> Result<usize, Box<dyn Error>> {
    let mut edit_count = 0;
    if task_ids.is_empty() {
        let mut tasks = task_list.tasks.clone();
        tasks.retain(|task| task.status == TaskStatus::Active);
        let mut tasks = tasks.into_sorted_vec();
        let task = tasks.remove(0);
        task_list.edit_task(task.id);
        edit_count = 1;
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
        task_list.tasks.pop();
    } else {
        // Remove selected tasks
        for id in task_ids {
            task_list.remove_task(id);
        }
    }
    Ok(prior_task_count - task_list.tasks.len())
}

fn process_add(task_list: &mut tasklist::TaskList, new_task_names: Vec<String>, is_interrupt: bool) -> Result<Vec<String>, Box<dyn Error>> {
    let mut created_task_ids: Vec<String> = Vec::new();
    if new_task_names.is_empty() {
        // Create default task with default name
        let default_task_name = format!("New task #{count}", count=task_list.num_tasks() + 1);
        let new_task = Task::new(default_task_name, "quick".to_string(), is_interrupt);
        created_task_ids.push(new_task.id.clone());
        print_task_oneline(&new_task, true);
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
                let new_task = Task::new(name, "quick".to_string(), is_interrupt);
                created_task_ids.push(new_task.id.clone());
                print_task_oneline(&new_task, true);
                task_list.add_task(new_task);
            } else {
                // Some task names are multi-word
                // Create multiple tasks with those task names
                for name in new_task_names {
                    let new_task = Task::new(name, "quick".to_string(), is_interrupt);
                    created_task_ids.push(new_task.id.clone());
                    print_task_oneline(&new_task, true);
                    task_list.add_task(new_task);
                }
            }
        } else {
            // Create single task with that task name
            let new_task = Task::new(new_task_names[0].clone(), "quick".to_string(), is_interrupt);
            created_task_ids.push(new_task.id.clone());
            print_task_oneline(&new_task, true);
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
        let mut iter = task_list.tasks.iter().skip(1);
        let id = iter.next().unwrap().id.clone();
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
        let mut iter = task_list.tasks.iter().skip(1);
        let id = iter.next().unwrap().id.clone();
        let args: Arguments = Arguments::parse_from(
            ["ztask", "--db", &db, "-v", "edit", &id]
        );
        drop(task_list);
        println!("args: {:?}", args);
        run(Some(args)).unwrap();
        __destroy_temp_db(db);
    }
}
