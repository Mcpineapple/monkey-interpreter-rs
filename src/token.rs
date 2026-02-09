mod token {
    const ILLEGAL: &str = "ILLEGAL";
    const EOF: &str = "EOF";

    // Identifiers + literals
    const IDENT: &str = "IDENT";
    const INT: &str = "INT";

    // operators
    const ASSIGN: &str = "=";
    const PLUS: &str = "+";

    // Delimiters
    const COMMA: &str = ",";
    const SEMICOLON: &str = ";";

    const LPAREN: &str = "(";
    const RPAREN: &str = ")";
    const LBRACE: &str = "{";
    const RBRACE: &str = "}";

    // Keywords
    const FUNCTION: &str = "FUNCTION";
    const LET: &str = "LET";

    type TokenType = String;

    struct Token {
        Type: TokenType,
        Literal: &str,
    }
}
