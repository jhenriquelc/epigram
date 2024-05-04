use rand::seq::SliceRandom;
use std::collections::HashMap;
use strum::IntoEnumIterator;

use crate::part_of_speech::PartOfSpeech;

type IniMap = HashMap<String, HashMap<String, Option<String>>>;

/// Possible errors when creating a `Dictionary`
#[derive(Debug)]
pub enum DictionaryError {
    InvalidHeader(String),
    MissingHeader(Vec<PartOfSpeech>),
}

impl std::fmt::Display for DictionaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn pos_vec_to_string(parts_of_speech: &Vec<PartOfSpeech>) -> String {
            let mut buf = String::new();
            for &pos in parts_of_speech {
                buf.push_str(String::from(pos).as_str());
                buf.push_str(", ");
            }
            buf.trim_end_matches(", ").to_string()
        }

        write!(
            f,
            "{}",
            match self {
                DictionaryError::InvalidHeader(invalid_pos) => format!(
                    "\"{}\" isn't recognized as a valid part of speech.",
                    invalid_pos
                ),
                DictionaryError::MissingHeader(missing_headers) => format!(
                    "The following part of speech headers are missing from your dictionary: {}",
                    pos_vec_to_string(missing_headers)
                ),
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
    map: HashMap<PartOfSpeech, Vec<String>>,
}

impl TryFrom<IniMap> for Dictionary {
    type Error = DictionaryError;

    fn try_from(mut ini_map: IniMap) -> Result<Dictionary, DictionaryError> {
        let mut dict = Dictionary::new();
        for (section, mut keys) in ini_map.drain() {
            let keys: Vec<String> = keys.drain().map(|(key, _)| key).collect();

            if let Ok(part_of_speech) = PartOfSpeech::try_from(section.as_str()) {
                let field = dict.get_part_of_speech_mut(part_of_speech);
                *field = keys;
            } else {
                return Err(DictionaryError::InvalidHeader(section.clone()));
            }
        }
        let mut missing_parts_of_speech = Vec::new();
        for part_of_speech in PartOfSpeech::iter() {
            let field = dict.get_part_of_speech(part_of_speech);
            if field.is_empty() {
                missing_parts_of_speech.push(part_of_speech);
            }
        }
        if !missing_parts_of_speech.is_empty() {
            return Err(DictionaryError::MissingHeader(missing_parts_of_speech));
        }
        Ok(dict)
    }
}

impl Default for Dictionary {
    fn default() -> Self {
        Self::new()
    }
}

impl Dictionary {
    /// Creates a new empty Dictionary.
    /// The `map` contains default values for every valid key.  
    pub fn new() -> Self {
        let mut dict = Dictionary {
            map: HashMap::new(),
        };
        for part_of_speech in PartOfSpeech::iter() {
            dict.map.entry(part_of_speech).or_default();
        }
        dict
    }

    /// Attempts to generate a random phrase with the words contained in the dictionary.
    /// Returns Some if all of the fields used for generation contain at least one word, otherwire returns None.
    pub fn get_phrase(&self) -> Option<String> {
        let mut rng = rand::thread_rng();
        let adjective = self
            .get_part_of_speech(PartOfSpeech::Adjective)
            .choose(&mut rng)?;
        let noun = self
            .get_part_of_speech(PartOfSpeech::Noun)
            .choose(&mut rng)?;
        let verb = self
            .get_part_of_speech(PartOfSpeech::Verb)
            .choose(&mut rng)?;
        let adverb = self
            .get_part_of_speech(PartOfSpeech::Adverb)
            .choose(&mut rng)?;

        Some(format!("The {adjective} {noun} {verb} {adverb}."))
    }

    /// Gets an immutable reference to the vector related to the PartOfSpeech passed to it.
    pub fn get_part_of_speech(&self, part_of_speech: PartOfSpeech) -> &Vec<String> {
        self.map.get(&part_of_speech).expect(
            "Dictionary should be initialized with empty vectors for each PartOfSpeech variant",
            // the initializer guarantees all possible keys contain a default value
        )
    }

    /// Gets a mutable reference to the vector related to the PartOfSpeech passed to it.
    pub fn get_part_of_speech_mut(&mut self, part_of_speech: PartOfSpeech) -> &mut Vec<String> {
        self.map.entry(part_of_speech).or_default()
    }
}
