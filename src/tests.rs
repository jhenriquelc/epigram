use crate::{phrase_gen::PhraseGen, STATIC_PHRASE_GEN_EXAMPLE_STR};
use toml;

#[test]
fn phrase_gen_example_non_empty_phrases() {
    let t = STATIC_PHRASE_GEN_EXAMPLE_STR
        .parse::<toml::Table>()
        .expect("parse toml");
    let pg = PhraseGen::try_from(t).expect("build PhraseGen");
    assert_ne!("", pg.get_phrase().expect("a phrase").trim());
}
