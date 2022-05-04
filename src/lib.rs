use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() <3 {
            return Err("Not enough arguments.");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config{ query, file_path})
    }
}

// In the function below usage of `?` indicates that 
// it will return error without panicking.
// `Box<dyn Error>` is a trait which covers all the errors. 
pub fn run (config: Config) -> Result<String, Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_path)?;
    
    println!("This is the file content {:?}", file_content);

    Ok(file_content)
}
