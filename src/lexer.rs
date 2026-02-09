use crate::token::{self, Token};

#[derive(Default)]
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
        let tok: Token = match self.ch {
            '=' => Token::Assign,
            ';' => Token::Semicolon,
            '(' => Token::Lparen,
            ')' => Token::Rparen,
            '+' => Token::Plus,
            '{' => Token::Lbrace,
            '}' => Token::Rbrace,
            ',' => Token::Comma,
            '\x00' => Token::Eof,
            _ => Token::Illegal,
        };

        self.readChar();

        tok
    }
}
