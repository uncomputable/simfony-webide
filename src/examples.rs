/// Constant map of example names, descriptions and program strings.
///
/// Names must be unique because they serve as primary keys.
const EXAMPLES: [(&str, &str, &str); 11] = [
    (
        "Empty",
        r#"The empty string is a valid program.
It does nothing and immediately unlocks its coins."#,
        r#""#,
    ),
    (
        "Block expressions",
        r#"Use blocks expressions to give your programs more structure.
Each block returns a value at its end."#,
        r#"let a: u32 = 10;
let b = {
    let c: u32 = 2;
    let d: u32 = 3;
    jet_verify(jet_eq_32(a, 10)); // Use variables from outer copes
    let a: u32 = 7; // Shadow variables from outer scopes
    jet_max_32(jet_max_32(c, d), a) // Missing ; because the block returns a value
};
jet_verify(jet_eq_32(b, 7));"#,
    ),
    (
        "Match expressions",
        r#"Use match expressions to choose between multiple execution paths.
Unused code is removed inside the Simplicity target code and will never touch the blockchain."#,
        r#"let bit: u1 = match Left(11) {
    Left(x) => jet_le_32(10, x),
    Right(y) => jet_le_32(y, 10),
};
jet_verify(bit);
let bit: u1 = match Some(11) {
    Some(x) => jet_le_32(10, x),
    None => 0,
};
jet_verify(bit);
let bit: bool = match true {
    true => 1,
    false => 0,
};
jet_verify(bit);"#,
    ),
    (
        "Functions",
        r#"Use functions to encapsulate repetitive code.
Functions are compressed inside the Simplicity target code."#,
        r#"fn forty_two() {
    42
};
let a = forty_two();
let b = 42;
jet_verify(jet_eq_32(a, b));
fn checked_add_32(x, y) {
    let (carry, sum) = jet_add_32(x, y);
    jet_verify(jet_complement_1(carry));
    sum
};
let a = 1;
let b = 4294967294;
let c = checked_add_32(a, b);
let d = 4294967295;
jet_verify(jet_eq_32(c, d));
fn first() {
    1
};
fn second() {
    checked_add_32(first(), first())
};
fn third() {
    checked_add_32(first(), second())
};
let a = third();
let b = 3;
jet_verify(jet_eq_32(a, b));"#,
    ),
    (
        "List sum",
        r#"Sum the elements of a list.
The length of the list is between one (inclusive) and a maximum (exclusive)."#,
        r#"fn checked_add_32(el, acc) {
    let (carry, sum) = jet_add_32(el, acc);
    // assert_eq!(carry, 0)
    jet_verify(jet_complement_1(carry));
    sum
};

// Sum 1 element
let list: List<u32, 2> = list![1];
let sum: u32 = fold::<2>(list, 0, checked_add_32);
jet_verify(jet_eq_32(1, sum));

// Sum 2 elements
let list: List<u32, 4> = list![1, 2];
let sum: u32 = fold::<4>(list, 0, checked_add_32);
jet_verify(jet_eq_32(3, sum));

// Sum 3 elements
let list: List<u32, 4> = list![1, 2, 3];
let sum: u32 = fold::<4>(list, 0, checked_add_32);
jet_verify(jet_eq_32(6, sum));

