/// Constant map of example names, descriptions and program strings.
///
/// Names must be unique because they serve as primary keys.
#[rustfmt::skip]
const EXAMPLES: [(&str, &str, &str); 16] = [
    (
        "Welcome 💡",
        r#"<h3>👋 Welcome to the Simfony IDE!</h3>
<p>Write your program in the text box above ⬆️
The Simfony compiler will give you live feedback about your code.
When you are happy with your program, click the run button below ⬇️
If your program succeeds, then you would be able to spend its locked coins if this were the blockchain.
If your program fails, then the transaction would be invalid. There is a stack trace for debugging.<p>

<h3>🚧 Troubleshooting</h3>
<p>Living parsing actually makes the IDE slower than it needs to be.
In a future version, we will do the heavy parsing just in time before running.
Some error messages are not very helpful because the compiler is too stupid.
We are working on making the compiler smarter.
Get help on <a href="https://github.com/BlockstreamResearch/simfony/discussions">GitHub discussions</a> / <a href="https://github.com/BlockstreamResearch/simfony/issues">open an issue</a> / reach out on <a href="https://t.me/simplicity_community">Telegram.</a><p>

<h3>📝 Your Task</h3>
<p>Click the run button.
You complete the lesson by making the program succeed.
Switch to the next lesson in the dropdown list above ⬆️</p>"#,
        r#"fn main() {
    // Anyone can spend the empty program
}"#,
    ),
    (
        "Variables 💡",
        r#"<h3>Assigning Values to Variables</h3>
<p>You define variables using let statements of the form <code>let X: TYPE_OF_X = VALUE_OF_X</code>.
In this example, the variable <code>X</code> is of type <code>TYPE_OF_X</code> and holds the value <code>VALUE_OF_X</code>.
The line ends with a semicolon <code>;</code>.
Unlike in Rust, every variable must be explicitly typed by the programmer.
The Simfony compiler doesn't infer types.</p>

<h3>📝 Your Task</h3>
<p>Assign the value 1337 to variable <code>x</code>.</p>"#,
        r#"fn main() {
    let x: u32 = ; // <- Assign the value
    assert!(jet::eq_32(x, 1337));
}"#,
    ),
    (
        "Integers 💡",
        r#"<h3>Available Integer Types</h3>
<p>Simfony supports unsigned integers from 1 bit to 256 bits:
<code>u1</code>, <code>u2</code>, <code>u4</code>, <code>u8</code>, <code>u16</code>, <code>u32</code>, <code>u64</code>, <code>u128</code>, <code>u256</code>.
You can write decimal literals: <code>0</code>, <code>1</code>, <code>2</code>.
You can also write bit literals of appropriate length: <code>0b00</code>, <code>0b01</code>, <code>0b10</code>.
Finally, types that are at least 1 byte long support byte (hex) literals of appropriate length: <code>0x00</code>, <code>0x01</code>, <code>0x02</code>.
Half bytes (aka nibbles) are not supported.</p>

<h3>📝 Your Task</h3>
<p>Assign the maximum <code>u8</code> and <code>u128</code> values.</p>"#,
        r#"fn main() {
    let max8: u8 = ; // <- Assign the maximum u8 value
    let max128: u128 = ; // <- Assign the maximum u128 value

    assert!(jet::all_8(max8));
    let (top, bot): (u64, u64) = <u128>::into(max128);
    assert!(jet::all_64(top));
    assert!(jet::all_64(bot));
}"#,
    ),
    (
        "Products 💡",
        r#"<h3>Combining Values in Products</h3>
<p>Simfony doesn't support structs or objects, but it supports "products".
You take data <code>x</code> and <code>y</code> and group them in the product <code>(x, y)</code>.
Think of <code>(x, y)</code> as an anonymous struct with two members: <code>x</code> and <code>y</code>.</p>

<h3>🚧 Troubleshooting</h3>
<p>We will introduce structs in a future version of Simfony, but for now you have to use products.</p>

<h3>📝 Your Task</h3>
<p>Combine "beef" with "babe".</p>"#,
        r#"fn main() {
    let beef: u16 = 0xbeef;
    let babe: u16 = 0xbabe;
    let beefbabe: (u16, u16) = ; // <- Construct a product
    // Cast product of two 16-bit integers to one 32-bit integer
    let beefbabe: u32 = <(u16, u16)>::into(beefbabe);

    assert!(jet::eq_32(0xbeefbabe, beefbabe));
}"#,
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
<p>Use shadowing to make <a href="https://en.wikipedia.org/wiki/2_%2B_2_%3D_5">2 + 2 = 5</a>.</p>"#,
        r#"fn main() {
    let (_, four): (bool, u32) = jet::add_32(2, 2);
    let five: u32 = 5;
    let what_is_false_is_true: bool = {
        // <- Shadow "four" to make 2 + 2 = 5
        jet::eq_32(four, five)
    };

    assert!(what_is_false_is_true);
}"#,
    ),
    (
        "Functions 💡",
        r#"<h3>Grouping Code in Functions</h3>
<p>Use functions to encapsulate repetitive code.
Like in Rust, the function signature starts with <code>fn</code>, followed by the function name, and the list of parameters in parentheses.
The function body follows, which is simply a block that may only use parameter variables.
The function returns on the final line of its body.

<h3>🚧 Troubleshooting</h3>
<p>There are no early returns via the <code>return</code> keyword at the moment.
Functions can call other function that have already been defined.
This means recursion is currently impossible.
We are looking into enabling safe recursion.</p>

<h3>📝 Your Task</h3>
<p>Define the function <code>checked_add_32</code> which takes two u32 values and adds them.
While <code>jet::add_32</code> returns a carry, <code>checked_add_32</code> is supposed to panic if there is an overflow.
It can be annoying to handle carry bits.</p>"#,
        r#"fn checked_add_32(a: u32, b: u32) -> u32 {
    // your function body here
    sum  // return the sum
}

