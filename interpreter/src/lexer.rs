use crate::token::{Token, KEYWORDS};

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

fn is_letter(ch: u8) -> bool {
    ch.is_ascii_alphabetic() || ch == b'_'
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

    fn read_with_condition<F>(&mut self, f: F) -> String
        where F: Fn(u8) -> bool {
        let position = self.position;
        while f(self.ch) {
            self.read_char();
        }
        self.position -= 1;
        self.read_position -= 1;
        String::from_utf8_lossy(&self.input.as_bytes()[position..=self.position]).to_string()
    }

    fn take_token(&mut self) -> Token {
        while self.ch.is_ascii_whitespace() {
            self.read_char()
        }
        match self.ch {
            b'=' => Token::ASSIGN,
            b'+' => Token::PLUS,
            b'-' => Token::MINUS,
            b'!' => Token::BANG,
            b'*' => Token::ASTERISK,
            b'/' => Token::SLASH,
            b'<' => Token::LT,
            b'>' => Token::GT,
            b';' => Token::SEMICOLON,
            b',' => Token::COMMA,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            0 => Token::EOF,
            _ if is_letter(self.ch) => {
                    let letter = self.read_with_condition(|x| is_letter(x));
                    let corr_tok = KEYWORDS.iter().find(|&x| x.0 == &letter);
                    if corr_tok.is_some() {
                        corr_tok.unwrap().1.clone()
                    } else {
                        Token::IDENT(letter)
                    }
                },
            _ if self.ch.is_ascii_digit() => {
                Token::INT(self.read_with_condition(|x| x.is_ascii_digit()).parse().unwrap())
            }
            _ => Token::ILLEGAL
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if self.read_position >= self.input.len() + 1 {
            return None;
        }
        self.read_char();
        Some(self.take_token())
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
    x+y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}";
        let lex = Lexer::new(input);
        let output: Vec<Token> = lex.collect();
        let expected = vec![
            Token::LET,
            Token::IDENT(String::from("five")),
            Token::ASSIGN,
            Token::INT(5),
            Token::LET,
            Token::IDENT(String::from("ten")),
            Token::ASSIGN,
            Token::INT(10),
            Token::LET,
            Token::IDENT(String::from("add")),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT(String::from("x")),
            Token::COMMA,
            Token::IDENT(String::from("y")),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT(String::from("x")),
            Token::PLUS,
            Token::IDENT(String::from("y")),
            Token::SEMICOLON,
            Token::RBRACE,
        ];
        assert_eq!(output, expected);
    }
}
