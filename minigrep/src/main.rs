use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);    // pub fn exit(code: u32) -> !
    });

    if config.case_sensitive {
        println!("Searching for {}", config.query);
    } else {
        println!("Searching for {} not considering uppercase", config.query);
    }
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e); process::exit(1);
    }
}
