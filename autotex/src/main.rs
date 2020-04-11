use std::env;
use std::process;

mod engines;
mod utils;

fn main() {
    let args: Vec<String> = utils::is_valid_args(env::args().collect());
    let (filename, options_) = args.split_last().unwrap_or_else(|| {
       eprintln!("Wierd Error!!");
       process::exit(1)
    });
    let options = Vec::from(options_);
    let init_time = utils::get_file_times(&None);
    compile_tex(&options, filename, init_time);
}

fn compile_tex(args_: &Vec<String>, fname: &str, init_time: Vec<std::time::SystemTime>) {
    let args: Vec<&String> = args_.iter().collect();
    let mut time = init_time;
    let continuous_option = String::from("-v");
    let args_removed: Vec<&String> = args_.iter()
        .filter(|x| ***x != continuous_option).collect();
    engines::run_engine(&args_removed, &fname);
    println!("Press Ctrl+C to finish the program");
    if args.contains(&&continuous_option) {
        loop {
            if time != utils::get_file_times(&None) {
                engines::run_engine(&args_removed, &fname);
                println!("Press Ctrl+C to finish the program");
                time = utils::get_file_times(&None);
            }
        }
    }
}
