#[allow(unused_imports)]
use rand::seq::SliceRandom;
use std::collections::HashMap;
use toml::{self, Table};

/// Possible errors when creating a `Dictionary`
#[derive(Debug)]
pub enum DictionaryError {
    MissingConfigTable,
    MissingConfigTypeHeader,
    InvalidConfigTypeHeader,
    MissingFormatKey,
    MissingFormatString,
    TomlError(toml::de::Error),
    MissingClassesTable,
    InvalidClassesKey,
}

impl std::fmt::Display for DictionaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DictionaryError::TomlError(diagnostic) =>
                    format!("Could not parse toml: {}", diagnostic),
                DictionaryError::MissingConfigTable => "'config' table is missing".to_owned(),
                DictionaryError::MissingFormatString => "config.format is missing".to_owned(),
                DictionaryError::MissingConfigTypeHeader => "config.type key is missing".to_owned(),
                DictionaryError::MissingFormatKey => todo!(),
                DictionaryError::InvalidConfigTypeHeader => todo!(),
                DictionaryError::MissingClassesTable => todo!(),
                DictionaryError::InvalidClassesKey => todo!(),
            }
        )
    }
}

impl std::error::Error for DictionaryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

/// Holds a `HashMap<PartOfSpeech, Vec<&str>` and associated logic for generating random phrases.
#[derive(Debug)]
pub struct Dictionary {
    map: HashMap<String, Vec<String>>,
    format: String,
}

impl TryFrom<String> for Dictionary {
    type Error = DictionaryError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parsed_file = value
            .parse::<Table>()
            .map_err(|e| DictionaryError::TomlError(e))?;

        // Get config table
        let config = parsed_file
            .get("config")
            .ok_or(DictionaryError::MissingConfigTable)?
            .as_table()
            .ok_or(DictionaryError::MissingConfigTable)?;

        // Fail if the config type isn't "static"
        assert_eq!(
            config
                .get("type")
                .ok_or(DictionaryError::MissingConfigTypeHeader)?
                .as_str()
                .ok_or(DictionaryError::InvalidConfigTypeHeader)?
                , "static"
        );

        // Get the format string
        let format = config
            .get("format")
            .ok_or(DictionaryError::MissingFormatString)?
            .as_str()
            .ok_or(DictionaryError::MissingFormatString)?
            .to_owned();

        // Get the map for the dictionary
        let mut map = HashMap::new();
        for (part_of_speech, words) in parsed_file
            .get("classes")
            .ok_or(DictionaryError::MissingClassesTable)?
            .as_table()
            .ok_or(DictionaryError::MissingClassesTable)?
            .iter()
        {
            map.insert(
                part_of_speech.clone(),
                words
                    .as_str()
                    .ok_or(DictionaryError::InvalidClassesKey)?
                    .split_terminator('\n')
                    .map(|s| s.to_owned())
                    .collect(),
            );
        }

        // Done
        Ok(Dictionary { map, format })
    }
}

impl Default for Dictionary {
    /// Creates a new empty Dictionary.
    fn default() -> Self {
        Dictionary {
            map: HashMap::new(),
            format: "".to_owned(),
        }
    }
}

impl Dictionary {
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