fn main() {
        // ⬇️ Use these two lines as function body ⬇️
        let (carry, manual_sum): (bool, u32) = jet::add_32(123456, 1);
        assert!(<u1>::into(jet::complement_1(<bool>::into(carry))));  // assert!(carry == false)
        // ⬆️ Use these two lines as function body ⬆️

        let automatic_sum: u32 = checked_add_32(123456, 1)
        assert!(jet::eq_32(manual_sum, automatic_sum));
}"#,
    ),
    (
        "Jets 💡",
        r#"<h3>Calling Optimized Subroutines</h3>
<p>Jets are predefined functions that live in the <code>jet::</code> namespace.
While functions are executed as a blob of Simplicity instructions, jets are executed as optimized machine instructions.
This means jets are faster than functions, but there is only a fixed set of jets.
Combine jets in a function to compute what you cannot compute with jets alone.</p>

<h3>🚧 Troubleshooting</h3>
<p>There is a <a href="https://docs.rs/simfony-as-rust/latest/simfony_as_rust/jet/index.html">complete documentation of all jets on docs.rs</a>.</p>

<h3>📝 Your Task</h3>
<p>Define a <a href="https://en.wikipedia.org/wiki/NAND_gate">NAND gate</a> using the available jets.</p>"#,
        r#"// jet::and_1 = AND, jet::or_1 = OR, jet::xor_1 = XOR, jet::complement_1 = NOT
fn nand(a: u1, b: u1) -> u1 {
    // <- a NAND b
}

fn main() {
    assert!(<u1>::into(nand(0, 0)));
    assert!(<u1>::into(nand(0, 1)));
    assert!(<u1>::into(nand(1, 0)));
    assert!(<u1>::into(jet::complement_1(nand(1, 1))));
}"#,
    ),
    (
        "Booleans 💡",
        r#"<h3>Boolean Values</h3>
<p>Values of type <code>bool</code> can either be <code>true</code> or <code>false</code>.</p>

<h3>Matching Booleans</h3>
<p>In Simfony, we use <code>match</code> expressions to handle Booleans.
The match executes the <code>true</code> branch if the input is <code>true</code>,
and it executes the <code>false</code> branch if the input is <code>false</code>.
Both branches return a value of the same output type.
The overall match returns a value of this output type.</p>

<h3>📝 Your Task</h3>
<p>Complete the function <code>invert</code> which inverts the input bit.<p>"#,
        r#"fn invert(bit: bool) -> bool {
    match bit {
        true  => , // <- Invert a true bit
        false => , // <- Invert a false bit
    }
}

