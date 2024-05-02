use configparser::ini::Ini;
use rand::seq::SliceRandom;
use std::{self, collections::HashMap, process::exit};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type IniMap = HashMap<String, HashMap<String, Option<String>>>;

#[derive(Debug)]
pub enum Error {
    InvalidHeader(String),
    MissingHeader(Vec<PartOfSpeech>),
}

fn pos_vec_to_string(parts_of_speech: &Vec<PartOfSpeech>) -> String {
    let mut buf = String::new();
    for pos in parts_of_speech {
        buf.push_str(String::from(pos).as_str());
        buf.push_str(", ");
    }
    buf.trim_end_matches(", ").to_string()
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::InvalidHeader(invalid_pos) => format!(
                    "\"{}\" isn't recognized as a valid part of speech.",
                    invalid_pos
                ),
                Error::MissingHeader(missing_headers) => format!(
                    "The following part of speech headers are missing from your dictionary: {}",
                    pos_vec_to_string(missing_headers)
                ),
            }
        )
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

#[derive(Debug, EnumIter)]
pub enum PartOfSpeech {
    Verb,
    Noun,
    Adjective,
    Adverb,
}

impl From<&PartOfSpeech> for String {
    fn from(value: &PartOfSpeech) -> Self {
        match value {
            PartOfSpeech::Verb => "verb",
            PartOfSpeech::Noun => "noun",
            PartOfSpeech::Adjective => "adjective",
            PartOfSpeech::Adverb => "adverb",
        }
        .to_string()
    }
}

impl TryFrom<&str> for PartOfSpeech {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        for pos in PartOfSpeech::iter() {
            if String::from(&pos) == value.to_lowercase().trim().trim_end_matches('s') {
                return Ok(pos);
            }
        }
        Err(
            Error::InvalidHeader(value.to_string())
        )
    }
}

#[derive(Debug)]
pub struct Dictionary<'a> {
    verbs: Vec<&'a str>,
    nouns: Vec<&'a str>,
    adjectives: Vec<&'a str>,
    adverbs: Vec<&'a str>,
}

impl<'a> From<&'a IniMap> for Dictionary<'a> {
    fn from(ini_map: &'a IniMap) -> Self {
        let mut dict = Dictionary {
            verbs: Vec::new(),
            nouns: Vec::new(),
            adjectives: Vec::new(),
            adverbs: Vec::new(),
        };
        for (section, keys) in ini_map.iter() {
            let keys: Vec<&str> = keys.iter().map(|(key, _)| key.as_str()).collect();

            if let Ok(part_of_speech) = PartOfSpeech::try_from(section.as_str()) {
                match part_of_speech {
                    PartOfSpeech::Verb => dict.verbs = keys,
                    PartOfSpeech::Noun => dict.nouns = keys,
                    PartOfSpeech::Adjective => dict.adjectives = keys,
                    PartOfSpeech::Adverb => dict.adverbs = keys,
                }
            }
        }
        dict
    }
}

impl Dictionary<'_> {
    fn get_phrase(&self) -> Option<String> {
        let mut rng = rand::thread_rng();
        let adjective = self.adjectives.choose(&mut rng)?;
        let noun = self.nouns.choose(&mut rng)?;
        let verb = self.verbs.choose(&mut rng)?;
        let adverb = self.adverbs.choose(&mut rng)?;

        Some(format!("The {adjective} {noun} {verb} {adverb}."))
    }
}

fn main() -> Result<(), u8> {
    let filename = "./data/dictionary.ini";

    let file = match Ini::new().load(filename) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Could not open \"{filename}\": {}", e);
            exit(1);
        }
    };

    let dict = Dictionary::from(&file);

    println!("{}", dict.get_phrase().unwrap_or("".to_string()));

    Ok(())
}
