pub mod static_phrase_gen;

use std::error::Error;
use toml;

use static_phrase_gen::StaticPhraseGen;
pub trait PhraseGen {
    /// Attempts to generate a random phrase.
    /// Returns Some if all of the fields used for generation contain at least one word, otherwire returns None.
    fn get_phrase(&self) -> Option<String>;
}

#[derive(Debug)]
pub enum PhraseGenBuilderError {
    UnknownConfigType(String),
    MissingConfigTypeStr,
    BuildingError(Box<dyn Error>),
}

impl std::fmt::Display for PhraseGenBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PhraseGenBuilderError::UnknownConfigType(t) => format!("Unknown config.type: {t}"),
                PhraseGenBuilderError::MissingConfigTypeStr =>
                    "config.type key missing.".to_owned(),
                PhraseGenBuilderError::BuildingError(_error) => todo!(),
            }
        )
    }
}

impl std::error::Error for PhraseGenBuilderError {}

/// Tries to get config.type as a string from a TOML.
pub fn get_config_type_from_toml(t: &toml::Table) -> Option<&str> {
    t.get("config")?.as_table()?.get("type")?.as_str()
}

/// Creates a PhraseGen according to the config.type specified in the TOML.
pub fn phrase_gen_from_toml(t: toml::Table) -> Result<Box<dyn PhraseGen>, PhraseGenBuilderError> {
    let config_type =
        get_config_type_from_toml(&t).ok_or(PhraseGenBuilderError::MissingConfigTypeStr)?;

    match config_type {
        "static" => match StaticPhraseGen::try_from(t) {
            Err(e) => Err(PhraseGenBuilderError::BuildingError(Box::new(e))),
            Ok(pg) => Ok(Box::new(pg)),
        },
        other => Err(PhraseGenBuilderError::UnknownConfigType(other.to_owned())),
    }
}
