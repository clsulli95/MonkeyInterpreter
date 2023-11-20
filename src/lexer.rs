use crate::token;

#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    char: char,
    position: usize,
    read_position: usize,
    eof: bool,
}

impl Lexer {
    pub fn new(input: &str) -> Result<Self, anyhow::Error> {
        // Then collect all characters into a vector
        let input: Vec<char> = input.chars().collect();
       
        // Construct the lexer
        let mut me = Self {
            input,
            char: '0',
            position: 0,
            read_position: 0,
            eof: false,
        };

        me.read_character();

        Ok(me)
    }

    pub fn next_token(&mut self) -> Result<token::Token, anyhow::Error> {
        if !self.eof {
            println!("Processing char: {}", self.char.to_string());
            // Consume any whitespace leading up to next identifier
            self.eat_whitespace()?;

            // If current identifier matches for a two character symbol
            if self.is_two_character_symbol()? {
                let symbol = self.consume_two_character_symbol()?;
                self.read_character();
                println!("Token found: {}", symbol);
                Ok(token::Token::try_from(symbol)?)
            // Otherwise, if identifier matches for a one character symbol
            } else if self.is_symbol() {
                let symbol = self.char.to_string();
                self.read_character();
                println!("Token found: {}", symbol);
                Ok(token::Token::try_from(symbol)?)
            // Otherwise, if identifier is alphabetic
            } else if self.char.is_alphabetic() {
                let symbol = self.read_identifier()?;
                println!("Token found: {}", symbol);
                Ok(token::Token::try_from(symbol)?)
            // Otherwise, if identifier is a number
            } else if self.char.is_digit(10) {
                let symbol = self.read_number()?;
                println!("Token found: {}", symbol);
                Ok(token::Token::try_from(symbol)?)
            // Otherwise, the identifier is illegal
            } else {
                self.read_character();
                Ok(token::Token::new(token::ILLEGAL, &self.char.to_string()))
            }
        } else {
            Ok(token::Token::new(token::EOF, ""))
        }
    }

    fn is_two_character_symbol(&self) -> Result<bool, anyhow::Error>{
        if let Ok(next_char) = self.peek_character() {
            let candidate_symbol : String = format!("{}{}", self.char.to_string(), next_char.to_string());

            println!("Candidate Symbol: {}", candidate_symbol);

            match candidate_symbol.as_str() {
                "==" => Ok(true),
                "!=" => Ok(true),
                _ => Ok(false),
            }
        } else {
            Ok(false) 
        }
    }

    fn consume_two_character_symbol(&mut self) -> Result<String, anyhow::Error> {
        let current_char = self.char;
        self.read_character();
        let next_char = self.char;
        let symbol = format!("{}{}", current_char, next_char);

        Ok(symbol)
    }

    fn peek_character(&self) -> Result<char, anyhow::Error> {
        if let Some(char) = self.input.get(self.read_position) {
            Ok(char.clone())
        } else {
            Err(anyhow::anyhow!("peek EOF"))
        }
    }

    fn read_character(&mut self) {
        if let Some(char) = self.input.get(self.read_position) {
            self.position = self.read_position;
            self.read_position += 1;
            self.char = char.clone();
        } else {
            self.eof = true;
        }
    }

    fn eat_whitespace(&mut self) -> Result<(), anyhow::Error>{
        while self.char.is_whitespace() {
            self.read_character();
        }

        Ok(())
    }

    fn read_identifier(&mut self) -> Result<String, anyhow::Error> {
        let position = self.position;

        while self.char.is_alphabetic() {
            self.read_character();
        }

        let identifier = String::from_iter(&self.input[position..self.position]);
        
        Ok(identifier)
    }

    fn read_number(&mut self) -> Result<i32, anyhow::Error> {
        let position = self.position;

        while self.char.is_digit(10) {
            self.read_character();
        }

        let number_string = String::from_iter(&self.input[position..self.position]);
        let number = number_string.parse::<i32>()?;

        Ok(number)
    }

