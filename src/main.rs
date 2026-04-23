mod errors;
mod evaluate;
mod grammar;
mod parser;
mod reader;
mod state;
mod tokenize;

use crate::errors::LoxError;
use crate::state::Environment;
use std::cell::RefCell;
use std::rc::Rc;

fn runner() -> Result<(), LoxError> {
    let mut source: reader::Source = reader::read_source()?;
    let tokens = tokenize::scan(&mut source)?;
    let stmts = parser::parse(tokens)?;
    let env = Rc::new(RefCell::new(Environment::new()));
    for stmt in &stmts {
        evaluate::evaluate(stmt, Rc::clone(&env))?;
    }
    Ok(())
}

fn main() {
    if let Err(e) = runner() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
