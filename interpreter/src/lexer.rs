#[derive(Debug, PartialEq)]
pub enum Token {
    ILLEGAL, EOF,
    // Identifiers + literals
    IDENT, INT,
    // Operators
    ASSIGN, PLUS,
    // Delimiters
    COMMA, SEMICOLON, LPAREN,
    RPAREN, LBRACE, RBRACE,
    // Keywords
    FUNCTION, LET,
}

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

    fn take_token(&self) -> Token {
        match self.ch {
            b'=' => Token::ASSIGN,
            b';' => Token::SEMICOLON,
            b',' => Token::COMMA,
            b'+' => Token::PLUS,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            0 => Token::EOF,
            _ => Token::ILLEGAL,
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if self.read_position >= self.input.len() + 1 {
            return None;
        } else if self.read_position == self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    Some(self.take_token())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){?},;";
        let lex = Lexer::new(input);
        let output: Vec<Token> = lex.collect();
        let expected = vec![
            Token::ASSIGN,
            Token::PLUS,
            Token::LPAREN,
            Token::RPAREN,
            Token::LBRACE,
            Token::ILLEGAL,
            Token::RBRACE,
            Token::COMMA,
            Token::SEMICOLON,
            Token::EOF,
        ];
        assert_eq!(output, expected);
    }
}
