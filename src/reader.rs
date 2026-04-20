use std::{env, fs};

pub type Source = String;

pub fn read_file(file_path: &str) -> Source {
    fs::read_to_string(file_path).expect("Something went wrong reading the file")
}

pub fn read_stdin() -> Source {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");
    input
}

pub fn read_source() -> Source {
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
            // incorrect usage
            std::process::exit(64);
        }
    }
}