use crate::{dictionary::Dictionary, BUILT_IN_DICTIONARY_STR};

#[test]
fn dictionary_from_builtin_generates_non_empty_phrases() {
    let dict =
        Dictionary::try_from(BUILT_IN_DICTIONARY_STR.to_string()).expect("valid dictionary");
    assert_ne!("", dict.get_phrase().expect("a phrase"));
}
