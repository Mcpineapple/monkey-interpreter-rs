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

    fn parseProgram(&mut self) -> ast::Program {
        let mut prog = ast::Program {
            statements: Vec::new(),
        };

        while self.curToken != token::Token::Eof {
            prog.statements.push(self.parseStatement().unwrap());
        }

        prog
    }

    fn parseStatement(&self) -> Option<ast::Statement> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::Statement;

    use super::*;
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

        assert_ne!(program.statements.len(), 3);

        let tests = vec!["x", "y", "foobar"];

        for i in 0..3 {
            assert!(testLetStatement(&program.statements[i], tests[i]))
        }
    }

    fn testLetStatement(s: &ast::Statement, name: &str) -> bool {
        if let ast::Statement::LetStatement(t, n, v) = s {
            if *t != token::Token::Let {
                println!("token is not let");
                return false;
            }

            if let n = token::Token::Ident(name.to_string()) {
                true
            } else {
                false
            }
        } else {
            println!("statement is not let");
            false
        }
    }
}
