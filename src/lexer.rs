use crate::token::{self, Token};

#[derive(Default, Debug)]
pub struct Lexer {
    pub input: String,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut result = Self {
            input: input.to_string(),
            ..Default::default()
        };
        result.read_char();
        result
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.chars().count() {
            self.ch = '\x00';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let mut skip = true;
        self.skip_whitespace();

        let tok: Token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            ';' => Token::Semicolon,
            '(' => Token::Lparen,
            ')' => Token::Rparen,
            '+' => Token::Plus,
            '{' => Token::Lbrace,
            '}' => Token::Rbrace,
            ',' => Token::Comma,
            '-' => Token::Minus,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Neq
                } else {
                    Token::Bang
                }
            }
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '<' => Token::Lt,
            '>' => Token::Gt,
            '\x00' => Token::Eof,
            other => {
                if is_letter(&other) {
                    skip = false;
                    token::string_to_token(self.read_identifier())
                } else if is_digit(&other) {
                    skip = false;
                    Token::Int(self.read_number())
                } else {
                    Token::Illegal
                }
            }
        };

        if skip {
            self.read_char();
        }

        tok
    }

    pub fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(&self.ch) {
            self.read_char();
        }

        return self.input[position..self.position].to_string();
    }

    pub fn read_number(&mut self) -> i64 {
        let position = self.position;
        while is_digit(&self.ch) {
            self.read_char();
        }

        return self.input[position..self.position]
            .to_string()
            .parse()
            .unwrap();
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    pub fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return '\x00';
        } else {
            return self.input.chars().nth(self.read_position).unwrap();
        }
    }
}
pub fn is_letter(ch: &char) -> bool {
    //!['=', ';', '(', ')', '+', '{', '}', ',', '\x00'].contains(ch)
    ch.is_ascii_alphabetic()
}

pub fn is_digit(ch: &char) -> bool {
    ch.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_next_token_one() {
        let input = "=+(){},;";

        let mut tests: Vec<Token> = Vec::new();
        tests.push(Token::Assign);
        tests.push(Token::Plus);
        tests.push(Token::Lparen);
        tests.push(Token::Rparen);
        tests.push(Token::Lbrace);
        tests.push(Token::Rbrace);
        tests.push(Token::Comma);
        tests.push(Token::Semicolon);
        tests.push(Token::Eof);

        let mut l = Lexer::new(input);

        for t in tests {
            let tok = l.next_token();

            assert_eq!(tok, t);
        }
    }
    #[test]
    fn test_next_token_two() {
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
10 != 9;
";

        let mut tests: Vec<Token> = Vec::new();
        tests.push(Token::Let);
        tests.push(Token::Ident("five".to_string()));
        tests.push(Token::Assign);
        tests.push(Token::Int(5));
        tests.push(Token::Semicolon);
        tests.push(Token::Let);
        tests.push(Token::Ident("ten".to_string()));
        tests.push(Token::Assign);
        tests.push(Token::Int(10));
        tests.push(Token::Semicolon);
        tests.push(Token::Let);
        tests.push(Token::Ident("add".to_string()));
        tests.push(Token::Assign);
        tests.push(Token::Function);
        tests.push(Token::Lparen);
        tests.push(Token::Ident("x".to_string()));
        tests.push(Token::Comma);
        tests.push(Token::Ident("y".to_string()));
        tests.push(Token::Rparen);
        tests.push(Token::Lbrace);
        tests.push(Token::Ident("x".to_string()));
        tests.push(Token::Plus);
        tests.push(Token::Ident("y".to_string()));
        tests.push(Token::Semicolon);
        tests.push(Token::Rbrace);
        tests.push(Token::Semicolon);
        tests.push(Token::Let);
        tests.push(Token::Ident("result".to_string()));
        tests.push(Token::Assign);
        tests.push(Token::Ident("add".to_string()));
        tests.push(Token::Lparen);
        tests.push(Token::Ident("five".to_string()));
        tests.push(Token::Comma);
        tests.push(Token::Ident("ten".to_string()));
        tests.push(Token::Rparen);
        tests.push(Token::Semicolon);
        tests.push(Token::Bang);
        tests.push(Token::Minus);
        tests.push(Token::Slash);
        tests.push(Token::Asterisk);
        tests.push(Token::Int(5));
        tests.push(Token::Semicolon);
        tests.push(Token::Int(5));
        tests.push(Token::Lt);
        tests.push(Token::Int(10));
        tests.push(Token::Gt);
        tests.push(Token::Int(5));
        tests.push(Token::Semicolon);
        tests.push(Token::If);
        tests.push(Token::Lparen);
        tests.push(Token::Int(5));
        tests.push(Token::Lt);
        tests.push(Token::Int(10));
        tests.push(Token::Rparen);
        tests.push(Token::Lbrace);
        tests.push(Token::Return);
        tests.push(Token::True);
        tests.push(Token::Semicolon);
        tests.push(Token::Rbrace);
        tests.push(Token::Else);
        tests.push(Token::Lbrace);
        tests.push(Token::Return);
        tests.push(Token::False);
        tests.push(Token::Semicolon);
        tests.push(Token::Rbrace);
        tests.push(Token::Int(10));
        tests.push(Token::Eq);
        tests.push(Token::Int(10));
        tests.push(Token::Semicolon);
        tests.push(Token::Int(10));
        tests.push(Token::Neq);
        tests.push(Token::Int(9));
        tests.push(Token::Semicolon);
        tests.push(Token::Eof);

        let mut l = Lexer::new(input);

        for t in tests {
            let tok = l.next_token();

            assert_eq!(tok, t);
        }
    }
}
