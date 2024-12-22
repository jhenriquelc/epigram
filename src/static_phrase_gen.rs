#[allow(unused_imports)]
use rand::seq::SliceRandom;
use std::collections::HashMap;
use toml::{self, Table};

/// Possible errors when creating a `StaticPhraseGen`
#[derive(Debug)]
pub enum StaticPhraseGenError {
    MissingConfigTable,
    MissingConfigTypeHeader,
    InvalidConfigTypeHeader,
    MissingFormatKey,
    MissingFormatString,
    TomlError(toml::de::Error),
    MissingClassesTable,
    InvalidClassesKey,
}

impl std::fmt::Display for StaticPhraseGenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StaticPhraseGenError::TomlError(diagnostic) =>
                    format!("Could not parse toml: {}", diagnostic),
                StaticPhraseGenError::MissingConfigTable => "'config' table is missing".to_owned(),
                StaticPhraseGenError::MissingFormatString => "config.format is missing".to_owned(),
                StaticPhraseGenError::MissingConfigTypeHeader => "config.type key is missing".to_owned(),
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

impl TryFrom<String> for StaticPhraseGen {
    type Error = StaticPhraseGenError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parsed_file = value
            .parse::<Table>()
            .map_err(|e| StaticPhraseGenError::TomlError(e))?;

        // Get config table
        let config = parsed_file
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
                .ok_or(StaticPhraseGenError::InvalidConfigTypeHeader)?
                , "static"
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
        for (part_of_speech, words) in parsed_file
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

impl StaticPhraseGen {
    /// Attempts to generate a random phrase with the words contained in the dictionary.
    /// Returns Some if all of the fields used for generation contain at least one word, otherwire returns None.
    pub fn get_phrase(&self) -> Option<String> {
        let mut rng = rand::thread_rng();
        let mut out = self.format.clone();
        for (part_of_speech, words) in &self.map {
            let key = format!("{{{{{part_of_speech}}}}}"); // {{ → {. damn you, escape sequences!!
            while out.contains(&key) {
                let word = words.choose(&mut rng)?;
                out = out.replacen(&key, word, 1);
            }
        }

        Some(out)
    }
}
