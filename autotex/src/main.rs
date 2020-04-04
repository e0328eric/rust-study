use std::env;
use std::process;

use autotex::{self, TeXEngine};

fn main() {
    let args: Vec<String> = env::args().collect();
    let (filename, options) = args.split_last().unwrap_or_else(|| {
       eprintln!("Wierd Error!!");
       process::exit(1)
    });
    match autotex::take_engine(&options[1..]) {
        Ok(eng) => {
            let x = vec![
                TeXEngine(String::from(eng)),
                TeXEngine(String::from(eng))
            ];
            TeXEngine::run(&x, &filename);
        },
        Err(e) => eprintln!("autotex Error : {}", e),
    }
}
