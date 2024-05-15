/// Constant map of example names, descriptions and program strings.
///
/// Names must be unique because they serve as primary keys.
const EXAMPLES: [(&str, &str, &str); 15] = [
    (
        "Welcome 💡",
        r#"<h3>👋 Welcome to the Simfony IDE!</h3>
<p>Write your program in the text box above ⬆️
The Simfony compiler will give you live feedback about your code.
When you are happy with your program, click the run button on the right ➡️
If your program succeeds, then you would be able to spend its locked coins if this were the blockchain.
If your program fails, then the transaction would be invalid. There is a stack trace for debugging.<p>

<h3>🚧 Troubleshooting</h3>
<p>Living parsing actually makes the IDE slower than it needs to be.
In a futur version, we will try to do the heavy parsing just in time before running.
Some error messages are not very helpful because the compiler is too stupid.
We are working on making the compiler smarter.
Get help on <a href="https://github.com/BlockstreamResearch/simfony/discussions">GitHub discussions</a> / <a href="https://github.com/BlockstreamResearch/simfony/issues">open an issue</a> / reach out on <a href="https://t.me/simplicity_community">Telegram.</a><p>

<h3>📝 Your Task</h3>
<p>Click the run button.
You complete the lesson by making the program succeed.</p>"#,
        r#"// Anyone can spend the empty program
// Click the run button"#,
    ),
    (
        "Variables 💡",
        r#"<h3>Assigning Values to Variables</h3>
<p>Like in Rust, you define variables using let statements.
The variable named <code>x</code> is of type <code>u32</code> (unsigned 32-bit integer).
<code>x</code> is assigned the value that follows after the equality sign </code>=</code>.
Lines are terminated with a semicolon <code>;</code>.</p>

<h3>📝 Your Task</h3>
<p>Assign the value 1337 to variable <code>x</code>.</p>"#,
        r#"let x: u32 = ; // <- Assign the value here
// Click the run button

jet_verify(jet_eq_32(x, 1337));"#,
    ),
    (
        "Integers 💡",
        r#"<h3>Available Integer Types</h3>
<p>Simfony supports unsigned integers from 1 bit to 256 bits:
<code>u1</code>, <code>u2</code>, <code>u4</code>, <code>u8</code>, <code>u16</code>, <code>u32</code>, <code>u64</code>, <code>u128</code>, <code>u256</code>.
You can write decimal literals 0, 1, 2 for values of type <code>u8</code> to <code>u64</code>.
<code>u1</code>, <code>u2</code>, <code>u4</code> require bit literals <code>0b01..01</code> of appropriate length.
<code>u128</code>, <code>u256</code> require byte literals (hex) <code>0xab...cd</code> of appropriate length.</p>

<h3>📝 Your Task</h3>
<p>Assign the maximum <code>u8</code> and <code>u128</code> values.</p>"#,
        r#"let max8: u8 = ; // <- Assign the maximum u8 value
let max128: u128 = ; // <- Assign the maximum u128 value
// Click the run button

jet_verify(jet_all_8(max8));
let (top, bot) = max128;
jet_verify(jet_all_64(top));
jet_verify(jet_all_64(bot));"#,
    ),
    (
        "Products 💡",
        r#"<h3>Combining Values in Products</h3>
<p>Simfony doesn't support structs or objects, but it supports "products".
You take data <code>x</code> and <code>y</code> and group them in the product <code>(x, y)</code>.
Think of <code>(x, y)</code> as an anonymous struct with two members: <code>x</code> and <code>y</code>.</p>

<h3>🚧 Troubleshooting</h3>
<p>We will introduce structs in a future version of Simfony, but for now you have to use products.
Right now, u16 is a macro for the product (u8, u8), and so on.</p>

<h3>📝 Your Task</h3>
<p>Combine "beef" with "babe".</p>"#,
        r#"let beef: u16 = 0xbeef;
let babe: u16 = 0xbabe;
let beefbabe: u32 = ; // <- Construct a product
// Click the run button

jet_verify(jet_eq_32(0xbeefbabe, beefbabe));"#,
    ),
    (
        "Blocks 💡",
        r#"<h3>Grouping Code in Blocks</h3>
<p>You can write expressions inside brackets <code>{ }</code> to put them into a "block".
Blocks execute a sequence of expressions and return a value at the end.
Like in Rust, the final line in a block ends without semicolon <code>;</code>.</p>

<h3>Scoping</h3>
<p>Each block introduces a new scope:
Variables live for as long as the block in which they are defined.
Variables from inner scopes overwrite / shadow variables from outer scopes.</p>

<h3>📝 Your Task</h3>
<p>Use shadowing to make 2 + 2 = 5.</p>"#,
        r#"let (_, four): (u1, u32) = jet_add_32(2, 2);
let five: u32 = 5;
let what_is_false_is_true = {
    // <- Shadow "four" to make 2 + 2 = 5
    jet_eq_32(four, five)
};

jet_verify(what_is_false_is_true);"#,
    ),
    (
        "Functions 💡",
        r#"<h3>Grouping Code in Functions</h3>
<p>Use functions to encapsulate repetitive code.
Like in Rust, the function signature starts with <code>fn</code>, followed by the function name, and the list of parameters in parentheses.
The function body follows, which is simply a block that may only use parameter variables.
The function returns on the final line of its body.

<h3>🚧 Troubleshooting</h3>
<p>Because the compiler is stupid, the parameters are implictly typed.
We are working on explicitly typed parameters.
There are no early returns via the <code>return</code> keyword at the moment.
Functions can call other function that have already been defined.
This means recursion is currently impossible.
We are looking into enabling safe recursion.</p>

<h3>📝 Your Task</h3>
<p>Define the function <code>checked_add_32</code> which takes two u32 values and adds them.
While <code>jet_add_32</code> returns a carry, <code>checked_add_32</code> is supposed to panic if there is an overflow.
It can be annoying to handle carry bits.</p>"#,
        r#"let (carry, sum) = jet_add_32(123456, 1);
// assert!(carry == false)
jet_verify(jet_complement_1(carry)); // <- Use the first three lines as body

jet_verify(jet_eq_32(sum, 123457));

fn checked_add_32(a, b) {
    // <- Add the body, using parameters a, b instead of concrete values
    // <- Return the sum at the end of the block
};

jet_verify_jet_eq_32(checked_add_32(123456, 1), 123457);"#,
    ),
    (
        "Jets 💡",
        r#"<h3>Calling Optimized Subroutines</h3>
<p>Jets are predefined functions that start with <code>jet_</code>.
While functions are executed as a blob of Simplicity instructions, jets are executed as optimized machine instructions.
This means jets are faster than functions, but there is only a fixed set of jets.
Combine jets in a function to compute what you cannot compute with jets alone.</p>

<h3>🚧 Troubleshooting</h3>
<p>You find a list of documented jets on the <a href="https://github.com/BlockstreamResearch/simplicity/wiki/Supported-Jets">Simplicity Wiki on GitHub</a>.</p>

<h3>📝 Your Task</h3>
<p>Define a NAND gate using the available jets.</p>"#,
        r#"// jet_and_1 = AND, jet_or_1 = OR, jet_xor_1 = XOR, jet_complement_1 = NOT
fn nand(a, b) {
    // <- Your body here
};

jet_verify(nand(false, false));
jet_verify(nand(false, true));
jet_verify(nand(true, false));
jet_verify(jet_complement_1(nand(true, true)));"#,
    ),
    (
        "Match expressions 💡",
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
        "BIP 340 Schnorr",
        r#"Verify a Schnorr signature.
Because the signed message is arbitrary, the program is as powerful as OP_CHECKSIGFROMSTACKVERIFY.
Here, the signature is backed into the program. This is just for demonstration purposes.
In reality, the signature would live inside the witness.
In a future version of the IDE, the witness data will be customizable."#,
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
🧨 This program is quite large, currently slow and might break your browser.
The IDE currently compiles the entire program every time the program text is updated."#,
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
