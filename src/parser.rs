use crate::ast;
use crate::lexer;
use crate::token;

#[derive(Default)]
pub struct Parser {
    l: lexer::Lexer,
    curToken: token::Token,
    peekToken: token::Token,
}

impl Parser {
    fn new(lex: lexer::Lexer) -> Self {
        let mut p = Parser {
            l: lex,
            ..Default::default()
        };
        p.nextToken();
        p.nextToken();
        p
    }

    fn nextToken(&mut self) {
        self.curToken = self.peekToken.clone();
        self.peekToken = self.l.nextToken();
    }
}
