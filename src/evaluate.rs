use std::fmt;

pub struct EvalResult {
    // TODO (vin): Define the structure of the evaluation result here.
}

impl fmt::Display for EvalResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

use crate::{errors::LoxError, parser};

pub fn evaluate(ast: parser::AST) -> Result<EvalResult, LoxError> {
    // TODO (vin): Implement the actual evaluation logic here.
    // For now, just return an empty result.
    Ok(EvalResult {})
}