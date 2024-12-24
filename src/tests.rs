use crate::{phrase_gen::phrase_gen_from_toml, STATIC_PHRASE_GEN_EXAMPLE_STR};
use toml;

#[test]
fn dictionary_from_builtin_generates_non_empty_phrases() {
    let t = STATIC_PHRASE_GEN_EXAMPLE_STR
        .parse::<toml::Table>()
        .expect("parse toml");
    let pg = phrase_gen_from_toml(t).expect("build PhraseGen");
    assert_ne!("", pg.get_phrase().expect("a phrase").trim());
}
