pub mod phrase_gen_build_error;

use phrase_gen_build_error::PhraseGenBuildError;
use std::collections::HashMap;
use toml;

/// Associates a word category with a vector of words to generate phrases with static structure.
#[derive(Debug)]
pub struct PhraseGen {
    map: HashMap<String, Vec<String>>,
    format: String,
}

impl TryFrom<toml::Table> for PhraseGen {
    type Error = PhraseGenBuildError;

    fn try_from(value: toml::Table) -> Result<Self, Self::Error> {
        // Get config table
        let config = value
            .get("config")
            .ok_or(PhraseGenBuildError::MissingField("config".to_owned()))?
            .as_table()
            .ok_or(PhraseGenBuildError::WrongFieldType("config".to_owned()))?;

        // Fail if the config type isn't "static"
        assert_eq!(
            config
                .get("type")
                .ok_or(PhraseGenBuildError::MissingField("config.type".to_owned()))?
                .as_str()
                .ok_or(PhraseGenBuildError::WrongFieldType(
                    "config.type".to_owned()
                ))?,
            "static"
        );

        // Get the format string
        let format = config
            .get("format")
            .ok_or(PhraseGenBuildError::MissingField(
                "config.format".to_owned(),
            ))?
            .as_str()
            .ok_or(PhraseGenBuildError::WrongFieldType(
                "config.format".to_owned(),
            ))?
            .to_owned();

        // Get the map for the dictionary
        let mut map = HashMap::new();
        for (part_of_speech, words) in value
            .get("classes")
            .ok_or(PhraseGenBuildError::MissingField("classes".to_owned()))?
            .as_table()
            .ok_or(PhraseGenBuildError::WrongFieldType("classes".to_owned()))?
            .into_iter()
        {
            map.insert(
                part_of_speech.clone(),
                words
                    .as_str()
                    .ok_or(PhraseGenBuildError::WrongFieldType(format!(
                        "classes.{part_of_speech}"
                    )))?
                    .split_terminator('\n')
                    .map(|s| s.to_owned())
                    .collect(),
            );
        }

        // Done
        Ok(PhraseGen { map, format })
    }
}

/// Tries to get config.type as a string from a TOML.
pub fn get_config_type_from_toml(t: &toml::Table) -> Option<&str> {
    t.get("config")?.as_table()?.get("type")?.as_str()
}
