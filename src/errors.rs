
use std::fmt;

#[derive(Debug)]
pub enum LoxError {
    ReaderIoError(std::io::Error),
    UsageError(String),
    ScanError { line: usize, col: usize, message: String },
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxError::ReaderIoError(e) => write!(f, "Reader IO error: {e}"),
            LoxError::UsageError(msg) => write!(f, "Usage error: {msg}"),
            LoxError::ScanError { line, col, message } => write!(f, "[line {line}, column {col}] Scan error: {message}"),
        }
    }
}

impl std::error::Error for LoxError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LoxError::ReaderIoError(e) => Some(e),
            LoxError::UsageError(_) => None,
            LoxError::ScanError { .. } => None,
        }
    }
}