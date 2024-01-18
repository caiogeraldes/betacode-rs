use betacode::{converter, validator};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Runs on a file
    #[clap(short, long, action)]
    pub file: bool,
    /// Applies more strict conversion rules
    #[clap(short, long, action)]
    pub strict: bool,

    /// String to be converted (if -f is flagged, it is the name of the file)
    pub text: Option<String>,

    /// Output file for conversion (default is stdout)
    #[clap(short, long, action)]
    pub output: Option<String>,

    #[clap(short, long, action)]
    pub inverse: bool,
}

fn convert_line(input: String) -> Result<String, validator::ValidationError> {
    match validator::validate(&input) {
        Ok(()) => Ok(converter::convert(input)),
        Err(e) => match e {
            validator::ValidationError::InvalidDiacriticOrder(_) => Ok(converter::convert(input)),
            validator::ValidationError::MixedCaseNotation => {
                log::warn!("Mixed case notation used, may contain errors.");
                Ok(converter::convert(input))
            }
            _ => Err(e),
        },
    }
}
fn revert_line(input: String) -> String {
    converter::revert(input)
}

fn convert_line_strict(input: String) -> Result<String, validator::ValidationError> {
    match validator::validate(&input) {
        Ok(()) => Ok(converter::convert(input)),
        Err(e) => Err(e),
    }
}

fn read_file(input: PathBuf) -> Result<String, std::io::Error> {
    let file: String = fs::read_to_string(input)?;
    Ok(file)
}

fn main() -> Result<(), validator::ValidationError> {
    pretty_env_logger::init();
    let args = Args::parse();

    let input_str: Option<String> = match args.file {
        true => match args.text {
            Some(string) => {
                let path = PathBuf::from(string);
                match read_file(path) {
                    Ok(string) => Some(string),
                    Err(e) => {
                        eprintln!("{e}");
                        std::process::exit(1)
                    }
                }
            }
            None => None,
        },
        false => args.text,
    };

    match args.inverse {
        true => match input_str {
            None => {
                eprintln! {"Empty string"};
                std::process::exit(1)
            }
            Some(input_str) => {
                let string = revert_line(input_str);
                match args.output {
                    None => {
                        println!("{}", revert_line(string));
                        Ok(())
                    }
                    Some(path) => match fs::write(PathBuf::from(path), string) {
                        Ok(_) => Ok(()),
                        Err(e) => {
                            eprintln!("{e}");
                            std::process::exit(1)
                        }
                    },
                }
            }
        },

        false => match args.strict {
            true => match input_str {
                None => {
                    eprintln! {"Empty string"};
                    std::process::exit(1)
                }
                Some(input_str) => match convert_line_strict(input_str) {
                    Ok(string) => {
                        println!("{string}");
                        Ok(())
                    }
                    Err(e) => match e {
                        validator::ValidationError::NotASCII(_) => {
                            eprintln!("Text passed is not in ASCII.\n{e}");
                            std::process::exit(1)
                        }
                        validator::ValidationError::InvalidChars(_) => {
                            eprintln!(
                            "Text passed violates ASCII Betacode standards as applied here.\n{e}"
                        );
                            std::process::exit(1)
                        }
                        validator::ValidationError::InvalidDiacriticOrder(_) => {
                            eprintln!(
                            "Text passed violates ASCII Betacode standards as applied here.\n{e}"
                        );
                            std::process::exit(1)
                        }
                        validator::ValidationError::MixedCaseNotation => {
                            eprintln!("Text passed contains both * and upper case ASCII for Greek Upper notation.\n{e}");
                            std::process::exit(1)
                        }
                    },
                },
            },
            false => match input_str {
                None => {
                    eprintln! {"Empty string"};
                    std::process::exit(1)
                }
                Some(input_str) => match convert_line(input_str) {
                    Ok(string) => match args.output {
                        None => {
                            println!("{string}");
                            Ok(())
                        }
                        Some(path) => match fs::write(PathBuf::from(path), string) {
                            Ok(_) => Ok(()),
                            Err(e) => {
                                eprintln!("{e}");
                                std::process::exit(1)
                            }
                        },
                    },
                    Err(e) => match e {
                        validator::ValidationError::NotASCII(_) => {
                            eprintln!("Text passed is not in ASCII.\n{e}");
                            std::process::exit(1)
                        }
                        validator::ValidationError::InvalidChars(_) => {
                            eprintln!(
                            "Text passed violates ASCII Betacode standards as applied here.\n{e}"
                        );
                            std::process::exit(1)
                        }
                        _ => Ok(()),
                    },
                },
            },
        },
    }
}
