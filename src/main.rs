use clap::Parser;
use std::fs;

use crate::errors::Error;

mod errors;
mod lexer;
mod token;
mod parser;
mod compiler;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Whether to output debug information
    #[clap(short, long, value_parser)]
    debug: bool,

    #[clap()]
    input: String,
}

fn main() {
    let args = Args::parse();
    let contents = match fs::read_to_string(&args.input) {
        Ok(text) => text,
        Err(_) => {
            Error::throw(
                &args.input,
                &args.input,
                0,
                &format!("File '{}' was not found", args.input),
                false,
            );
            String::new()
        }
    };

    let lexer = lexer::lex(&contents);

    parser::parse(lexer);

    // println!("{:?}", lexer.next());
}
