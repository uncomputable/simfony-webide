// Names must be unique because they serve as primary keys
pub(crate) const NAME_TO_PROGRAM: [(&str, &str); 13] = [
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
    ("Schnorr signature failure", SCHNORR),
    ("Bit flip failure", BIT_FLIP),
    ("SHA failure", SHA),
];

#[rustfmt::skip]
pub(crate) const NAME_TO_DESCRIPTION: [(&str, &str); 6] = [
    ("unit", UNIT_DESCRIPTION),
    ("iden", IDEN_DESCRIPTION),
    ("byte equality failure", BYTE_EQUALITY_DESCRIPTION),
    ("Schnorr signature failure", SCHNORR_DESCRIPTION),
    ("Bit flip failure", BIT_FLIP_DESCRIPTION),
    ("SHA failure", SHA_DESCRIPTION),
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
main := comp (disconnect id1 ?hole) unit : 1 -> 1
hole := unit : 1 * 1 -> 1"#;
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

pub const SCHNORR: &str = r#"sig := const 0xe907831f80848d1069a5371b402410364bdf1c5f8307b0084c55f1ce2dca821525f66a4a85ea8b71e482a74f382d2ce5ebeee8fdb2172f477df4900d310536c0 : 1 -> 2^512
pk := const 0xf9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9 : 1 -> 2^256
msg := const 0x0000000000000000000000000000000000000000000000000000000000000000 : 1 -> 2^256
in := pair (pair pk msg) sig : 1 -> 2^512 * 2^512
out := jet_bip_0340_verify : 2^512 * 2^512 -> 1
main := comp in out"#;
const SCHNORR_DESCRIPTION: &str =
    r#"Succeeds if the Schnorr signature matches the public key and message, and fails otherwise."#;

pub const BIT_FLIP: &str = r#"bit_0 := injl unit : 1 * 1 -> 2
bit_1 := injr unit : 1 * 1 -> 2
bit_out := case bit_1 bit_0 : 2 * 1 -> 2
pad_r := pair iden unit : 2 -> 2 * 1
bitflip := comp pad_r bit_out : 2 -> 2
zerovfy := comp bitflip jet_verify : 2 -> 1
input := const 0b0 : 1 -> 2
main := comp input zerovfy : 1 -> 1"#;
const BIT_FLIP_DESCRIPTION: &str = r#"Verify that the input bit is zero.

Flips the input bit and uses the verify jet to assert "1".

See https://blog.blockstream.com/simplicity-sharing-of-witness-and-disconnect/"#;

pub const SHA: &str = r#"sha256_init : 2^256 -> _
sha256_init := comp unit jet_sha_256_ctx_8_init

sha256 : 2^256 -> 2^256
sha256 := comp
    comp
        pair sha256_init iden
        jet_sha_256_ctx_8_add_32
    jet_sha_256_ctx_8_finalize

preimage := const 0x0000000000000000000000000000000000000000000000000000000000000000
image := const 0x66687aadf862bd776c8fc18b8e9f8e20089714856ee233b3902a591d0d5f2925

main := comp
    comp
        pair (comp preimage sha256) image
        jet_eq_256
    jet_verify"#;
pub const SHA_DESCRIPTION: &str = r#"Verify that the given preimage results in the given image.

See https://blog.blockstream.com/simplicity-sharing-of-witness-and-disconnect/"#;

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
