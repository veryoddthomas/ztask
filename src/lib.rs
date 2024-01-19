use std::error::Error;
use std::fs;
use std::env;

mod task;
mod search;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search::search_case_insensitive(&config.query, &contents)
    } else {
        search::search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }


    // Load the database, print it, and then write it back.
    // Note: write back will restrict to known data structure
    // for Task, and remove other fields.
    // let tasks = task::read_db().expect("fetch task list");
    let tasks = task::read_db().expect("fetch task list");
    for t in &tasks {
        println!("{name}", name = t.name);
    }

    task::write_db(&tasks);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    /// Test that expected arguments return an appropriate Config struct
    #[test]
    fn config_1() {
        let test_args = vec!["ztask".to_string(), "Who".to_string(), "poem.txt".to_string()];
        let config = Config::build(&test_args).unwrap();

        assert_eq!(config.query, "Who".to_string());
        assert_eq!(config.file_path, "poem.txt".to_string());
    }

    /// Test that invalid arguments return an error
    #[test]
    #[should_panic]
    fn invalid_args() {
        let test_args = vec!["ztask".to_string()];
        Config::build(&test_args).unwrap();
    }

    /// Test that the case-sensitive query is found in the contents
    #[test]
    fn case_sensitive() {
        // contstraint: This test depends on the contents of poem.txt!
        let test_args = vec!["ztask".to_string(), "Who".to_string(), "poem.txt".to_string()];
        let config = Config::build(&test_args).unwrap();

        run(config).unwrap();
    }

}