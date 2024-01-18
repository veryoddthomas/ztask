use std::error::Error;
use std::fs;
use std::env;

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

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

/*
  From rust doc including this example...
  https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html

  Notice that we need to define an explicit lifetime 'a in the signature of
  search and use that lifetime with the contents argument and the return value.
  Recall in Chapter 10 that the lifetime parameters specify which argument
  lifetime is connected to the lifetime of the return value. In this case, we
  indicate that the returned vector should contain string slices that reference
  slices of the argument contents (rather than the argument query).
  In other words, we tell Rust that the data returned by the search function
  will live as long as the data passed into the search function in the contents
  argument. This is important! The data referenced by a slice needs to be valid
  for the reference to be valid; if the compiler assumes we’re making string
  slices of query rather than contents, it will do its safety checking
  incorrectly.
*/

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
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

    /// Test that the case-sensitive query is found in the contents
    #[test]
    fn case_sensitive_low_level() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    /// Test that the case-insensitive query is found in the contents
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}