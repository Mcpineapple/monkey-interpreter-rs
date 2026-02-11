use std::fmt::format;

use crate::token::Token;
trait Node {
    fn tokenLiteral(&self) -> String;
}

trait Statement: Node {
    fn statementNode(&self);
}

trait Expression: Node {
    fn expressionNode(&self);
}

struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn tokenLiteral(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].tokenLiteral()
        } else {
            "".to_string()
        }
    }
}

struct LetStatement {
    token: Token, // let
    name: Identifier,
    value: Box<dyn Expression>,
}

impl Node for LetStatement {
    fn tokenLiteral(&self) -> String {
        self.token.to_string()
    }
}

impl Statement for LetStatement {
    fn statementNode(&self) {}
}

struct Identifier {
    token: Token,
    value: String,
}

impl Node for Identifier {
    fn tokenLiteral(&self) -> String {
        self.token.to_string()
    }
}

impl Expression for Identifier {
    fn expressionNode(&self) {}
}
