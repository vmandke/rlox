pub struct AST {
    // TODO (vin): Define the structure of the AST here.
}

use crate::{errors::LoxError, tokenize};

pub fn parse(tokens: Vec<tokenize::Token>) -> Result<AST, LoxError> {
    // TODO (vin): Implement the actual parsing logic here.
    // For now, just return an empty AST.
    Ok(AST {})
}
