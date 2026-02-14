use crate::token::Token;
use std::fmt::format;

#[derive(PartialEq, Debug)]
pub enum Statement {
    LetStatement {
        tok: Token,
        name: Token,
        value: Expression,
    },
}

#[derive(PartialEq, Debug)]
pub enum Expression {
    Identifier(Token),
}

pub struct Program {
    pub statements: Vec<Statement>,
}
