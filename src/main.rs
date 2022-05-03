use std::env;
use std::fs;
use std::process;

fn main() {

    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem while parsing arguments: {}", err);
        process::exit(1);
    });

    println!(
    "This is the query: {},
    This is the file_path: {} ",
    config.query, config.file_path
    );

    let file_content = fs::read_to_string("poem.txt");
    println!("This is the file content {:?}", file_content);

}

struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() <3 {
            return Err("Not enough arguments.");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config{ query, file_path})
    }
}