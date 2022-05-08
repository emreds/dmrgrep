use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String], case_sensitive: bool) -> Result<Config, &'static str> {
        if args.len() <3 {
            return Err("Not enough arguments.");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config{ query, file_path, case_sensitive})
    }
}

/*
 * Notice that we need an explicit lifetime 'a defined in the signature of search and 
 * used with the contents argument and the return value. 
 * Recall in Chapter 10 that the lifetime parameters specify which argument lifetime is 
 * connected to the lifetime of the return value. 
 * In this case, we indicate that the returned vector should contain string slices 
 * that reference slices of the argument contents (rather than the argument query).

* In other words, we tell Rust that the data returned by the search function 
* will live as long as the data passed into the search function in the contents argument. 
* This is important! The data referenced by a slice needs to be valid for the reference to be valid; 
* if the compiler assumes we’re making string slices of query rather than contents, it will do 
* its safety checking incorrectly.
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

    let mut results: Vec<&str> = Vec::new();
    let lowercase_query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&lowercase_query) {
            results.push(line);
        }
    }

    results
}

// In the function below usage of `?` indicates that 
// it will return error without panicking.
// `Box<dyn Error>` is a trait which covers all the errors. 
pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_path)?;

    let results = if config.case_sensitive {
        search(&config.query, &file_content)
    } else {
        search_case_insensitive(&config.query, &file_content)
    };

    for line in results{
        println!("{}", line);
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));

        
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}