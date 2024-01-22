use clap::{Parser, ArgAction};
use std::error::Error;
// use std::fs;
// use std::env;
mod task;

const DB_PATH: &str = "./data/db.json";

#[derive(Parser,Default,Debug)]
#[clap(name="ZTask", author="Tom Zakrajsek", version, about)]
/// A very simple Task Manager
struct Arguments {
    #[clap(short, long, default_value = DB_PATH)]
    /// Database file of tasks
    // db: Option<String>,
    db: String,

    #[clap(short, long, action=ArgAction::SetTrue )]
    /// Add a task to the list
    list: bool,

    /// Increase logging verbosity
    #[clap(short, long, action=ArgAction::Count)]
    verbose: u8,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args: Arguments = Arguments::parse();
    println!("{:?}", args);

    let task_list = task::TaskList::new(args.db);

    // let new_task = task::Task::new("New task".to_string(), "".to_string());
    // task_list.add_task(new_task);
    task_list.print_list();

    // match &args.add.unwrap() {
    //     Ok(c) => println!("{} uses found", c),
    //     Err(e) => eprintln!("error in processing : {}", e),
    // }


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