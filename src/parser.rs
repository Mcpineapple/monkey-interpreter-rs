use crate::ast;
use crate::lexer;
use crate::token;

#[derive(Default)]
pub struct Parser {
    pub l: lexer::Lexer,
    pub curToken: token::Token,
    pub peekToken: token::Token,
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

    fn parseProgram(&mut self) -> ast::Program {
        let mut prog = ast::Program {
            statements: Vec::new(),
        };

        while self.curToken != token::Token::Eof {
            prog.statements
                .push(self.parseStatement().expect("Bad statement"));
            self.nextToken();
        }

        prog
    }

    fn parseStatement(&mut self) -> Option<ast::Statement> {
        match self.curToken {
            token::Token::Let => Some(self.parseLetStatement().expect("bad let statement")),
            _ => None,
        }
    }

    fn parseLetStatement(&mut self) -> Option<ast::Statement> {
        let stmt_token = self.curToken.clone();

        if !self.expectPeek(token::Token::Ident("".to_string())) {
            return None;
        }

        let stmt = ast::Statement::LetStatement {
            tok: stmt_token,
            name: self.curToken.clone(),
            value: ast::Expression::Identifier(token::Token::Illegal),
        };

        if !self.expectPeek(token::Token::Assign) {
            return None;
        }

        while !self.curTokenIs(token::Token::Semicolon) {
            self.nextToken();
        }

        return Some(stmt);
    }

    fn expectPeek(&mut self, t: token::Token) -> bool {
        if self.peekTokenIs(t) {
            self.nextToken();
            true
        } else {
            false
        }
    }

    fn peekTokenIs(&self, t: token::Token) -> bool {
        self.peekToken.same_tok(t)
    }

    fn curTokenIs(&self, t: token::Token) -> bool {
        t == self.curToken
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Statement;

    #[test]
    fn test_LetStatements() {
        let input = "
let x = 5;
let y = 10;
let foobar = 838383;
";
        let l = lexer::Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parseProgram();

        assert_eq!(program.statements.len(), 3);

        let tests = vec!["x", "y", "foobar"];

        for i in 0..3 {
            assert!(testLetStatement(&program.statements[i], tests[i]))
        }
    }

    fn testLetStatement(s: &ast::Statement, name: &str) -> bool {
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
