use std::env;
use std::process;

use dmrgrep::Config;

fn main() {

    let args: Vec<String> = env::args().collect();
    // Difference between `unwrap_or` and `unwrap_or_else` is; 
    // in `unwrap_or_else` returning value for
    // `err` only evaluated when the error triggered.
    let config: Config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem while parsing arguments: {}", err);
        process::exit(1);
    });

    let contents = dmrgrep::run(config);

    if let Err(e) = contents {
        println!("Application error: {}", e);
        process::exit(1);
    }
    
}
