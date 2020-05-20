use std::convert::From;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    ILLEGAL, EOF,
    // Identifiers + literals
    IDENT(String), INT(isize),
    // Operators
    ASSIGN, PLUS, MINUS, BANG,
    ASTERISK, SLASH, LT, GT,
    EQ, NOTEQ,
    // Delimiters
    COMMA, SEMICOLON, LPAREN,
    RPAREN, LBRACE, RBRACE,
    // Keywords
    FUNCTION, LET, TRUE, FALSE,
    IF, ELSE, RETURN,
}

pub fn is_letter(ch: u8) -> bool {
    ch.is_ascii_alphabetic() || ch == b'_'
}

pub fn is_symbol(ch: u8) -> bool {
    let symbol_lst: [u8; 2] = [b'=', b'!']; // only support == and !=
    symbol_lst.contains(&ch)
}

impl From<String> for Token {
    fn from(s: String) -> Self {
        match s.as_bytes() {
            b"=" => Token::ASSIGN,
            b"+" => Token::PLUS,
            b"-" => Token::MINUS,
            b"!" => Token::BANG,
            b"*" => Token::ASTERISK,
            b"/" => Token::SLASH,
            b"<" => Token::LT,
            b">" => Token::GT,
            b"==" => Token::EQ,
            b"!=" => Token::NOTEQ,
            b"," => Token::COMMA,
            b";" => Token::SEMICOLON,
            b"(" => Token::LPAREN,
            b")" => Token::RPAREN,
            b"{" => Token::LBRACE,
            b"}" => Token::RBRACE,
            b"fn" => Token::FUNCTION,
            b"let" => Token::LET,
            b"true" => Token::TRUE,
            b"false" => Token::FALSE,
            b"if" => Token::IF,
            b"else" => Token::ELSE,
            b"return" => Token::RETURN,
            [0] => Token::EOF,
            _ if s.as_bytes().iter().all(|&x| is_letter(x))
                => Token::IDENT(s),
            _ if s.as_bytes().iter().all(|&x| x.is_ascii_digit())
                => Token::INT(s.parse().unwrap()),
            _ => Token::ILLEGAL,
        }
    }
}
