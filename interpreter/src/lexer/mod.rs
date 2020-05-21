pub mod token;

use crate::lexer::token::{is_letter, is_symbol, Token};

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: String::from(input),
            position: 0,
            read_position: 0,
            ch: 0,
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }

    fn read_with_condition<F>(&mut self, f: F) -> String
    where F: Fn(u8) -> bool {
        let position = self.position;
        while f(self.ch) {
            self.read_char();
        }
        self.position -= 1;
        self.read_position -= 1;
        String::from_utf8_lossy(&self.input.as_bytes()[position..=self.position])
            .to_string()
    }

    fn take_token(&mut self) -> Token {
        while self.ch.is_ascii_whitespace() {
            self.read_char()
        }
        if is_symbol(self.ch) {
            self.read_with_condition(is_symbol).into()
        } else if is_letter(self.ch) {
            self.read_with_condition(is_letter).into()
        } else if self.ch.is_ascii_digit() {
            self.read_with_condition(|x| x.is_ascii_digit()).into()
        } else {
            String::from_utf8(vec![self.ch]).unwrap().into()
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if self.read_position > self.input.len() {
            None
        } else {
            self.read_char();
            Some(self.take_token())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lex() {
        let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;@
";
        let lex = Lexer::new(input);
        let output: Vec<Token> = lex.collect();
        let expected = vec![
            Token::LET, Token::IDENT(String::from("five")),
            Token::ASSIGN, Token::INT(5), Token::SEMICOLON, Token::LET,
            Token::IDENT(String::from("ten")), Token::ASSIGN,
            Token::INT(10), Token::SEMICOLON, Token::LET,
            Token::IDENT(String::from("add")),
            Token::ASSIGN, Token::FUNCTION, Token::LPAREN,
            Token::IDENT(String::from("x")), Token::COMMA,
            Token::IDENT(String::from("y")), Token::RPAREN,
            Token::LBRACE, Token::IDENT(String::from("x")),
            Token::PLUS, Token::IDENT(String::from("y")),
            Token::SEMICOLON, Token::RBRACE, Token::SEMICOLON, Token::LET,
            Token::IDENT(String::from("result")),
            Token::ASSIGN, Token::IDENT(String::from("add")),
            Token::LPAREN, Token::IDENT(String::from("five")),
            Token::COMMA, Token::IDENT(String::from("ten")),
            Token::RPAREN, Token::SEMICOLON,
            Token::BANG, Token::MINUS, Token::SLASH, Token::ASTERISK,
            Token::INT(5), Token::SEMICOLON,
            Token::INT(5), Token::LT, Token::INT(10), Token::GT, Token::INT(5),
            Token::SEMICOLON, Token::IF, Token::LPAREN, Token::INT(5), Token::LT,
            Token::INT(10), Token::RPAREN, Token::LBRACE, Token::RETURN,
            Token::TRUE, Token::SEMICOLON, Token::RBRACE, Token::ELSE,
            Token::LBRACE, Token::RETURN, Token::FALSE, Token::SEMICOLON,
            Token::RBRACE,
            Token::INT(10), Token::EQ, Token::INT(10), Token::SEMICOLON,
            Token::INT(10), Token::NOTEQ, Token::INT(9), Token::SEMICOLON,
            Token::ILLEGAL, Token::EOF
        ];
        assert_eq!(output, expected);
    }
}
