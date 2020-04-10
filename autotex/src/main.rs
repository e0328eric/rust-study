use std::env;
use std::process;

mod engines;
mod utils;

fn main() {
    let args: Vec<String> = utils::is_valid_args(env::args().collect());
    let (filename, options) = args.split_last().unwrap_or_else(|| {
       eprintln!("Wierd Error!!");
       process::exit(1)
    });
    let mut init_time = utils::get_file_times(&None);
    engines::run_engine(&options, &filename);
    loop {
        if init_time != utils::get_file_times(&None) {
            engines::run_engine(&options, &filename);
            init_time = utils::get_file_times(&None);
        }
    }
}
