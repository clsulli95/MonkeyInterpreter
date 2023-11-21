//! A Interpreter for the Monkey language
pub mod ast;
pub mod lexer;
pub mod repl;
pub mod token;

fn main() -> Result<(), anyhow::Error> {
    repl::start()?;
    Ok(())
}
