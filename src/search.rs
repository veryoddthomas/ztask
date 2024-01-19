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

    /// Test that the case-sensitive query is found in the contents
    #[test]
    fn case_sensitive() {
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