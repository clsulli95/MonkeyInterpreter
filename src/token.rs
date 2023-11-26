#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Token {
    pub token_type: String,
    pub literal: String,
}

impl Token {
    #[must_use]
    pub fn new(token_type: &str, literal: &str) -> Self {
        Self {
            token_type: String::from(token_type),
            literal: String::from(literal),
        }
    }

    fn lookup(literal: &str) -> Self {
        match literal {
            "fn" => Self::new(FUNCTION, "fn"),
            "let" => Self::new(LET, "let"),
            "true" => Self::new(TRUE, "true"),
            "false" => Self::new(FALSE, "false"),
            "if" => Self::new(IF, "if"),
            "else" => Self::new(ELSE, "else"),
            "return" => Self::new(RETURN, "return"),
            "=" => Self::new(ASSIGN, "="),
            ";" => Self::new(SEMICOLON, ";"),
            "(" => Self::new(LPAREN, "("),
            ")" => Self::new(RPAREN, ")"),
            "," => Self::new(COMMA, ","),
            "+" => Self::new(PLUS, "+"),
            "{" => Self::new(LBRACE, "{"),
            "}" => Self::new(RBRACE, "}"),
            "!" => Self::new(BANG, "!"),
            "-" => Self::new(MINUS, "-"),
            "/" => Self::new(SLASH, "/"),
            "*" => Self::new(ASTERISK, "*"),
            "<" => Self::new(LT, "<"),
            ">" => Self::new(GT, ">"),
            "==" => Self::new(EQ, "=="),
            "!=" => Self::new(NOT_EQ, "!="),
            _ => Self::new(IDENT, literal),
        }
    }
}

impl TryFrom<String> for Token {
    type Error = anyhow::Error;

    fn try_from(input: String) -> Result<Self, anyhow::Error> {
        Ok(Self::lookup(&input))
    }
}

impl TryFrom<i32> for Token {
    type Error = anyhow::Error;

    fn try_from(input: i32) -> Result<Self, anyhow::Error> {
        Ok(Self::new(INT, &input.to_string()))
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
