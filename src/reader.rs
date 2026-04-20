use std::{env, fs};

pub struct Source {
    text: String,
}

use crate::errors::LoxError;


pub fn read_file(file_path: &str) -> Result<Source, LoxError> {
    fs::read_to_string(file_path)
        .map(|buf| Source { text: buf })
        .map_err(|e| LoxError::ReaderIoError(e))
}

pub fn read_stdin() -> Result<Source, LoxError> {
    let mut input = String::new();
    // TODO (vin): Possibly keep reading until EOF instead of just one line?
    // Q: How to pass the collected input to the runner (code structure) ?
    std::io::stdin().read_line(&mut input).map_err(|e| LoxError::ReaderIoError(e))?;
    Ok(Source { text: input })
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
            // raise an error here since this is an invalid usage of the program
            Err(LoxError::UsageError("Invalid number of arguments".into()))
        }
    }
}