fn main() {
    assert!(invert(false));
    assert!(<u1>::into(jet::complement_1(<bool>::into(invert(true)))));
}"#,
    ),
    // TODO: "Unwrap" checked_div_32(1, 0) = None
    (
        "Options 💡",
        r#"<h3>Optional Values</h3>
<p>Values of type <code>Option&lt;B&gt;</code> are either <code>Some</code> and contain a value <code>b</code> of type <code>B</code>, or they are <code>None</code> and contain no value.
We use optional values for functions that may or may not take an input (subtraction of unsigned integers),
for functions that may or may not return an output (division by zero),
and so on.
Optional values replace null pointers and runtime exceptions.</p>

<h3>Matching Optional Values</h3>
<p>We use <code>match</code> expressions to handle optional values:
The <code>None</code> branch is executed if the input contains no value.
The <code>Some(b)</code> branch is executed if the input contains value <code>b</code>.</p>

<h3>📝 Your Task</h3>
<p>The jet <code>jet::divide_32</code> returns 0 for <a href="https://en.wikipedia.org/wiki/Division_by_zero">division by zero</a>.
Complete the function <code>checked_div_32</code> which returns <code>None</code> instead."#,
        r#"fn checked_div_32(x: u32, y: u32) -> Option<u32> {
    match jet::is_zero_32(y) {
        true  => , // <- Return no integer
        false => , // <- Return some quotient
    }
}

fn main() {
    // assert!(is_none::<u32>(checked_div_32(1, 0)));
    assert!(jet::eq_32(3, unwrap(checked_div_32(10, 3))));
}"#,
    ),
    (
        "Sums 💡",
        r#"<h3>Sum Values</h3>
<p>Values of type <code>Either&lt;A, B&gt;</code> are either <code>Left</code> and contain a value <code>a</code> of type <code>A</code>, or they are <code>Right</code> and contain a value <code>b</code> of type <code>B</code>.
We use sum values to distinguish different cases:
<code>Either&lt;u32, u32&gt;</code> could be locktimes in block format or in timestamp format.
<code>Either&lt;A, B&gt;</code> could be successful values <code>Left(success)</code> or failing values <code>Right(error)</code>.
And so on.

<h3>Matching Sum Values</h3>
<p>We use <code>match</code> expressions to handle sum values:
The <code>Left(a: A)</code> branch is executed if the input contains value <code>a</code> of type <code>A</code>.
The <code>Right(b: B)</code> branch is executed if the input contains value <code>b</code> of type <code>B</code>.
Like variable assignments, variable bindings in match arms must be explicitly typed.</p>

<h3>📝 Your Task</h3>
<p><a href="https://en.wikipedia.org/wiki/Furlong">Eight furlongs are one mile.</a>
Complete the function <code>to_mile</code> which converts distances to miles.
The input may be formatted in furlongs or in miles.
Use <code>jet::divide_32</code> to do division.</p>"#,
        r#"fn to_miles(distance: Either<u32, u32>) -> u32 {
    match distance {
        Left(furlongs: u32) => , // <- Divide by 8
        Right(miles: u32)   => , // <- Return input
    }
}

