use std::str::FromStr;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
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

const TOKSTR: [(Token, &str); 23] = [
    (Token::ASSIGN, "="),
    (Token::PLUS, "+"),
    (Token::MINUS, "-"),
    (Token::BANG, "!"),
    (Token::ASTERISK, "*"),
    (Token::SLASH, "/"),
    (Token::LT, "<"),
    (Token::GT, ">"),
    (Token::EQ, "=="),
    (Token::NOTEQ, "!="),
    (Token::COMMA, ","),
    (Token::SEMICOLON, ";"),
    (Token::LPAREN, "("),
    (Token::RPAREN, ")"),
    (Token::LBRACE, "{"),
    (Token::RBRACE, "}"),
    (Token::FUNCTION, "fn"),
    (Token::LET, "let"),
    (Token::TRUE, "true"),
    (Token::FALSE, "false"),
    (Token::IF, "if"),
    (Token::ELSE, "else"),
    (Token::RETURN, "return"),
];

pub fn is_letter(ch: u8) -> bool {
    ch.is_ascii_alphabetic() || ch == b'_'
}

pub fn is_symbol(ch: u8) -> bool {
    let symbol_lst: [u8; 2] = [b'=', b'!']; // only support == and !=
    symbol_lst.contains(&ch)
}

impl ToString for Token {
    fn to_string(&self) -> String {
        if *self >= Token::ASSIGN {
            TOKSTR.iter().find(|x| x.0 == *self).unwrap().1.to_string()
        } else {
            match self {
                Token::IDENT(s) => s.clone(),
                Token::INT(n) => n.to_string(),
                _ => panic!("Both eof and illegal character cannot converted to string.")
            }
        }
    }
}

impl FromStr for Token {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let is_in_list = TOKSTR.iter().find(|x| x.1 == s);
        match is_in_list {
            None => {
                if s.as_bytes().iter().all(|&x| is_letter(x)) {
                    Ok(Token::IDENT(s.to_string()))
                } else if s.as_bytes().iter().all(|&x| x.is_ascii_digit()) {
                    Ok(Token::INT(s.parse().unwrap()))
                } else if s.as_bytes() == [0] {
                    Ok(Token::EOF)
                } else {
                    Ok(Token::ILLEGAL)
                }
            },
            Some(tuple) => Ok(tuple.0.clone()),
        }
    }
}
