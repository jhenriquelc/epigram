use super::PhraseGen;
use rand::prelude::IndexedRandom;
use std::collections::HashMap;
use toml;

pub const EXAMPLE_STR: &str = include_str!("./examples/static.toml");

/// Possible errors when creating a `StaticPhraseGen`
#[derive(Debug)]
pub enum StaticPhraseGenError {
    MissingConfigTable,
    MissingConfigTypeHeader,
    InvalidConfigTypeHeader,
    MissingFormatKey,
    MissingFormatString,
    MissingClassesTable,
    InvalidClassesKey,
}

impl std::fmt::Display for StaticPhraseGenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StaticPhraseGenError::MissingConfigTable => "'config' table is missing".to_owned(),
                StaticPhraseGenError::MissingFormatString => "config.format is missing".to_owned(),
                StaticPhraseGenError::MissingConfigTypeHeader =>
                    "config.type key is missing".to_owned(),
                StaticPhraseGenError::MissingFormatKey => todo!(),
                StaticPhraseGenError::InvalidConfigTypeHeader => todo!(),
                StaticPhraseGenError::MissingClassesTable => todo!(),
                StaticPhraseGenError::InvalidClassesKey => todo!(),
            }
        )
    }
}

impl std::error::Error for StaticPhraseGenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

/// Associates a word category with a vector of words to generate phrases with static structure.
#[derive(Debug)]
pub struct StaticPhraseGen {
    map: HashMap<String, Vec<String>>,
    format: String,
}

impl TryFrom<toml::Table> for StaticPhraseGen {
    type Error = StaticPhraseGenError;

    fn try_from(value: toml::Table) -> Result<Self, Self::Error> {
        // Get config table
        let config = value
            .get("config")
            .ok_or(StaticPhraseGenError::MissingConfigTable)?
            .as_table()
            .ok_or(StaticPhraseGenError::MissingConfigTable)?;

        // Fail if the config type isn't "static"
        assert_eq!(
            config
                .get("type")
                .ok_or(StaticPhraseGenError::MissingConfigTypeHeader)?
                .as_str()
                .ok_or(StaticPhraseGenError::InvalidConfigTypeHeader)?,
            "static"
        );

        // Get the format string
        let format = config
            .get("format")
            .ok_or(StaticPhraseGenError::MissingFormatString)?
            .as_str()
            .ok_or(StaticPhraseGenError::MissingFormatString)?
            .to_owned();

        // Get the map for the dictionary
        let mut map = HashMap::new();
        for (part_of_speech, words) in value
            .get("classes")
            .ok_or(StaticPhraseGenError::MissingClassesTable)?
            .as_table()
            .ok_or(StaticPhraseGenError::MissingClassesTable)?
            .iter()
        {
            map.insert(
                part_of_speech.clone(),
                words
                    .as_str()
                    .ok_or(StaticPhraseGenError::InvalidClassesKey)?
                    .split_terminator('\n')
                    .map(|s| s.to_owned())
                    .collect(),
            );
        }

        // Done
        Ok(StaticPhraseGen { map, format })
    }
}

impl Default for StaticPhraseGen {
    /// Creates a new empty Dictionary.
    fn default() -> Self {
        StaticPhraseGen {
            map: HashMap::new(),
            format: "".to_owned(),
        }
    }
}

impl PhraseGen for StaticPhraseGen {
    /// Returns a copy of the format string with the category keys replaced with one of its values.
    /// Returns Some if all of the fields used for generation contain at least one word, otherwire returns None.
    fn get_phrase(&self) -> Option<String> {
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
