use clap::Parser;
use std::fs;

mod errors;
mod token;
mod util;

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
    //unsure if it has to be all caps, maybe experiment with it yourself :shrug:
    println!("{}", args.input);
    let contents = match fs::read_to_string(&args.input) {
        Ok(text) => text,
        Err(_) => {
            util::error(
                &args.input,
                "",
                "E0000",
                "File not found",
                0..0,
            );
            String::new()
        }
    };

    println!("{contents}");
}
