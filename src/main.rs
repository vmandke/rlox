mod errors;
mod evaluate;
mod grammar;
mod parser;
mod reader;
mod state;
mod tokenize;

use crate::errors::LoxError;

fn runner() -> Result<(), LoxError> {
    let mut source: reader::Source = reader::read_source()?;
    let tokens = tokenize::scan(&mut source)?;
    let stmts = parser::parse(tokens)?;
    let mut env = state::Environment::new();
    for stmt in &stmts {
        evaluate::evaluate(stmt, &mut env)?;
    }
    Ok(())
}

fn main() {
    if let Err(e) = runner() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
