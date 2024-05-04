use crate::{dictionary::Dictionary, BUILT_IN_DICTIONARY_STR};
use configparser::ini::Ini;

#[test]
fn make_dictionary_from_builtin() {
    let map = Ini::new()
        .read(BUILT_IN_DICTIONARY_STR.to_string())
        .expect("valid ini");
    let _dict = Dictionary::try_from(map).expect("valid dictionary");
}
