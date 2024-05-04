use clap::Parser;
use clio::{self, Input};
use configparser::ini::Ini;
use rand::seq::SliceRandom;
use std::{self, collections::HashMap, hash::Hash, process::exit};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type IniMap = HashMap<String, HashMap<String, Option<String>>>;

const BUILT_IN_DICTIONARY_STR: &str = include_str!("./dictionary.ini");

#[derive(Debug)]
pub enum Error {
    InvalidHeader(String),
    MissingHeader(Vec<PartOfSpeech>),
}

fn pos_vec_to_string(parts_of_speech: &Vec<PartOfSpeech>) -> String {
    let mut buf = String::new();
    for &pos in parts_of_speech {
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

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PartOfSpeech {
    Verb,
    Noun,
    Adjective,
    Adverb,
}

impl From<PartOfSpeech> for String {
    fn from(value: PartOfSpeech) -> Self {
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
            if String::from(pos) == value.to_lowercase().trim().trim_end_matches('s') {
                return Ok(pos);
            }
        }
        Err(Error::InvalidHeader(value.to_string()))
    }
}

#[derive(Debug)]
pub struct Dictionary<'a> {
    map: HashMap<PartOfSpeech, Vec<&'a str>>,
}

impl<'a> TryFrom<&'a IniMap> for Dictionary<'a> {
    type Error = Error;

    fn try_from(ini_map: &'a IniMap) -> Result<Dictionary<'a>, Error> {
        let mut dict = Dictionary::new();
        for (section, keys) in ini_map.iter() {
            let keys: Vec<&str> = keys.iter().map(|(key, _)| key.as_str()).collect();

            if let Ok(part_of_speech) = PartOfSpeech::try_from(section.as_str()) {
                let field = dict.get_part_of_speech_mut(part_of_speech);
                *field = keys;
            } else {
                return Err(Error::InvalidHeader(section.clone()));
            }
        }
        let mut missing_parts_of_speech = Vec::new();
        for part_of_speech in PartOfSpeech::iter() {
            let field = dict.get_part_of_speech(part_of_speech);
            if field.len() == 0 {
                missing_parts_of_speech.push(part_of_speech);
            }
        }
        if missing_parts_of_speech.len() != 0 {
            return Err(Error::MissingHeader(missing_parts_of_speech));
        }
        Ok(dict)
    }
}

impl<'a> Dictionary<'a> {
    pub fn new() -> Self {
        let mut dict = Dictionary {
            map: HashMap::new(),
        };
        for part_of_speech in PartOfSpeech::iter() {
            dict.map.entry(part_of_speech).or_insert(Vec::new());
        }
        dict
    }

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

    pub fn get_part_of_speech(&self, part_of_speech: PartOfSpeech) -> &Vec<&str> {
        self.map.get(&part_of_speech).expect(
            "Dictionary should be initialized with empty vectors for each PartOfSpeech variant",
        )
    }

    pub fn get_part_of_speech_mut(&mut self, part_of_speech: PartOfSpeech) -> &mut Vec<&'a str> {
        self.map.entry(part_of_speech).or_insert(Vec::new())
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(value_parser)]
    dictionary_file: Option<Input>,

    #[arg(short = 'n', long, default_value_t = 1, group = "len")]
    count: u128,

    #[arg(short, long, group = "len")]
    infinite: bool,
}

fn main() -> Result<(), u8> {
    let Args {
        dictionary_file: input,
        count,
        infinite,
    } = Args::parse();

    let ini_string = if let Some(mut input) = input {
        if input.is_tty() {
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
        BUILT_IN_DICTIONARY_STR.to_string()
    };

    let parsed = Ini::new().read(ini_string);
    let map = match parsed {
        Ok(map) => map,
        Err(e) => {
            eprintln!("Could not parse ini: {}", e);
            exit(1);
        }
    };

    let dict = match Dictionary::try_from(&map) {
        Ok(dict) => dict,
        Err(e) => {
            eprintln!("{}", e);
            exit(3);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_dictionary_from_builtin() {
        let map = Ini::new()
            .read(BUILT_IN_DICTIONARY_STR.to_string())
            .expect("valid ini");
        let _dict = Dictionary::try_from(&map).expect("valid dictionary");
    }
}
