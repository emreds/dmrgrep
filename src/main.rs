use std::env;
use std::process;

use dmrgrep::Config;

fn main() {

    let args: Vec<String> = env::args().collect();
    // We use `.is_err()` here because we just check if the `CASE_INSENSITIVE` environment
    // variable is defined. It does not matter what value it has.
    // When we use `is_err()` program not panics if `is_err()` correct.
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
    // Difference between `unwrap_or` and `unwrap_or_else` is; 
    // in `unwrap_or_else` returning value for
    // `err` only evaluated when the error triggered.
    let config: Config = Config::new(&args, case_sensitive).unwrap_or_else(|err|{
        eprintln!("Problem while parsing arguments: {}", err);
        process::exit(1);
    });

    let contents = dmrgrep::run(config);

    if let Err(e) = contents {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    
}
