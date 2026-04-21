mod errors;
mod evaluate;
mod parser;
mod reader;
mod tokenize;

use crate::errors::LoxError;

fn runner() -> Result<(), LoxError> {
    let mut source: reader::Source = reader::read_source()?;
    let tokens = tokenize::scan(&mut source)?;
    let ast = parser::parse(tokens)?;
    let result = evaluate::evaluate(ast)?;
    println!("{}", result);
    Ok(())
}

fn main() {
    if let Err(e) = runner() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
