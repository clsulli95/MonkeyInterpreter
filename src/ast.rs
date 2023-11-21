use crate::token;

trait Node {
    fn token_literal(&self) -> String;
}

trait Statement: Node {
    fn statement_node(&self);
}

trait Expression: Node {
    fn expression_node(&self);
}

struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if let Some(statement) = self.statements.first() {
            statement.token_literal()
        } else {
            "".to_string()
        }
    }
}

struct LetStatement {
    token: token::Token,
    name: String,
    value: Box<dyn Expression>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {
        todo!()
    }
}

struct Identifier {
    token: token::Token,
    value: String,
}
