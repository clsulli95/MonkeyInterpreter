use crate::ast;
use crate::lexer;
use crate::token;
use std::collections::hash_map::HashMap;

pub trait Prefix {
    fn parse(&self) -> Result<ast::Expression, anyhow::Error>;
}

pub trait Infix {
    fn parse(&self, expression: ast::Expression) -> Result<ast::Expression, anyhow::Error>;
}

pub struct Parser {
    lexer: lexer::Lexer,
    current_token: token::Token,
    peek_token: token::Token,
    pub errors: Vec<String>,
    prefix_parse_fns: HashMap<String, Box<dyn Prefix>>,
    infix_parse_fns: HashMap<String, Box<dyn Infix>>,
}

impl Parser {
    pub fn new(lexer: lexer::Lexer) -> Result<Self, anyhow::Error> {
        let mut me = Self {
            lexer,
            current_token: token::Token::new(token::ILLEGAL, ""),
            peek_token: token::Token::new(token::ILLEGAL, ""),
            errors: Vec::default(),
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        me.next_token()?;
        me.next_token()?;

        Ok(me)
    }

    pub fn register_prefix(&mut self, token_type: &str, prefix_fn: Box<dyn Prefix>) {
        self.prefix_parse_fns
            .insert(token_type.to_owned(), prefix_fn);
    }

    pub fn register_infix(&mut self, token_type: &str, infix_fn: Box<dyn Infix>) {
        self.infix_parse_fns.insert(token_type.to_owned(), infix_fn);
    }

    pub fn next_token(&mut self) -> Result<(), anyhow::Error> {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token()?;

        Ok(())
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, anyhow::Error> {
        let mut program = ast::Program::default();

        while self.current_token.token_type != token::EOF {
            let statement = self.parse_statement()?;
            program.statements.push(statement);

            self.next_token()?;
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<ast::Statement, anyhow::Error> {
        match self.current_token.token_type.as_str() {
            token::LET => {
                return self.parse_let_statement();
            }
            token::RETURN => {
                return self.parse_return_statement();
            }
            _ => {
                return Err(anyhow::anyhow!("Error could not parse statement!"));
            }
        }
    }

    fn parse_let_statement(&mut self) -> Result<ast::Statement, anyhow::Error> {
        if !self.expect_peek(token::IDENT)? {
            self.errors.push("expected identifier".to_owned());
            return Err(anyhow::anyhow!("Expected Identifier"));
        }

        let ident = ast::Identifier::new(self.current_token.clone(), &self.current_token.literal);

        if !self.expect_peek(token::ASSIGN)? {
            self.errors.push("expected assign".to_owned());
            return Err(anyhow::anyhow!("Expected Assign"));
        }

        while !self.current_token_is(token::SEMICOLON) {
            self.next_token()?;
        }

        // TODO parse expression
        let expression = ast::Expression::Integer(token::Token::new(token::INT, "42"));

        return Ok(ast::Statement::Let(ident, expression));
    }

    fn parse_return_statement(&mut self) -> Result<ast::Statement, anyhow::Error> {
        // TODO parse expression
        let expression = ast::Expression::Integer(token::Token::new(token::INT, "42"));

        while !self.current_token_is(token::SEMICOLON) {
            self.next_token()?;
        }

        return Ok(ast::Statement::Return(expression));
    }

    fn expect_peek(&mut self, token_type: &str) -> Result<bool, anyhow::Error> {
        if self.peek_token_is(token_type) {
            self.next_token()?;
            return Ok(true);
        } else {
            self.errors.push(format!(
                "expected next token to be {}, got {} instead",
                token_type, self.peek_token.token_type
            ));
            return Ok(false);
        }
    }

    fn current_token_is(&self, token_type: &str) -> bool {
        return self.current_token.token_type == token_type;
    }

    fn peek_token_is(&self, token_type: &str) -> bool {
        return self.peek_token.token_type == token_type;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verify_no_parser_errors(parser: Parser) -> bool {
        if parser.errors.len() == 0 {
            return true;
        }

        for err in parser.errors {
            println!("{}", err);
        }

        return false;
    }

    fn create_expected_let_statement(id_name: &str, exp_val: &str) -> ast::Statement {
        let expected_token = token::Token::new(token::IDENT, id_name);
        let expected_ident = ast::Identifier::new(expected_token, id_name);
        let expected_expression = ast::Expression::Integer(token::Token::new(token::INT, exp_val));
        let expected_statement = ast::Statement::Let(expected_ident, expected_expression);
        return expected_statement;
    }

    fn create_expected_expression_statement(
        token_type: &str,
        literal: &str,
        name: &str,
    ) -> ast::Statement {
        let expected_token = token::Token::new(token_type, literal);
        let expected_expression =
            ast::Expression::Identifier(token::Token::new(token::IDENT, name));
        return ast::Statement::Expression(expected_token, expected_expression);
    }

    #[test]
    fn test_let_statements() {
        let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "#;

        let lexer = lexer::Lexer::new(input);
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse_program().unwrap();

        assert_eq!(3, program.statements.len());
        assert_eq!(
            program.statements.get(0).unwrap(),
            &create_expected_let_statement("x", "42")
        );
        assert_eq!(
            program.statements.get(1).unwrap(),
            &create_expected_let_statement("y", "42")
        );
        assert_eq!(
            program.statements.get(2).unwrap(),
            &create_expected_let_statement("foobar", "42")
        );
        assert_eq!(true, verify_no_parser_errors(parser));
        println!("{}", program.to_string());
    }

    #[test]
    fn test_let_statements_error() {
        let input = r#"
        let x = 5;
        let y = 10;
        let 838383;
        "#;

        let lexer = lexer::Lexer::new(input);
        let mut parser = Parser::new(lexer).unwrap();
        let _ = parser.parse_program().unwrap_err();

        assert_eq!(false, verify_no_parser_errors(parser));
    }

    #[test]
    fn test_return_statements() {
        let input = r#"
        return 5;
        return 10;
        return 993322;
        "#;

        let lexer = lexer::Lexer::new(input);
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse_program().unwrap();

        assert_eq!(3, program.statements.len());
        assert!(verify_no_parser_errors(parser));
        assert!(matches!(
            program.statements.get(0).unwrap(),
            ast::Statement::Return { .. }
        ));
        assert!(matches!(
            program.statements.get(1).unwrap(),
            ast::Statement::Return { .. }
        ));
        assert!(matches!(
            program.statements.get(2).unwrap(),
            ast::Statement::Return { .. }
        ));

        println!("{}", program.to_string());
    }
}
