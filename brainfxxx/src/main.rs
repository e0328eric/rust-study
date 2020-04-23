use std::env;
use std::fs;

mod modint;
mod tape;
mod lexer;
mod parser;
mod interpreter;

use crate::tape::Tape;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input = if args.len() > 2 { Some(&args[2]) } else { None };
    let script = fs::read_to_string(filename).unwrap();
    let mut tape = Tape::new();
    let parsed = parser::parser(&lexer::lex(&script));
    let result = interpreter::interpreter(&parsed, &mut tape, &(input, 0));
    println!("{}", result);
}
