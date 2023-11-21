use crate::lexer;
use crate::token;
use std::io::Write;

pub fn start() -> Result<(), anyhow::Error> {
    loop {
        // Grab user input
        print!(">> ");
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let mut lexer = lexer::Lexer::new(&input);

        loop {
            let token = lexer.next_token()?;

            match token.token_type.as_str() {
                token::ILLEGAL => {
                    println!("Illegal! {:?}", token);
                }
                token::EOF => {
                    break;
                }
                _ => {
                    println!("Token: {:?}", token);
                }
            }
        }
    }
}
