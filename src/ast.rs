use crate::token::Token;
use std::fmt::format;

#[derive(PartialEq, Debug)]
pub enum Statement {
    LetStatement {
        tok: Token,
        name: Token,
        value: Expression,
    },
    ReturnStatement {
        tok: Token,
        return_value: Expression,
    },
    BadStatement,
}

#[derive(PartialEq, Debug)]
pub enum Expression {
    Identifier(Token),
}

pub struct Program {
    pub statements: Vec<Statement>,
}
