use crate::ast;
use crate::lexer;
use crate::token;

#[derive(Default)]
pub struct Parser {
    pub l: lexer::Lexer,
    pub cur_token: token::Token,
    pub peek_token: token::Token,
    pub errors: Vec<String>,
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
            prog.statements.push(
                self.parse_statement()
                    .unwrap_or(ast::Statement::BadStatement),
            );
            self.next_token();
        }

        prog
    }

    fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.cur_token {
            token::Token::Let => Some(
                self.parse_let_statement()
                    .unwrap_or(ast::Statement::BadStatement),
            ),
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
        if self.peek_token_is(t.clone()) {
            self.next_token();
            true
        } else {
            self.peek_error(t.clone());
            false
        }
    }

    fn peek_token_is(&self, t: token::Token) -> bool {
        self.peek_token.same_tok(t)
    }

    fn cur_token_is(&self, t: token::Token) -> bool {
        t == self.cur_token
    }

    fn get_errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    fn peek_error(&mut self, t: token::Token) {
        let msg = format!(
            "expected next token to be {}, got {} instead",
            t.to_string(),
            self.peek_token.to_string()
        );
        self.errors.push(msg);
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
        check_parser_errors(&p);

        assert_eq!(program.statements.len(), 3);

        let tests = vec!["x", "y", "foobar"];

        for i in 0..3 {
            assert!(test_let_statement(&program.statements[i], tests[i]))
        }
    }

    #[test]
    #[should_panic]
    fn test_bad_let_statements() {
        let input = "
let x = 5;
let y = 10;
let = 838383;
";
        let l = lexer::Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        check_parser_errors(&p);

        assert_eq!(program.statements.len(), 3);

        let tests = vec!["x", "y", "foobar"];

        for i in 0..3 {
            assert!(test_let_statement(&program.statements[i], tests[i]))
        }
    }

    fn check_parser_errors(p: &Parser) {
        let errors = p.get_errors();
        if errors.len() == 0 {
            return;
        }

        println!("parser has {} errors", errors.len());

        for msg in errors {
            println!("parser error : {}", msg);
        }

        panic!();
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
