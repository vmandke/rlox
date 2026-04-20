mod reader;
mod tokenize;
mod parser;
mod evaluate;
mod errors;

use crate::errors::LoxError;

fn runner() -> Result<(), LoxError> {
    let contents: reader::Source = reader::read_source()?;
    println!("{}", contents);
    Ok(())
}

fn main() {
    if let Err(e) = runner() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
