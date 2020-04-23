#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Lexer {
    Add,
    Sub,
    Left,
    Right,
    Input,
    Output,
    BLoop,
    ELoop,
}

// This lexer cannot treat the comment like "+" and so on
pub fn lex(s: &String) -> Vec<Lexer> {
    let y: Vec<u8> = s.bytes().collect();
    let mut output: Vec<Lexer> = Vec::new();
    for i in y {
        match i {
            43 => output.push(Lexer::Add),
            44 => output.push(Lexer::Input),
            45 => output.push(Lexer::Sub),
            46 => output.push(Lexer::Output),
            60 => output.push(Lexer::Left),
            62 => output.push(Lexer::Right),
            91 => output.push(Lexer::BLoop),
            93 => output.push(Lexer::ELoop),
            _ => {},
        }
    }
    output
}
