#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    ILLEGAL, EOF,
    // Identifiers + literals
    IDENT(String), INT(isize),
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

pub const KEYWORDS: [(&str, Token); 2] = [
    ("fn", Token::FUNCTION),
    ("let", Token::LET)
];

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

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        self.position -= 1;
        self.read_position -= 1;
        String::from_utf8_lossy(&self.input.as_bytes()[position..=self.position]).to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        self.position -= 1;
        self.read_position -= 1;
        String::from_utf8_lossy(&self.input.as_bytes()[position..=self.position]).to_string()
    }

    fn take_token(&mut self) -> Token {
        if self.ch.is_ascii_whitespace() {
            self.read_char()
        }
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
            _ if is_letter(self.ch) => {
                    let letter = self.read_identifier();
                    let corr_tok = KEYWORDS.iter().find(|&x| x.0 == &letter);
                    if corr_tok.is_some() {
                        corr_tok.unwrap().1.clone()
                    } else {
                        Token::IDENT(letter)
                    }
                },
            _ if self.ch.is_ascii_digit() => {
                Token::INT(self.read_number().parse().unwrap())
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
    fn basic_lexing() {
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

    #[test]
    fn lex_identifier() {
        let input = "let= foo = bar}fn;123";
        let lex = Lexer::new(input);
        let output: Vec<Token> = lex.collect();
        let expected = vec![
            Token::LET,
            Token::ASSIGN,
            Token::IDENT(String::from("foo")),
            Token::ASSIGN,
            Token::IDENT(String::from("bar")),
            Token::RBRACE,
            Token::FUNCTION,
            Token::SEMICOLON,
            Token::INT(123),
            Token::EOF
        ];
        assert_eq!(output, expected);
    }
}
