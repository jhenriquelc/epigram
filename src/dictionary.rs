use configparser::ini::{Ini, IniDefault};
#[allow(unused_imports)]
use rand::seq::SliceRandom;
use std::collections::HashMap;

/// Possible errors when creating a `Dictionary`
#[derive(Debug)]
pub enum DictionaryError {
    MissingConfigHeader,
    MissingFormatKey,
    MissingFormatString,
    IniError(String),
}

impl std::fmt::Display for DictionaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DictionaryError::IniError(diagnostic) =>
                    format!("Could not parse ini: {}", diagnostic),
                DictionaryError::MissingConfigHeader => "'config' header is missing".to_owned(),
                DictionaryError::MissingFormatKey =>
                    "'format' key missing in 'config' header".to_owned(),
                DictionaryError::MissingFormatString => "'format' key missing its value".to_owned(),
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

    fn try_from(value: String) -> Result<Dictionary, DictionaryError> {
        // set up ini settings
        let default_ini = {
            let mut default = IniDefault::default();
            default.case_sensitive = true;
            default.multiline = false;
            default
        };

        // read ini file
        let mut ini_map = match Ini::new_from_defaults(default_ini).read(value) {
            Ok(map) => Ok(map),
            Err(e) => Err(DictionaryError::IniError(e)),
        }?;

        // extract format string
        let format: String = ini_map
            .remove("config")
            .ok_or(DictionaryError::MissingConfigHeader)?
            .remove("format")
            .ok_or(DictionaryError::MissingFormatKey)?
            .ok_or(DictionaryError::MissingFormatString)?;

        // extract parts of speech and their respective words
        let mut map = HashMap::new();
        for (section, mut keys) in ini_map.drain() {
            let keys: Vec<String> = keys.drain().map(|(key, _)| key).collect();

            let part_of_speech = map.entry(section).or_default();
            *part_of_speech = keys;
        }

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

    /// Gets an immutable reference to the vector related to the PartOfSpeech passed to it.
    pub fn get_part_of_speech(&self, part_of_speech: String) -> &Vec<String> {
        self.map.get(&part_of_speech).expect(
            "Dictionary should be initialized with empty vectors for each PartOfSpeech variant",
            // the initializer guarantees all possible keys contain a default value
        )
    }

    /// Gets a mutable reference to the vector related to the PartOfSpeech passed to it.
    pub fn get_part_of_speech_mut(&mut self, part_of_speech: String) -> &mut Vec<String> {
        self.map.entry(part_of_speech).or_default()
    }
}
