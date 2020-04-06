use std::env;
use std::process;

mod engines;
mod utils;

use crate::engines::TeXEngine;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (filename, options) = args.split_last().unwrap_or_else(|| {
       eprintln!("Wierd Error!!");
       process::exit(1)
    });
    let mut init_time = utils::get_file_times(&None);
    match engines::take_engine(&options[1..]) {
        Ok((_, eng)) => {
            let x = vec![
                TeXEngine(String::from(eng)),
                TeXEngine(String::from(eng))
            ];
            TeXEngine::run(&x, &filename);
            loop {
                if init_time != utils::get_file_times(&None) {
                    TeXEngine::run(&x, &filename);
                    init_time = utils::get_file_times(&None);
                }
            }
        },
        Err(e) => eprintln!("autotex Error : {}", e),
    }
}