// Sum 4 elements
let list: List<u32, 8> = list![1, 2, 3, 4];
let sum: u32 = fold::<8>(list, 0, checked_add_32);
jet_verify(jet_eq_32(10, sum));"#,
    ),
    (
        "Byte hash loop 🧨",
        r#"Hash bytes 0x00 to 0xff in a loop.
🧨 This program is quite large, currently slow and might break your browser."#,
        r#"// Add counter to streaming hash and finalize when the loop exists
fn hash_counter_8(cnt, acc) {
    let new_acc = jet_sha_256_ctx_8_add_1(acc, cnt);
    match jet_all_8(cnt) {
        true => Left(jet_sha_256_ctx_8_finalize(new_acc)),
        false => Right(new_acc),
    }
};

// Hash bytes 0x00 to 0xff
let ctx: (List<u8, 64>, (u64, u256)) = jet_sha_256_ctx_8_init();
let c: Either<u256, (List<u8, 64>, (u64, u256))> = forWhile::<256>(ctx, hash_counter_8);
let expected: u256 = 0x40aff2e9d2d8922e47afd4648e6967497158785fbd1da870e7110266bf944880;
jet_verify(jet_eq_256(expected, unwrap_left(c)));"#,
    ),
    (
        "BIP 340 Schnorr",
        r#"Verify a Schnorr signature.
Because the signed message is arbitrary, the program is as powerful as OP_CHECKSIGFROMSTACKVERIFY."#,
        r#"let pk: u256 = 0xf9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9;
let sig: (u256, u256) = 0xe907831f80848d1069a5371b402410364bdf1c5f8307b0084c55f1ce2dca821525f66a4a85ea8b71e482a74f382d2ce5ebeee8fdb2172f477df4900d310536c0;
let msg: u256 = 0x0000000000000000000000000000000000000000000000000000000000000000;
jet_bip_0340_verify(pk, msg, sig);"#,
    ),
    (
        "OP_CAT",
        r#"Concatenate some bytes and verify the result."#,
        r#"let a = 0x10;
let b = 0x01;
let ab: u16 = (a, b);
let c = 0x1001;
jet_verify(jet_eq_16(ab, c));"#,
    ),
    (
        "Recursive covenant",
        r#"The world's simplest recursive covenant:
The scriptPubKey of the UTXO must be repeated in the first output of the spending transaction.
The spending transaction is hardcoded to satisfy the covenant.
In a future version of the IDE, the transaction will be customizable."#,
        "let utxo_hash:  u256 = jet_current_script_hash();
let spend_hash: u256 = unwrap(jet_output_script_hash(0));
jet_verify(jet_eq_256(utxo_hash, spend_hash));",
    ),
    (
        "OP_CTV",
        r#"Verify an OP_CTV hash.
Instead of specifying the template hash as in BIP CTV,
we require the user to specify all the components of the sighash
that they want to commit.
The spending transaction is hardcoded to satisfy the covenant.
In a future version of the IDE, the transaction will be customizable."#,
        r#"let ctx = jet_sha_256_ctx_8_init();
let ctx = jet_sha_256_ctx_8_add_4(ctx, jet_version());
let ctx = jet_sha_256_ctx_8_add_4(ctx, jet_lock_time());
let ctx = jet_sha_256_ctx_8_add_32(ctx, jet_input_script_sigs_hash());
let ctx = jet_sha_256_ctx_8_add_4(ctx, jet_num_inputs());
let ctx = jet_sha_256_ctx_8_add_32(ctx, jet_input_sequences_hash());
let ctx = jet_sha_256_ctx_8_add_4(ctx, jet_num_outputs());
let ctx = jet_sha_256_ctx_8_add_32(ctx, jet_outputs_hash());
let ctx = jet_sha_256_ctx_8_add_4(ctx, jet_current_index());
let ctv_hash: u256 = jet_sha_256_ctx_8_finalize(ctx);

let expected_hash: u256 = 0xae3d019b30529c6044d2b3d7ee2e0ee5db51a7f05ed5db8f089cd5d455f1fc5d;
jet_verify(jet_eq_256(ctv_hash, expected_hash));"#,
    ),
    (
        "SIGHASH_NONE",
        r#"Verify a Schnorr signature based on SIGHASH_NONE of the spending transaction.
Here, the signature is backed into the program. This is just for demonstration purposes.
In reality, the signature would live inside the witness.
In a future version of the IDE, the witness data will be customizable."#,
        r#"let ctx = jet_sha_256_ctx_8_init();
// Blockchain
let ctx = jet_sha_256_ctx_8_add_32(ctx, jet_genesis_block_hash());
let ctx = jet_sha_256_ctx_8_add_32(ctx, jet_genesis_block_hash());
// Transaction
let ctx = jet_sha_256_ctx_8_add_4(ctx, jet_version());
let ctx = jet_sha_256_ctx_8_add_4(ctx, jet_lock_time());
let ctx = jet_sha_256_ctx_8_add_32(ctx, jet_tap_env_hash());
// All inputs
let ctx = jet_sha_256_ctx_8_add_32(ctx, jet_inputs_hash());
let ctx = jet_sha_256_ctx_8_add_32(ctx, jet_input_utxos_hash());
// No outputs
// Current index
let ctx = jet_sha_256_ctx_8_add_4(ctx, jet_current_index());
// Message
let msg = jet_sha_256_ctx_8_finalize(ctx);

let pk: u256 = 0xf9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9;
let sig: (u256, u256) = 0x346152583d5b60b972bb4c03ab672e339431060e2b09c447ab983c65dabc70a459f3beca7788bfa5da221cf99227b65b4ad3821a2045c847ee56d48df26aee9c;
jet_bip_0340_verify(pk, msg, sig);"#,
    ),
];

/// Iterate over the example names.
pub fn get_names() -> impl ExactSizeIterator<Item = &'static str> {
    EXAMPLES.iter().map(|entry| entry.0)
}

/// Take an example name and return the example description.
pub fn get_description(name: &str) -> Option<&'static str> {
    EXAMPLES
        .iter()
        .find(|entry| entry.0 == name)
        .map(|entry| entry.1)
}

/// Take an example name and return the example program string.
pub fn get_program_str(name: &str) -> Option<&'static str> {
    EXAMPLES
        .iter()
        .find(|entry| entry.0 == name)
        .map(|entry| entry.2)
}

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
