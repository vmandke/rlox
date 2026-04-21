use std::{env, fs};

use crate::errors::LoxError;

pub struct Source {
    chars: std::iter::Peekable<std::vec::IntoIter<char>>,
}

impl Source {
    pub fn new(text: String) -> Self {
        
        Source {
            // Discussion on lifetimes: How many copies of text are created?
            // Should Source be consumed by tokenizer instead of being passed by mutable reference?
            // TODO (vin): Consider using a more efficient data structure for the source text.
            
            // Challenge: Can the tokenizer work off the original source, and all the collected lexemes be slices of the original source?             // text.chars().enumerate().peekable(); -> Would mean the text needs to be kept alive.
            chars: text.chars().collect::<Vec<_>>().into_iter().peekable(),
        }
    }
    pub fn advance(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub fn peek_char(&mut self) -> Option<char> {
        // TODO (vin): Can this be passed by reference instead of creating a copy??
        self.chars.peek().copied()
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
