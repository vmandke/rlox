use std::fmt;

#[derive(Debug)]
pub enum LoxError {
    ReaderIoError(std::io::Error),
    UsageError(String),
    ScanError {
        line: usize,
        col: usize,
        message: String,
    },
    ParserErrorCannotConsumeExpectedType {
        expected_token_type: String,
    },
    ParserErrorAdvanceExpectedAToken(String),
    ParserErrorExpressionExpected(String),
    InterpretUnaryMinusUndefined(String),
    InterpretUnaryNotUndefined(String),
    InterpretBinaryOpUndefined(String),
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxError::ReaderIoError(e) => write!(f, "Reader IO error: {e}"),
            LoxError::UsageError(msg) => write!(f, "Usage error: {msg}"),
            LoxError::ScanError { line, col, message } => {
                write!(f, "[line {line}, column {col}] Scan error: {message}")
            }
            LoxError::ParserErrorCannotConsumeExpectedType {
                expected_token_type,
            } => {
                write!(
                    f,
                    "Parser: Expected, but not found, cannot consume {expected_token_type}"
                )
            }
            LoxError::ParserErrorAdvanceExpectedAToken(msg) => write!(f, "Parser error: {msg}"),
            LoxError::ParserErrorExpressionExpected(msg) => write!(f, "Parser error: {msg}"),
            LoxError::InterpretUnaryMinusUndefined(msg) => write!(f, "Interpret error: {msg}"),
            LoxError::InterpretUnaryNotUndefined(msg) => write!(f, "Interpret error: {msg}"),
            LoxError::InterpretBinaryOpUndefined(msg) => write!(f, "Interpret error: {msg}"),
        }
    }
}

impl std::error::Error for LoxError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LoxError::ReaderIoError(e) => Some(e),
            LoxError::UsageError(_) => None,
            LoxError::ScanError { .. } => None,
            LoxError::ParserErrorCannotConsumeExpectedType { .. } => None,
            LoxError::ParserErrorAdvanceExpectedAToken(_) => None,
            LoxError::ParserErrorExpressionExpected(_) => None,
            LoxError::InterpretUnaryMinusUndefined(_) => None,
            LoxError::InterpretUnaryNotUndefined(_) => None,
            LoxError::InterpretBinaryOpUndefined(_) => None,
        }
    }
}
