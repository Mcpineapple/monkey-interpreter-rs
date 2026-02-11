use crate::token::{self, Token};

#[derive(Default, Debug)]
pub struct Lexer<'a> {
    pub input: &'a str,
    pub position: usize,
    pub readPosition: usize,
    pub ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut result = Self {
            input,
            ..Default::default()
        };
        result.readChar();
        result
    }

    pub fn readChar(&mut self) {
        if self.readPosition >= self.input.chars().count() {
            self.ch = '\x00';
        } else {
            self.ch = self.input.chars().nth(self.readPosition).unwrap();
        }

        self.position = self.readPosition;
        self.readPosition += 1;
    }

    pub fn nextToken(&mut self) -> Token {
        let mut skip = true;
        self.skipWhitespace();

        let tok: Token = match self.ch {
            '=' => {
                if self.peekChar() == '=' {
                    self.readChar();
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
                if self.peekChar() == '=' {
                    self.readChar();
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
                if isLetter(&other) {
                    skip = false;
                    token::stringToToken(self.readIdentifier())
                } else if isDigit(&other) {
                    skip = false;
                    Token::Int(self.readNumber())
                } else {
                    Token::Illegal
                }
            }
        };

        if skip {
            self.readChar();
        }

        tok
    }

    pub fn readIdentifier(&mut self) -> String {
        let position = self.position;
        while isLetter(&self.ch) {
            self.readChar();
        }

        return self.input[position..self.position].to_string();
    }

    pub fn readNumber(&mut self) -> i64 {
        let position = self.position;
        while isDigit(&self.ch) {
            self.readChar();
        }

        return self.input[position..self.position]
            .to_string()
            .parse()
            .unwrap();
    }

    pub fn skipWhitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.readChar();
        }
    }

    pub fn peekChar(&self) -> char {
        if self.readPosition >= self.input.len() {
            return '\x00';
        } else {
            return self.input.chars().nth(self.readPosition).unwrap();
        }
    }
}
pub fn isLetter(ch: &char) -> bool {
    //!['=', ';', '(', ')', '+', '{', '}', ',', '\x00'].contains(ch)
    ch.is_ascii_alphabetic()
}

pub fn isDigit(ch: &char) -> bool {
    ch.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_NextToken_One() {
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
            let tok = l.nextToken();

            assert_eq!(tok, t);
        }
    }
    #[test]
    fn test_NextToken_Two() {
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
            let tok = l.nextToken();

            assert_eq!(tok, t);
        }
    }
}
