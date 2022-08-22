use betacode::{converter, validator};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(string) => match validator::validate(string) {
            Ok(()) => {
                let result = converter::convert(string.to_string());
                println!("{result}");
            }
            Err(e) => match e {
                validator::ValidationError::NotASCII(_) => {
                    println!("Text passed is not in ASCII.");
                    eprintln!("{e}")
                }
                validator::ValidationError::InvalidChars(_) => {
                    println!("Text passed violates ASCII Betacode standards as applied here.");
                    eprintln!("{e}")
                }
                validator::ValidationError::InvalidDiacriticOrder(_) => {
                    let result = converter::convert(string.to_string());
                    println!("{result}");
                }
            },
        },
        None => eprintln!("No argument passed"),
    };
}