fn main() {
    let eight_furlongs: Either<u32, u32> = Left(8);
    let one_mile: Either<u32, u32> = Right(1);
    assert!(jet::eq_32(1, to_miles(eight_furlongs)));
    assert!(jet::eq_32(1, to_miles(one_mile)));
}"#,
    ),
    (
        "BIP 340 Schnorr",
        r#"Verify a Schnorr signature.
Because the signed message is arbitrary, the program is as powerful as OP_CHECKSIGFROMSTACKVERIFY.
Here, the signature is backed into the program. This is just for demonstration purposes.
In reality, the signature would live inside the witness.
In a future version of the IDE, the witness data will be customizable."#,
        r#"fn main() {
    let pk: u256 = 0xf9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9;
    let msg: u256 = 0x0000000000000000000000000000000000000000000000000000000000000000;
    let sig: [u8; 64] = [0xe9, 0x07, 0x83, 0x1f, 0x80, 0x84, 0x8d, 0x10, 0x69, 0xa5, 0x37, 0x1b, 0x40, 0x24, 0x10, 0x36, 0x4b, 0xdf, 0x1c, 0x5f, 0x83, 0x07, 0xb0, 0x08, 0x4c, 0x55, 0xf1, 0xce, 0x2d, 0xca, 0x82, 0x15, 0x25, 0xf6, 0x6a, 0x4a, 0x85, 0xea, 0x8b, 0x71, 0xe4, 0x82, 0xa7, 0x4f, 0x38, 0x2d, 0x2c, 0xe5, 0xeb, 0xee, 0xe8, 0xfd, 0xb2, 0x17, 0x2f, 0x47, 0x7d, 0xf4, 0x90, 0x0d, 0x31, 0x05, 0x36, 0xc0];
    jet::bip_0340_verify((pk, msg), sig);
}"#,
    ),
    (
        "OP_CAT",
        r#"Concatenate some bytes and verify the result."#,
        r#"fn main() {
    let ab: u16 = <(u8, u8)>::into((0x10, 0x01));
    let c: u16 = 0x1001;
    assert!(jet::eq_16(ab, c));
}"#,
    ),
    (
        "Recursive covenant",
        r#"The world's simplest recursive covenant:
The scriptPubKey of the UTXO must be repeated in the first output of the spending transaction.
The spending transaction is hardcoded to satisfy the covenant.
In a future version of the IDE, the transaction will be customizable."#,
        r#"fn main() {
    let utxo_hash: u256 = jet::current_script_hash();
    let spend_hash: u256 = unwrap(jet::output_script_hash(0));
    assert!(jet::eq_256(utxo_hash, spend_hash));
}"#,
    ),
    (
        "OP_CTV",
        r#"Verify an OP_CTV hash.
Instead of specifying the template hash as in BIP CTV,
we require the user to specify all the components of the sighash
that they want to commit.
The spending transaction is hardcoded to satisfy the covenant.
In a future version of the IDE, the transaction will be customizable."#,
        r#"fn main() {
    let ctx: Ctx8 = jet::sha_256_ctx_8_init();
    let ctx: Ctx8 = jet::sha_256_ctx_8_add_4(ctx, jet::version());
    let ctx: Ctx8 = jet::sha_256_ctx_8_add_4(ctx, jet::lock_time());
    let ctx: Ctx8 = jet::sha_256_ctx_8_add_32(ctx, jet::input_script_sigs_hash());
    let ctx: Ctx8 = jet::sha_256_ctx_8_add_4(ctx, jet::num_inputs());
    let ctx: Ctx8 = jet::sha_256_ctx_8_add_32(ctx, jet::input_sequences_hash());
    let ctx: Ctx8 = jet::sha_256_ctx_8_add_4(ctx, jet::num_outputs());
    let ctx: Ctx8 = jet::sha_256_ctx_8_add_32(ctx, jet::outputs_hash());
    let ctx: Ctx8 = jet::sha_256_ctx_8_add_4(ctx, jet::current_index());
    let ctv_hash: u256 = jet::sha_256_ctx_8_finalize(ctx);

    let expected_hash: u256 = 0xae3d019b30529c6044d2b3d7ee2e0ee5db51a7f05ed5db8f089cd5d455f1fc5d;
    assert!(jet::eq_256(ctv_hash, expected_hash));
}"#,
    ),
    (
        "SIGHASH_NONE",
        r#"Verify a Schnorr signature based on SIGHASH_NONE of the spending transaction.
Here, the signature is backed into the program. This is just for demonstration purposes.
In reality, the signature would live inside the witness.
In a future version of the IDE, the witness data will be customizable."#,
        r#"fn main() {
    let ctx: Ctx8 = jet::sha_256_ctx_8_init();
    // Blockchain
    let ctx: Ctx8 = jet::sha_256_ctx_8_add_32(ctx, jet::genesis_block_hash());
    let ctx: Ctx8 = jet::sha_256_ctx_8_add_32(ctx, jet::genesis_block_hash());
    // Transaction
    let ctx: Ctx8 = jet::sha_256_ctx_8_add_4(ctx, jet::version());
    let ctx: Ctx8 = jet::sha_256_ctx_8_add_4(ctx, jet::lock_time());
    let ctx: Ctx8 = jet::sha_256_ctx_8_add_32(ctx, jet::tap_env_hash());
    // All inputs
    let ctx: Ctx8 = jet::sha_256_ctx_8_add_32(ctx, jet::inputs_hash());
    let ctx: Ctx8 = jet::sha_256_ctx_8_add_32(ctx, jet::input_utxos_hash());
    // No outputs
    // Current index
    let ctx: Ctx8 = jet::sha_256_ctx_8_add_4(ctx, jet::current_index());
    // Message
    let msg: u256 = jet::sha_256_ctx_8_finalize(ctx);

    let pk: u256 = 0xf9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9;
    let sig: [u8; 64] = [0x34, 0x61, 0x52, 0x58, 0x3d, 0x5b, 0x60, 0xb9, 0x72, 0xbb, 0x4c, 0x03, 0xab, 0x67, 0x2e, 0x33, 0x94, 0x31, 0x06, 0x0e, 0x2b, 0x09, 0xc4, 0x47, 0xab, 0x98, 0x3c, 0x65, 0xda, 0xbc, 0x70, 0xa4, 0x59, 0xf3, 0xbe, 0xca, 0x77, 0x88, 0xbf, 0xa5, 0xda, 0x22, 0x1c, 0xf9, 0x92, 0x27, 0xb6, 0x5b, 0x4a, 0xd3, 0x82, 0x1a, 0x20, 0x45, 0xc8, 0x47, 0xee, 0x56, 0xd4, 0x8d, 0xf2, 0x6a, 0xee, 0x9c];
    jet::bip_0340_verify((pk, msg), sig);
}"#,
    ),
    (
        "List sum",
        r#"Sum the elements of a list."#,
        r#"fn checked_add_32(a: u32, b: u32) -> u32 {
    let (carry, sum): (bool, u32) = jet::add_32(a, b);
    assert!(<u1>::into(jet::complement_1(<bool>::into(carry))));
    sum
}

fn main() {
    // This list holds less than 4 elements
    let list: List<u32, 4> = list![1, 2, 3];
    // This list holds less than 8 elements
    let bigger_list: List<u32, 8> = list![1, 2, 3, 4, 5, 6, 7];

    // Sum the first list
    let sum: u32 = fold::<checked_add_32, 4>(list, 0);
    assert!(jet::eq_32(6, sum));

    // Sum the second list
    let sum: u32 = fold::<checked_add_32, 8>(bigger_list, 0);
    assert!(jet::eq_32(28, sum));
}"#,
    ),
    /*
    (
        "Byte hash loop 🧨",
        r#"<p>Hash bytes 0x00 to 0xff in a loop.</p>

<h3>🧨 Reckless Program</h3>
<p>This program is quite large and might break your browser.
Every time you type, the IDE parses and compiles the entire program, which is slow.
Running the loop also takes longer than expected.
We are working on browser optimizations.
Mind that the program runs within milliseconds on the blockchain!</p>"#,
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
    */
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