    fn is_symbol(&self) -> bool {
        match self.char {
            '=' => true,
            ';' => true,
            '(' => true,
            ')' => true,
            ',' => true,
            '+' => true,
            '{' => true,
            '}' => true,
            '!' => true,
            '-' => true,
            '/' => true,
            '*' => true,
            '<' => true,
            '>' => true,
            _ => false,
        }
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let mut lexer = Lexer::new(input).unwrap();

        assert_eq!(token::Token::new(token::ASSIGN, "="), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::PLUS, "+"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::LPAREN, "("), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::RPAREN, ")"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::LBRACE, "{"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::RBRACE, "}"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::COMMA, ","), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::SEMICOLON, ";"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::EOF, ""), lexer.next_token().unwrap());
    }

    #[test]
    fn test_next_token_easy_source_code() {
        let input = "let five = 5;";
        let mut lexer = Lexer::new(input).unwrap();


        assert_eq!(token::Token::new(token::LET, "let"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::IDENT, "five"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::ASSIGN, "="), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::INT, "5"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::SEMICOLON, ";"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::EOF, ""), lexer.next_token().unwrap());
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

        let mut lexer = Lexer::new(input).unwrap();

		assert_eq!(token::Token::new(token::LET, "let"), lexer.next_token().unwrap());
		assert_eq!(token::Token::new(token::IDENT, "five"), lexer.next_token().unwrap());
		assert_eq!(token::Token::new(token::ASSIGN, "="), lexer.next_token().unwrap());
		assert_eq!(token::Token::new(token::INT, "5"), lexer.next_token().unwrap());
		assert_eq!(token::Token::new(token::SEMICOLON, ";"), lexer.next_token().unwrap()); 
		assert_eq!(token::Token::new(token::LET, "let"), lexer.next_token().unwrap()); 
		assert_eq!(token::Token::new(token::IDENT, "ten"), lexer.next_token().unwrap()); 
		assert_eq!(token::Token::new(token::ASSIGN, "="), lexer.next_token().unwrap()); 
		assert_eq!(token::Token::new(token::INT, "10"), lexer.next_token().unwrap()); 
		assert_eq!(token::Token::new(token::SEMICOLON, ";"), lexer.next_token().unwrap()); 
        assert_eq!(token::Token::new(token::LET, "let"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::IDENT, "add"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::ASSIGN, "="), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::FUNCTION, "fn"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::LPAREN, "("), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::IDENT, "x"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::COMMA, ","), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::IDENT, "y"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::RPAREN, ")"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::LBRACE, "{"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::IDENT, "x"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::PLUS, "+"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::IDENT, "y"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::SEMICOLON, ";"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::RBRACE, "}"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::SEMICOLON, ";"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::LET, "let"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::IDENT, "result"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::ASSIGN, "="), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::IDENT, "add"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::LPAREN, "("), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::IDENT, "five"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::COMMA, ","), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::IDENT, "ten"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::RPAREN, ")"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::SEMICOLON, ";"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::BANG, "!"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::MINUS, "-"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::SLASH, "/"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::ASTERISK, "*"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::INT, "5"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::SEMICOLON, ";"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::INT, "5"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::LT, "<"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::INT, "10"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::GT, ">"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::INT, "5"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::SEMICOLON, ";"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::IF, "if"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::LPAREN, "("), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::INT, "5"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::LT, "<"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::INT, "10"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::RPAREN, ")"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::LBRACE, "{"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::RETURN, "return"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::TRUE, "true"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::SEMICOLON, ";"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::RBRACE, "}"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::ELSE, "else"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::LBRACE, "{"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::RETURN, "return"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::FALSE, "false"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::SEMICOLON, ";"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::RBRACE, "}"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::INT, "10"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::EQ, "=="), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::INT, "10"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::SEMICOLON, ";"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::INT, "10"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::NOT_EQ, "!="), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::INT, "9"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::SEMICOLON, ";"), lexer.next_token().unwrap());
        assert_eq!(token::Token::new(token::EOF, ""), lexer.next_token().unwrap());
    }
}
