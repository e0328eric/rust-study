use crate::lexer::token::Token;
use crate::lexer;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Let { name: String, value: Expr }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Const(isize),
    String(String),
    Bool(bool),
}
