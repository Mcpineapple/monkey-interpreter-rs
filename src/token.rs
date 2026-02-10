#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    // Identifiers + literals
    Ident(String),
    Int(i64),

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // Keywords
    Function,
    Let,
}

pub fn stringToToken(string: String) -> Token {
    match string.as_str() {
        "let" => Token::Let,
        "fn" => Token::Function,
        other => Token::Ident(string),
    }
}
