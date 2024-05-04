use crate::DictionaryError;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// Represents a word category
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
    type Error = DictionaryError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        for pos in PartOfSpeech::iter() {
            if String::from(pos) == value.to_lowercase().trim().trim_end_matches('s') {
                return Ok(pos);
            }
        }
        Err(DictionaryError::InvalidHeader(value.to_string()))
    }
}
