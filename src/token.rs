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

impl Default for Token {
    fn default() -> Self {
        Token::Illegal
    }
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

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Self::Illegal => "Illegal token".to_string(),
            Self::Eof => "End of File".to_string(),

            // Identifiers + literals
            Self::Ident(cont) => format!("Identifier : {}", cont),
            Self::Int(num) => format!("Int : {}", num),

            // Operators
            Self::Assign => "=".to_string(),
            Self::Plus => "+".to_string(),
            Self::Minus => "-".to_string(),
            Self::Bang => "!".to_string(),
            Self::Asterisk => "*".to_string(),
            Self::Slash => "/".to_string(),

            Self::Lt => "<".to_string(),
            Self::Gt => ">".to_string(),

            Self::Eq => "==".to_string(),
            Self::Neq => "!=".to_string(),

            // Delimiters
            Self::Comma => ",".to_string(),
            Self::Semicolon => ";".to_string(),
            Self::Lparen => "(".to_string(),
            Self::Rparen => ")".to_string(),
            Self::Lbrace => "{".to_string(),
            Self::Rbrace => "}".to_string(),

            // Keywords
            Self::Function => "Function".to_string(),
            Self::Let => "Let".to_string(),
            Self::True => "True".to_string(),
            Self::False => "False".to_string(),
            Self::If => "If".to_string(),
            Self::Else => "Else".to_string(),
            Self::Return => "Ret".to_string(),
        }
    }
}
