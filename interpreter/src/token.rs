#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    ILLEGAL, EOF,
    // Identifiers + literals
    IDENT(String), INT(isize),
    // Operators
    ASSIGN, PLUS, MINUS, BANG,
    ASTERISK, SLASH, LT, GT,
    // Delimiters
    COMMA, SEMICOLON, LPAREN,
    RPAREN, LBRACE, RBRACE,
    // Keywords
    FUNCTION, LET, TRUE, FALSE,
    IF, ELSE, RETURN,
}

pub const KEYWORDS: [(&str, Token); 7] = [
    ("fn", Token::FUNCTION),
    ("let", Token::LET),
    ("true", Token::LET),
    ("false", Token::LET),
    ("if", Token::LET),
    ("else", Token::LET),
    ("return", Token::LET),
];

