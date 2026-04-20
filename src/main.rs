use std::fs;
use std::env;


mod reader;
mod tokenize;
mod parser;
mod evaluate;

fn main() {
    let contents: reader::Source = reader::read_source();
    println!("{}", contents);
}
