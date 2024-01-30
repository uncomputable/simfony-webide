// Names must be unique because they serve as primary keys
pub(crate) const NAME_TO_PROGRAM: [(&str, &str); 10] = [
    ("unit", UNIT),
    ("iden", IDEN),
    ("not", NOT),
    ("word", WORD),
    ("disconnect", DISCONNECT),
    ("assertl", ASSERTL),
    ("assertr", ASSERTR),
    ("assertl failure", ASSERTL_FAILURE),
    ("jet_one failure", JET_ONE_FAILURE),
    ("byte equality failure", BYTE_EQUALITY),
];

#[rustfmt::skip]
pub(crate) const NAME_TO_DESCRIPTION: [(&str, &str); 3] = [
    ("unit", UNIT_DESCRIPTION),
    ("iden", IDEN_DESCRIPTION),
    ("byte equality failure", BYTE_EQUALITY_DESCRIPTION),
];

pub fn get_names() -> impl ExactSizeIterator<Item = &'static str> {
    NAME_TO_PROGRAM.iter().map(|(name, _)| *name)
}

pub fn get_program(name: &str) -> Option<&'static str> {
    NAME_TO_PROGRAM
        .iter()
        .find(|(program_name, _)| &name == program_name)
        .map(|(_, human)| *human)
}

pub fn get_description(name: &str) -> Option<&'static str> {
    NAME_TO_DESCRIPTION
        .iter()
        .find(|(program_name, _)| &name == program_name)
        .map(|(_, description)| *description)
}

pub const UNIT: &str = r#"main := unit : 1 -> 1"#;
const UNIT_DESCRIPTION: &str = r#"The unit program is an ANYONECANSPEND."#;

pub const IDEN: &str = r#"main := iden : 1 -> 1"#;
const IDEN_DESCRIPTION: &str = r#"The identity program is an ANYONECANSPEND"#;

pub const NOT: &str = r#"not := comp (pair iden unit) (case (injr unit) (injl unit)) : 2 -> 2
input := injl unit : 1 -> 2
output := unit : 2 -> 1
main := comp input (comp not output) : 1 -> 1"#;
pub const WORD: &str = r#"input := const 0xff : 1 -> 2^8
output := unit : 2^8 -> 1
main := comp input output: 1 -> 1"#;
pub const DISCONNECT: &str = r#"id1 := iden : 2^256 * 1 -> 2^256 * 1
disc1 := unit : 1 * 1 -> 1
main := comp (disconnect id1 ?hole) unit : 1 -> 1"#;
pub const ASSERTL: &str = r#"input := pair (const 0b0) unit : 1 -> 2 * 1
output := assertl unit #{unit} : 2 * 1 -> 1
main := comp input output : 1 -> 1"#;
pub const ASSERTR: &str = r#"input := pair (const 0b1) unit : 1 -> 2 * 1
output := assertr #{unit} unit : 2 * 1 -> 1
main := comp input output : 1 -> 1"#;
pub const ASSERTL_FAILURE: &str = r#"input := pair (const 0b1) unit : 1 -> 2 * 1
output := assertl unit #{unit} : 2 * 1 -> 1
main := comp input output : 1 -> 1"#;
pub const JET_ONE_FAILURE: &str = r#"main := comp jet_one_8 unit : 1 -> 1"#;

pub const BYTE_EQUALITY: &str = r#"a := const 0x00
b := const 0x00
main := comp (comp (pair a b) jet_eq_8) jet_verify"#;
const BYTE_EQUALITY_DESCRIPTION: &str = r#"Succeeds if a and b are equal and fails otherwise."#;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn name_primary_key() {
        assert_eq!(get_names().len(), get_names().collect::<HashSet<_>>().len());
    }
}
