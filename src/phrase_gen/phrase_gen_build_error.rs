use super::PhraseGen;
use rand::prelude::IndexedRandom;
use std::collections::HashMap;

pub const EXAMPLE_STR: &str = include_str!("./examples/static.toml");

/// Possible errors when creating a `PhraseGen`
#[derive(Debug)]
pub enum PhraseGenBuildError {
    MissingField(String),
    WrongFieldType(String),
}

impl std::fmt::Display for PhraseGenBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PhraseGenBuildError::MissingField(field) => format!("'{field}' is missing"),
                PhraseGenBuildError::WrongFieldType(field) =>
                    format!("'{field}' doesn't have the expected type"),
            }
        )
    }
}

impl std::error::Error for PhraseGenBuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl Default for PhraseGen {
    /// Creates a new empty Dictionary.
    fn default() -> Self {
        PhraseGen {
            map: HashMap::new(),
            format: "".to_owned(),
        }
    }
}

impl PhraseGen {
    /// Returns a copy of the format string with the category keys replaced with one of its values.
    /// Returns Some if all of the fields used for generation contain at least one word, otherwire returns None.
    pub fn get_phrase(&self) -> Option<String> {
        // initialize rng
        let mut rng = rand::rng(); // FIXME: should do in struct initialization?

        // initialize return value
        let mut out = self.format.clone();

        // go through every word class provided
        for (part_of_speech, words) in &self.map {
            // create string to search for
            let key = format!("{{{{{part_of_speech}}}}}"); // {{ → {. damn you, escape sequences!!

            // substitute all occurences of the search string
            while out.contains(&key) {
                // FIXME: if a substituted value contains a searched string it will be replaced! Maybe use regex matches?
                let word = words.choose(&mut rng)?;
                out = out.replacen(&key, word, 1);
            }
        }

        Some(out)
    }
}
