pub struct Token {
    // TODO (vin): Add attributes like token type, lexeme, line number, etc.
    text: String,
}

use crate::{errors::LoxError, reader};

pub fn scan(source: reader::Source) -> Result<Vec<Token>, LoxError> {
    // TODO (vin): Implement the actual scanning logic here.
    // For now, just return an empty vector of tokens.
    Ok(vec![])
}