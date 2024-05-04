//! > **CLI random phrase generator**
//!
//! Epigram generates random phrases from a pool of categorized words.
//! Run `epigram --help` for CLI syntax.

use clap::Parser;
use clio::{self, Input};

use std::{self, process::exit};

pub mod dictionary;
pub mod part_of_speech;
#[cfg(test)]
mod tests;

use dictionary::{Dictionary, DictionaryError};

const BUILT_IN_DICTIONARY_STR: &str = include_str!("./dictionary.ini");

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to ini-like file with categorized words
    #[arg(value_parser)]
    dictionary_file: Option<Input>,

    /// Number of phrases to generate
    #[arg(short = 'n', long, default_value_t = 1, group = "len")]
    count: u128,

    /// Generate phrases indefinetly
    #[arg(short, long, group = "len")]
    infinite: bool,
}

fn main() -> Result<(), u8> {
    let Args {
        dictionary_file: input,
        count,
        infinite,
    } = Args::parse(); // get CLI arguments

    let dict_string = if let Some(mut input) = input {
        if input.is_tty() {
            // display message when getting string from user input
            if cfg!(unix) {
                eprintln!("Reading ini dictionary from stdin, close with ^D (EOF)...")
            } else if cfg!(windows) {
                eprintln!("Reading ini dictionary from input, close with Ctrl+Z (EOF)...")
            }
        }

        let mut buf = String::new();
        let e = input.lock().read_to_string(&mut buf);
        match e {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Could not decode file: {e}");
                exit(2);
            }
        }
        buf
    } else {
        // use built-in dictionary when a file path is not supplied
        BUILT_IN_DICTIONARY_STR.to_string()
    };

    let dict = match Dictionary::try_from(dict_string) {
        Ok(dict) => dict,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    fn print_phrase(dict: &Dictionary) {
        println!(
            "{}",
            dict.get_phrase()
                .expect("all dictionary parts of speech should be filled.")
        )
    }

    if infinite {
        loop {
            print_phrase(&dict)
        }
    } else {
        for _ in 0..count {
            print_phrase(&dict)
        }
    }

    Ok(())
}
