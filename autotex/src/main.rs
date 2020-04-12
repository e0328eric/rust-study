extern crate signal;
use std::env;
use std::process;
use std::thread;
use std::time::{Duration, Instant};

use signal::Signal;
use signal::trap::Trap;

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

fn compile_tex(args: &Vec<String>, fname: &str, init_time: Vec<std::time::SystemTime>) {
    let trap_int = Trap::trap(&[Signal::SIGINT]);
    let now = Instant::now();
    let mut time = init_time;
    let continuous_option = String::from("-v");
    let args_removed: Vec<&String> = args.iter()
        .filter(|x| ***x != continuous_option).collect();
    engines::run_engine(&args_removed, &fname);
    print_quit_message(&args, &continuous_option);
    if args.contains(&continuous_option) {
        loop {
            if time != utils::get_file_times(&None) {
                engines::run_engine(&args_removed, &fname);
                print_quit_message(&args, &continuous_option);
                time = utils::get_file_times(&None);
            }
            thread::sleep(Duration::from_secs(1));
            match trap_int.wait(now) {
                Some(_) => {
                    println!("\nQuitting");
                    break
                },
                None => continue,
            };
        }
    }
}

fn print_quit_message(args: &Vec<String>, continuous_option: &String) {
    if args.contains(&continuous_option) {
        println!("Press Ctrl+C to finish the program");
    }
}
