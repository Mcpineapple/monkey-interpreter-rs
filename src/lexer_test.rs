use crate::lexer;
use crate::token::Token;

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

        let mut l = lexer::Lexer::new(input);

        for t in tests {
            let tok = l.nextToken();

            assert_eq!(tok, t);
        }
    }
}
