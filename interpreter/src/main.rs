#![allow(unused)]
extern crate signal;
use std::time::Instant;

use signal::trap::Trap;
use signal::Signal;

mod lexer;
mod parser;
mod repl;

fn main() {
    let trap_int = Trap::trap(&[Signal::SIGINT]);
    let now = Instant::now();
    while let None = trap_int.wait(now) {
        repl::start_repl();
    }
}
