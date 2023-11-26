use core::fmt;

use crate::token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Let(Identifier, Expression),
    Return(Expression),
    Expression(token::Token, Expression),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Identifier(token::Token),
    Integer(token::Token),
}

#[derive(Default, Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PROGRAM: \n")?;
        for (idx, statement) in self.statements.iter().enumerate() {
            write!(f, "--- {}: {:?}\n", idx, statement)?;
        }
        Ok(())
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct Identifier {
    pub token: token::Token,
    pub value: String,
}

impl Identifier {
    pub fn new(token: token::Token, value: &str) -> Self {
        Self {
            token,
            value: value.to_owned(),
        }
    }
}
