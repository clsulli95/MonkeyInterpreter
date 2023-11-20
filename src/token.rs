use anyhow::anyhow;

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: String,
    literal: String,
}

impl Token {
    pub fn new(token_type: &str, literal: &str) -> Token {
        println!("Creating<{}, {}>", token_type, literal);
        Token {
            token_type: String::from(token_type),
            literal: String::from(literal),
        }
    }

    fn lookup(literal: &str) -> Result<Token, anyhow::Error> {
        println!("Looking up: {}", literal);

        match literal {
            "fn" => Ok(Token::new(FUNCTION, "fn")),
            "let" => Ok(Token::new(LET, "let")),
            "true" => Ok(Token::new(TRUE, "true")),
            "false" => Ok(Token::new(FALSE, "false")),
            "if" => Ok(Token::new(IF, "if")),
            "else" => Ok(Token::new(ELSE, "else")),
            "return" => Ok(Token::new(RETURN, "return")),
            "=" => Ok(Token::new(ASSIGN, "=")),
            ";" => Ok(Token::new(SEMICOLON, ";")),
            "(" => Ok(Token::new(LPAREN, "(")),
            ")" => Ok(Token::new(RPAREN, ")")),
            "," => Ok(Token::new(COMMA, ",")),
            "+" => Ok(Token::new(PLUS, "+")),
            "{" => Ok(Token::new(LBRACE, "{")),
            "}" => Ok(Token::new(RBRACE, "}")),
            "!" => Ok(Token::new(BANG, "!")),
            "-" => Ok(Token::new(MINUS, "-")),
            "/" => Ok(Token::new(SLASH, "/")),
            "*" => Ok(Token::new(ASTERISK, "*")),
            "<" => Ok(Token::new(LT, "<")),
            ">" => Ok(Token::new(GT, ">")),
            "==" => Ok(Token::new(EQ, "==")),
            "!=" => Ok(Token::new(NOT_EQ, "!=")),
            _ => Ok(Token::new(IDENT, literal)),
        }
    }
}

impl TryFrom<String> for Token {
    type Error = anyhow::Error;

    fn try_from(input: String) -> Result<Self, anyhow::Error> {
        Ok(Token::lookup(&input)?)
    }
}

impl TryFrom<i32> for Token {
    type Error = anyhow::Error;

    fn try_from(input: i32) -> Result<Self, anyhow::Error> {
        Ok(Token::new(INT, &input.to_string())) 
    }
}

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";

// Identifiers and literals
pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";

// Operators
pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const BANG: &str = "!";
pub const ASTERISK: &str = "*";
pub const SLASH: &str = "/";
pub const LT: &str = "<";
pub const GT: &str = ">";
pub const EQ: &str = "==";
pub const NOT_EQ: &str = "!=";

// Delimiters
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";

pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";

// Keywords
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";
pub const TRUE: &str = "TRUE";
pub const FALSE: &str = "FALSE";
pub const IF: &str = "IF";
pub const ELSE: &str = "ELSE";
pub const RETURN: &str = "RETURN";
