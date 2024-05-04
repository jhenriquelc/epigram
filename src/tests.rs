use crate::{dictionary::Dictionary, BUILT_IN_DICTIONARY_STR};

#[test]
fn make_dictionary_from_builtin() {
    let _dict =
        Dictionary::try_from(BUILT_IN_DICTIONARY_STR.to_string()).expect("valid dictionary");
}
