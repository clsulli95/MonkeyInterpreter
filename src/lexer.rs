use crate::token;

#[derive(Debug)]
pub struct Lexer {
    /// The input to the lexer. Can be a string or file
    input: Vec<char>,
    /// The current character the lexer is lexing
    char: char,
    /// The current position within the input that the lexer is lexing
    position: usize,
    /// The read position is current position + 1.  This is always looking ahead to
    /// catch two-character symbols
    read_position: usize,
    /// The end of file boolean.  Will be set to true once the lexer determines we are
    /// at the end of the file
    eof: bool,
}

impl Lexer {
    #[must_use]
    pub fn new(input: &str) -> Self {
        // Collect all characters into a vector
        let input_as_chars: Vec<char> = input.chars().collect();

        // Construct the lexer
        let mut me = Self {
            input: input_as_chars,
            char: '0',
            position: 0,
            read_position: 0,
            eof: false,
        };

        me.read_character();

        me
    }

    /// # Errors
    /// Something errs prolly
    pub fn next_token(&mut self) -> Result<token::Token, anyhow::Error> {
        if self.eof {
            Ok(token::Token::new(token::EOF, ""))
        } else {
            // Consume any whitespace leading up to next identifier
            self.eat_whitespace();

            // If current identifier matches for a two character symbol
            if self.is_two_character_symbol() {
                let symbol = self.consume_two_character_symbol();
                self.read_character();
                Ok(token::Token::try_from(symbol)?)
            // Otherwise, if identifier matches for a one character symbol
            } else if self.is_symbol() {
                let symbol = self.char.to_string();
                self.read_character();
                Ok(token::Token::try_from(symbol)?)
            // Otherwise, if identifier is alphabetic
            } else if self.char.is_alphabetic() {
                let symbol = self.read_identifier();
                Ok(token::Token::try_from(symbol)?)
            // Otherwise, if identifier is a number
            } else if self.char.is_ascii_digit() {
                let symbol = self.read_number()?;
                Ok(token::Token::try_from(symbol)?)
            // Otherwise, if EOF was discovered while identifing the token
            } else if self.eof {
                Ok(token::Token::new(token::EOF, ""))
            // Otherwise, the identifier is illegal
            } else {
                self.read_character();
                Ok(token::Token::new(token::ILLEGAL, &self.char.to_string()))
            }
        }
    }

    fn is_two_character_symbol(&self) -> bool {
        self.peek_character().map_or(false, |next_char| {
            let candidate_symbol: String = format!("{}{}", self.char, next_char);
            matches!(candidate_symbol.as_str(), "==" | "!=")
        })
    }

    fn consume_two_character_symbol(&mut self) -> String {
        let current_char = self.char;
        self.read_character();
        let next_char = self.char;
        let symbol = format!("{current_char}{next_char}");

        symbol
    }

    fn peek_character(&self) -> Result<char, anyhow::Error> {
        self.input
            .get(self.read_position)
            .map_or_else(|| Err(anyhow::anyhow!("Peek EOF")), |char| Ok(*char))
    }

    fn read_character(&mut self) {
        if let Some(char) = self.input.get(self.read_position) {
            self.position = self.read_position;
            self.read_position += 1;
            self.char = *char;
        } else {
            self.eof = true;
        }
    }

    fn eat_whitespace(&mut self) {
        while self.char.is_whitespace() && !self.eof {
            self.read_character();
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;

        while self.char.is_alphabetic() && !self.eof {
            self.read_character();
        }

        String::from_iter(&self.input[position..self.position])
    }

    fn read_number(&mut self) -> Result<i32, anyhow::Error> {
        let position = self.position;

        while self.char.is_ascii_digit() && !self.eof {
            self.read_character();
        }

        let number_string = String::from_iter(&self.input[position..self.position]);
        let number = number_string.parse::<i32>()?;

        Ok(number)
    }

    const fn is_symbol(&self) -> bool {
        matches!(
            self.char,
            '=' | ';' | '(' | ')' | ',' | '+' | '{' | '}' | '!' | '-' | '/' | '*' | '<' | '>'
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let mut lexer = Lexer::new(input);

        assert_eq!(
            token::Token::new(token::ASSIGN, "="),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::PLUS, "+"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::LPAREN, "("),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::RPAREN, ")"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::LBRACE, "{"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::RBRACE, "}"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::COMMA, ","),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::SEMICOLON, ";"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::EOF, ""),
            lexer.next_token().unwrap()
        );
    }

    #[test]
    fn test_next_token_easy_source_code() {
        let input = "let five = 5;";
        let mut lexer = Lexer::new(input);

        assert_eq!(
            token::Token::new(token::LET, "let"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::IDENT, "five"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::ASSIGN, "="),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::INT, "5"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::SEMICOLON, ";"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::EOF, ""),
            lexer.next_token().unwrap()
        );
    }

    #[test]
    fn test_next_token_source_code() {
        let input = r#"
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;

            if (5 < 10)  {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
        "#;

        let mut lexer = Lexer::new(input);

        assert_eq!(
            token::Token::new(token::LET, "let"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::IDENT, "five"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::ASSIGN, "="),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::INT, "5"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::SEMICOLON, ";"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::LET, "let"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::IDENT, "ten"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::ASSIGN, "="),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::INT, "10"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::SEMICOLON, ";"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::LET, "let"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::IDENT, "add"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::ASSIGN, "="),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::FUNCTION, "fn"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::LPAREN, "("),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::IDENT, "x"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::COMMA, ","),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::IDENT, "y"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::RPAREN, ")"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::LBRACE, "{"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::IDENT, "x"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::PLUS, "+"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::IDENT, "y"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::SEMICOLON, ";"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::RBRACE, "}"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::SEMICOLON, ";"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::LET, "let"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::IDENT, "result"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::ASSIGN, "="),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::IDENT, "add"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::LPAREN, "("),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::IDENT, "five"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::COMMA, ","),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::IDENT, "ten"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::RPAREN, ")"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::SEMICOLON, ";"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::BANG, "!"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::MINUS, "-"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::SLASH, "/"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::ASTERISK, "*"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::INT, "5"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::SEMICOLON, ";"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::INT, "5"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::LT, "<"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::INT, "10"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::GT, ">"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::INT, "5"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::SEMICOLON, ";"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::IF, "if"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::LPAREN, "("),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::INT, "5"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::LT, "<"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::INT, "10"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::RPAREN, ")"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::LBRACE, "{"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::RETURN, "return"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::TRUE, "true"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::SEMICOLON, ";"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::RBRACE, "}"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::ELSE, "else"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::LBRACE, "{"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::RETURN, "return"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::FALSE, "false"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::SEMICOLON, ";"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::RBRACE, "}"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::INT, "10"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::EQ, "=="),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::INT, "10"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::SEMICOLON, ";"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::INT, "10"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::NOT_EQ, "!="),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::INT, "9"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::SEMICOLON, ";"),
            lexer.next_token().unwrap()
        );
        assert_eq!(
            token::Token::new(token::EOF, ""),
            lexer.next_token().unwrap()
        );
    }
}
