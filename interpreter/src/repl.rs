use std::io::{self, Write};

use crate::lexer::token::Token;
use crate::lexer;

pub fn start_repl() {
    print!(">> ");
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line);
    let lex = lexer::Lexer::new(&line);
    let output: Vec<Token> = lex.filter(|x| *x != Token::EOF).collect();
    println!("{:?}", output);
}
