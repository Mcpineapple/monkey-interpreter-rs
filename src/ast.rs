use crate::token::Token;
use std::fmt::format;

#[derive(PartialEq, Debug)]
pub enum Statement {
    LetStatement(Token, Token, Expression),
}

#[derive(PartialEq, Debug)]
pub enum Expression {
    Identifier(Token),
}

pub struct Program {
    pub statements: Vec<Statement>,
}
