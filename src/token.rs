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
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,

    Eq,
    Neq,

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
    True,
    False,
    If,
    Else,
    Return,
}

pub fn stringToToken(string: String) -> Token {
    match string.as_str() {
        "let" => Token::Let,
        "fn" => Token::Function,
        "return" => Token::Return,
        "else" => Token::Else,
        "if" => Token::If,
        "false" => Token::False,
        "true" => Token::True,
        other => Token::Ident(string),
    }
}
