//! > **CLI random phrase generator**
//!
//! Epigram generates random phrases from a pool of categorized words.
//! Run `epigram --help` for CLI syntax.

use clap::Parser;
use clio::{self, Input};
use phrase_gen::static_phrase_gen::EXAMPLE_STR as STATIC_PHRASE_GEN_EXAMPLE_STR;
use phrase_gen::PhraseGen;

use std::{self, process::exit};

pub mod phrase_gen;

#[cfg(test)]
mod tests;

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

fn main() {
    let Args {
        dictionary_file: input,
        count,
        infinite,
    } = Args::parse(); // get CLI arguments

    let pg_definition = if let Some(mut input) = input {
        if input.is_tty() {
            // display message when getting string from user input
            if cfg!(unix) {
                eprintln!("Reading toml definitions from stdin, close with ^D (EOF)...")
            } else if cfg!(windows) {
                eprintln!("Reading toml definitions from input, close with Ctrl+Z (EOF)...")
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
        // use built-in example when a file path is not supplied
        STATIC_PHRASE_GEN_EXAMPLE_STR.to_string()
    };

    // turn into toml table
    let pg_definition = match pg_definition.parse::<toml::Table>() {
        Ok(pg_table) => pg_table,
        Err(e) => {
            eprintln!("Failed to process toml: {e}");
            exit(3)
        }
    };

    // instantiate PhraseGen
    let pg = match phrase_gen::phrase_gen_from_toml(pg_definition) {
        Ok(dict) => dict,
        Err(e) => {
            eprintln!("Failed to initialise PhraseGen: {}", e);
            exit(1);
        }
    };

    fn print_phrase(pg: &dyn PhraseGen) {
        println!(
            "{}",
            pg.get_phrase().expect("PhraseGen should generate a phrase")
        )
    }

    if infinite {
        loop {
            print_phrase(pg.as_ref())
        }
    } else {
        for _ in 0..count {
            print_phrase(pg.as_ref())
        }
    }

    exit(0);
}
