#![warn(rust_2018_idioms, clippy::all)]

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(error) => {
                eprintln!("{:?}", error);
                break;
            }
            Ok(line) => {
                println!("{}", line);
            }
        }
    }
}
