use std::{env, fs};

use crate::errors::LoxError;

pub struct Source {
    text: String,
    pos: usize,
}

impl Source {
    pub fn new(text: String) -> Self {
        Source { text, pos: 0 }
    }

    pub fn advance(&mut self) -> Option<char> {
        let c = self.text[self.pos..].chars().next()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    pub fn peek_char(&self) -> Option<char> {
        self.text[self.pos..].chars().next()
    }
}

pub fn read_file(file_path: &str) -> Result<Source, LoxError> {
    fs::read_to_string(file_path)
        .map(Source::new)
        .map_err(|e| LoxError::ReaderIoError(e))
}

pub fn read_stdin() -> Result<Source, LoxError> {
    let mut input = String::new();
    // TODO (vin): Possibly keep reading until EOF instead of just one line?
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|e| LoxError::ReaderIoError(e))?;
    Ok(Source::new(input))
}

pub fn read_source() -> Result<Source, LoxError> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            println!("Starting the lox repl...");
            read_stdin()
        }
        2 => {
            let file_path = &args[1];
            read_file(file_path)
        }
        _ => {
            println!("Usage: rlox [optional script]");
            Err(LoxError::UsageError("Invalid number of arguments".into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_source() {
        let mut source = Source::new("hello".into());
        assert_eq!(source.advance(), Some('h'));
        assert_eq!(source.advance(), Some('e'));
        assert_eq!(source.advance(), Some('l'));
        assert_eq!(source.advance(), Some('l'));
        assert_eq!(source.advance(), Some('o'));
        assert_eq!(source.advance(), None);
    }

    #[test]
    fn test_peek_source() {
        let mut source = Source::new("world".into());
        assert_eq!(source.peek_char(), Some('w'));
        assert_eq!(source.advance(), Some('w'));
        assert_eq!(source.peek_char(), Some('o'));
        assert_eq!(source.advance(), Some('o'));
        assert_eq!(source.peek_char(), Some('r'));
        assert_eq!(source.advance(), Some('r'));
        assert_eq!(source.peek_char(), Some('l'));
        assert_eq!(source.advance(), Some('l'));
        assert_eq!(source.peek_char(), Some('d'));
        assert_eq!(source.advance(), Some('d'));
        assert_eq!(source.peek_char(), None);
        assert_eq!(source.advance(), None);
    }
}
