use crate::ast;
use crate::lexer;
use crate::token;

#[derive(Default)]
pub struct Parser {
    pub l: lexer::Lexer,
    pub cur_token: token::Token,
    pub peek_token: token::Token,
}

impl Parser {
    fn new(lex: lexer::Lexer) -> Self {
        let mut p = Parser {
            l: lex,
            ..Default::default()
        };
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    fn parse_program(&mut self) -> ast::Program {
        let mut prog = ast::Program {
            statements: Vec::new(),
        };

        while self.cur_token != token::Token::Eof {
            prog.statements
                .push(self.parse_statement().expect("Bad statement"));
            self.next_token();
        }

        prog
    }

    fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.cur_token {
            token::Token::Let => Some(self.parse_let_statement().expect("bad let statement")),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<ast::Statement> {
        let stmt_token = self.cur_token.clone();

        if !self.expect_peek(token::Token::Ident("".to_string())) {
            return None;
        }

        let stmt = ast::Statement::LetStatement {
            tok: stmt_token,
            name: self.cur_token.clone(),
            value: ast::Expression::Identifier(token::Token::Illegal),
        };

        if !self.expect_peek(token::Token::Assign) {
            return None;
        }

        while !self.cur_token_is(token::Token::Semicolon) {
            self.next_token();
        }

        return Some(stmt);
    }

    fn expect_peek(&mut self, t: token::Token) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn peek_token_is(&self, t: token::Token) -> bool {
        self.peek_token.same_tok(t)
    }

    fn cur_token_is(&self, t: token::Token) -> bool {
        t == self.cur_token
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Statement;

    #[test]
    fn test_let_statements() {
        let input = "
let x = 5;
let y = 10;
let foobar = 838383;
";
        let l = lexer::Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();

        assert_eq!(program.statements.len(), 3);

        let tests = vec!["x", "y", "foobar"];

        for i in 0..3 {
            assert!(test_let_statement(&program.statements[i], tests[i]))
        }
    }

    fn test_let_statement(s: &ast::Statement, name: &str) -> bool {
        if let ast::Statement::LetStatement {
            tok: t,
            name: n,
            value: v,
        } = s
        {
            if *t != token::Token::Let {
                println!("token is not let");
                return false;
            }

            if let n = token::Token::Ident(name.to_string()) {
                true
            } else {
                println!("token has wrong value");
                false
            }
        } else {
            println!("statement is not let");
            false
        }
    }
}
