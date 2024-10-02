use simfony::witness::WitnessValues;
use simfony::{elements, CompiledProgram};

#[derive(Clone, Copy, Debug)]
pub struct Example {
    description: &'static str,
    program: &'static str,
    witness: &'static str,
    lock_time: u32,
    sequence: u32,
}

impl Example {
    #[allow(dead_code)]
    pub fn description(self) -> &'static str {
        self.description
    }

    pub fn compiled(self) -> CompiledProgram {
        CompiledProgram::new(self.program).expect("example program should compile")
    }

    pub fn program_text(self) -> &'static str {
        self.program
    }

    pub fn witness_values(self) -> WitnessValues {
        serde_json::from_str(self.witness).expect("example witness should parse")
    }

    #[cfg(test)]
    pub fn satisfied(self) -> simfony::SatisfiedProgram {
        self.compiled()
            .satisfy(&self.witness_values())
            .expect("example program should be satisfied")
    }

    pub fn lock_time(self) -> elements::LockTime {
        elements::LockTime::from_consensus(self.lock_time)
    }

    pub fn sequence(self) -> elements::Sequence {
        elements::Sequence::from_consensus(self.sequence)
    }

    #[cfg(test)]
    pub fn tx_env(
        self,
    ) -> simfony::simplicity::jet::elements::ElementsEnv<std::sync::Arc<elements::Transaction>>
    {
        simfony::dummy_env::dummy_with(self.lock_time(), self.sequence())
    }
}

const ENABLE_LOCKTIME_NO_RBF: u32 = 0xFFFFFFFE;

const P2PK: Example = Example {
    description: r#"Pay to public key.

The coins move if the person with the given public key signs the transaction."#,
    program: r#"fn main() {
    let pk: Pubkey = 0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798; // 1 * G
    let msg: u256 = jet::sig_all_hash();
    let sig: Signature = witness("sig");
    jet::bip_0340_verify((pk, msg), sig)
}"#,
    witness: r#"{
    "sig": {
        "value": "0xf74b3ca574647f8595624b129324afa2f38b598a9c1c7cfc5f08a9c036ec5acd3c0fbb9ed3dae5ca23a0a65a34b5d6cccdd6ba248985d6041f7b21262b17af6f",
        "type": "Signature"
    }
}"#,
    lock_time: 0,
    sequence: u32::MAX,
};

const P2PKH: Example = Example {
    description: r#"Pay to public key hash.

The coins move if the person with the public key that matches the given hash signs the transaction."#,
    program: r#"fn sha2(string: u256) -> u256 {
    let hasher: Ctx8 = jet::sha_256_ctx_8_init();
    let hasher: Ctx8 = jet::sha_256_ctx_8_add_32(hasher, string);
    jet::sha_256_ctx_8_finalize(hasher)
}

fn main() {
    let pk: Pubkey = witness("pk");
    let expected_pk_hash: u256 = 0x132f39a98c31baaddba6525f5d43f2954472097fa15265f45130bfdb70e51def; // sha2(1 * G)
    let pk_hash: u256 = sha2(pk);
    assert!(jet::eq_256(pk_hash, expected_pk_hash));

    let msg: u256 = jet::sig_all_hash();
    let sig: Signature = witness("sig");
    jet::bip_0340_verify((pk, msg), sig)
}"#,
    witness: r#"{
    "pk": {
        "value": "0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        "type": "Pubkey"
    },
    "sig": {
        "value": "0xf74b3ca574647f8595624b129324afa2f38b598a9c1c7cfc5f08a9c036ec5acd3c0fbb9ed3dae5ca23a0a65a34b5d6cccdd6ba248985d6041f7b21262b17af6f",
        "type": "Signature"
    }
}"#,
    lock_time: 0,
    sequence: u32::MAX,
};

const P2MS: Example = Example {
    description: r#"Pay to multisig.

The coins move if 2 of 3 people agree to move them. These people provide their signatures, of which exactly 2 are required."#,
    program: r#"fn not(bit: bool) -> bool {
    <u1>::into(jet::complement_1(<bool>::into(bit)))
}

fn checksig(pk: Pubkey, sig: Signature) {
    let msg: u256 = jet::sig_all_hash();
    jet::bip_0340_verify((pk, msg), sig);
}

fn checksig_add(counter: u8, pk: Pubkey, maybe_sig: Option<Signature>) -> u8 {
    match maybe_sig {
        Some(sig: Signature) => {
            checksig(pk, sig);
            let (carry, new_counter): (bool, u8) = jet::increment_8(counter);
            assert!(not(carry));
            new_counter
        }
        None => counter,
    }
}

fn check2of3multisig(pks: [Pubkey; 3], maybe_sigs: [Option<Signature>; 3]) {
    let [pk1, pk2, pk3]: [Pubkey; 3] = pks;
    let [sig1, sig2, sig3]: [Option<Signature>; 3] = maybe_sigs;

    let counter1: u8 = checksig_add(0, pk1, sig1);
    let counter2: u8 = checksig_add(counter1, pk2, sig2);
    let counter3: u8 = checksig_add(counter2, pk3, sig3);

    let threshold: u8 = 2;
    assert!(jet::eq_8(counter3, threshold));
}

fn main() {
    let pks: [Pubkey; 3] = [
        0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798, // 1 * G
        0xc6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5, // 2 * G
        0xf9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9, // 3 * G
    ];
    let maybe_sigs: [Option<Signature>; 3] = witness("maybe_sigs");
    check2of3multisig(pks, maybe_sigs);
}"#,
    witness: r#"{
    "maybe_sigs": {
        "value": "[Some(0xf74b3ca574647f8595624b129324afa2f38b598a9c1c7cfc5f08a9c036ec5acd3c0fbb9ed3dae5ca23a0a65a34b5d6cccdd6ba248985d6041f7b21262b17af6f), None, Some(0x29dbeab5628ae472bce3e08728ead1997ef789d4f04b5be39cc08b362dc229f553fd353f8a0acffdfbddd471d15a0dda3b306842416ff246bc07462e5667eb89)]",
        "type": "[Option<Signature>; 3]"
    }
}"#,
    lock_time: 0,
    sequence: u32::MAX,
};

const SIGHASH_ANYPREVOUT: Example = Example {
    description: r#"Pay to public key with SIGHASH_ANYPREVOUT.

The coins move if the person with the given public key signs the transaction.
The transaction input can be exchanged by a third party with a "similar" input while the signature remains valid."#,
    program: r#"fn main() {
    let ctx: Ctx8 = jet::sha_256_ctx_8_init();
    // Blockchain
    let ctx: Ctx8 = jet::sha_256_ctx_8_add_32(ctx, jet::genesis_block_hash());
    let ctx: Ctx8 = jet::sha_256_ctx_8_add_32(ctx, jet::genesis_block_hash());
    // Transaction
    let ctx: Ctx8 = jet::sha_256_ctx_8_add_4(ctx, jet::version());
    let ctx: Ctx8 = jet::sha_256_ctx_8_add_4(ctx, jet::lock_time());
    let ctx: Ctx8 = jet::sha_256_ctx_8_add_32(ctx, jet::tap_env_hash());
    // Current input without outpoint
    let ctx: Ctx8 = jet::sha_256_ctx_8_add_4(ctx, jet::current_sequence());
    let (current_asset, current_amount): (Asset1, Amount1) = jet::current_amount();
    let ctx: Ctx8 = jet::asset_amount_hash(ctx, current_asset, current_amount);
    let ctx: Ctx8 = jet::sha_256_ctx_8_add_32(ctx, jet::current_script_hash());
    let ctx: Ctx8 = jet::annex_hash(ctx, jet::current_annex_hash());
    // All outputs
    let ctx: Ctx8 = jet::sha_256_ctx_8_add_32(ctx, jet::outputs_hash());
    let ctx: Ctx8 = jet::sha_256_ctx_8_add_32(ctx, jet::issuances_hash());
    let ctx: Ctx8 = jet::sha_256_ctx_8_add_32(ctx, jet::output_surjection_proofs_hash());
    // No current index
    // Message
    let msg: u256 = jet::sha_256_ctx_8_finalize(ctx);

    let pk: Pubkey = 0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798; // 1 * G
    let sig: Signature = witness("sig");
    jet::bip_0340_verify((pk, msg), sig);
}"#,
    witness: r#"{
    "sig": {
        "value": "0x171678f669e1a81980b94e7677dcc827d5d0d429d8fde0503c333eab4781ae9e4861eeef3a7e9f6d840e330fcc70bf9f3b3723594b0dd4093b211de995b30e52",
        "type": "Signature"
    }
}"#,
    lock_time: 0,
    sequence: u32::MAX,
};

const HTLC: Example = Example {
    description: r#"Hash Time-Locked contract.

The recipient can spend the coins by providing the secret preimage of a hash.
The sender can cancel the transfer after a fixed block height.

HTLCs enable two-way payment channels and multi-hop payments, such as on the Lightning network."#,
    program: r#"fn sha2(string: u256) -> u256 {
    let hasher: Ctx8 = jet::sha_256_ctx_8_init();
    let hasher: Ctx8 = jet::sha_256_ctx_8_add_32(hasher, string);
    jet::sha_256_ctx_8_finalize(hasher)
}

fn checksig(pk: Pubkey, sig: Signature) {
    let msg: u256 = jet::sig_all_hash();
    jet::bip_0340_verify((pk, msg), sig);
}

fn complete_spend(preimage: u256, recipient_sig: Signature) {
    let hash: u256 = sha2(preimage);
    let expected_hash: u256 = 0x66687aadf862bd776c8fc18b8e9f8e20089714856ee233b3902a591d0d5f2925; // sha2([0x00; 32])
    assert!(jet::eq_256(hash, expected_hash));
    let recipient_pk: Pubkey = 0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798; // 1 * G
    checksig(recipient_pk, recipient_sig);
}

fn cancel_spend(sender_sig: Signature) {
    let timeout: Height = 1000;
    jet::check_lock_height(timeout);
    let sender_pk: Pubkey = 0xc6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5; // 2 * G
    checksig(sender_pk, sender_sig)
}

fn main() {
    let complete_or_cancel: Either<(u256, Signature), Signature> = witness("complete_or_cancel");
    match complete_or_cancel {
        Left(preimage_sig: (u256, Signature)) => {
            let (preimage, recipient_sig): (u256, Signature) = preimage_sig;
            complete_spend(preimage, recipient_sig);
        },
        Right(sender_sig: Signature) => cancel_spend(sender_sig),
    }
}"#,
    witness: r#"{
    "complete_or_cancel": {
        "value": "Left((0x0000000000000000000000000000000000000000000000000000000000000000, 0xf74b3ca574647f8595624b129324afa2f38b598a9c1c7cfc5f08a9c036ec5acd3c0fbb9ed3dae5ca23a0a65a34b5d6cccdd6ba248985d6041f7b21262b17af6f))",
        "type": "Either<(u256, Signature), Signature>"
    }
}"#,
    lock_time: 0,
    sequence: u32::MAX,
};

const HOLD_VAULT: Example = Example {
    description: r#"Lock your coins until the Bitcoin price exceeds a threshold.

An oracle signs a message with the current block height and the current Bitcoin price.
The block height is compared with a minimum height to prevent the use of old data.
The transaction is timelocked to the oracle height,
which means that the transaction becomes valid after the oracle height."#,
    program: r#"fn checksig(pk: Pubkey, sig: Signature) {
    let msg: u256 = jet::sig_all_hash();
    jet::bip_0340_verify((pk, msg), sig);
}

fn checksigfromstack(pk: Pubkey, bytes: [u32; 2], sig: Signature) {
    let [word1, word2]: [u32; 2] = bytes;
    let hasher: Ctx8 = jet::sha_256_ctx_8_init();
    let hasher: Ctx8 = jet::sha_256_ctx_8_add_4(hasher, word1);
    let hasher: Ctx8 = jet::sha_256_ctx_8_add_4(hasher, word2);
    let msg: u256 = jet::sha_256_ctx_8_finalize(hasher);
    jet::bip_0340_verify((pk, msg), sig);
}

fn main() {
    let min_height: Height = 1000;
    let oracle_height: Height = witness("oracle_height");
    assert!(jet::le_32(min_height, oracle_height));
    jet::check_lock_height(oracle_height);

    let target_price: u32 = 100000; // laser eyes until 100k
    let oracle_price: u32 = witness("oracle_price");
    assert!(jet::le_32(target_price, oracle_price));

    let oracle_pk: Pubkey = 0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798; // 1 * G
    let oracle_sig: Signature = witness("oracle_sig");
    checksigfromstack(oracle_pk, [oracle_height, oracle_price], oracle_sig);

    let owner_pk: Pubkey = 0xc6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5; // 2 * G
    let owner_sig: Signature = witness("owner_sig");
    checksig(owner_pk, owner_sig);
}"#,
    witness: r#"{
    "oracle_height": {
        "value": "1000",
        "type": "u32"
    },
    "oracle_price": {
        "value": "100000",
        "type": "u32"
    },
    "oracle_sig": {
        "value": "0x90231b8de96a1f940ddcf406fe8389417ca8fb0b03151608e2f94b31b443a7e0d26a12e437df69028f09027c37d5f6742a10c1e8864061d119b8bbce962d26d3",
        "type": "Signature"
    },
    "owner_sig": {
        "value": "0xf2341f571f069216edfc72822f6094b8ec339c2f72dc64aea0eed1e3d60abf4572fdd04618e5b5bc672ccd71cfaf125b6c1b101aeca3a7b938fe83932ab38743",
        "type": "Signature"
    }
}"#,
    lock_time: 1000,
    sequence: ENABLE_LOCKTIME_NO_RBF,
};

const LAST_WILL: Example = Example {
    description: r#"The inheritor can spend the coins if the owner doesn't move the them for 180 days.
The owner has to repeat the covenant when he moves the coins with his hot key.
The owner can break out of the covenant with his cold key."#,
    program: r#"fn checksig(pk: Pubkey, sig: Signature) {
    let msg: u256 = jet::sig_all_hash();
    jet::bip_0340_verify((pk, msg), sig);
}

// Enforce the covenant to repeat in the first output.
//
// Elements has explicit fee outputs, so enforce a fee output in the second output.
// Disallow further outputs.
fn recursive_covenant() {
    assert!(jet::eq_32(jet::num_outputs(), 2));
    let this_script_hash: u256 = jet::current_script_hash();
    let output_script_hash: u256 = unwrap(jet::output_script_hash(0));
    assert!(jet::eq_256(this_script_hash, output_script_hash));
    assert!(unwrap(jet::output_is_fee(1)));
}

fn inherit_spend(inheritor_sig: Signature) {
    let days_180: Distance = 25920;
    jet::check_lock_distance(days_180);
    let inheritor_pk: Pubkey = 0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798; // 1 * G
    checksig(inheritor_pk, inheritor_sig);
}

fn cold_spend(cold_sig: Signature) {
    let cold_pk: Pubkey = 0xc6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5; // 2 * G
    checksig(cold_pk, cold_sig);
}

fn refresh_spend(hot_sig: Signature) {
    let hot_pk: Pubkey = 0xf9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9; // 3 * G
    checksig(hot_pk, hot_sig);
    recursive_covenant();
}

fn main() {
    let inherit_or_not: Either<Signature, Either<Signature, Signature>> = witness("inherit_or_not");
    match inherit_or_not {
        Left(inheritor_sig: Signature) => inherit_spend(inheritor_sig),
        Right(cold_or_hot: Either<Signature, Signature>) => match cold_or_hot {
            Left(cold_sig: Signature) => cold_spend(cold_sig),
            Right(hot_sig: Signature) => refresh_spend(hot_sig),
        },
    }
}"#,
    witness: r#"{
    "inherit_or_not": {
        "value": "Left(0x755201bb62b0a8b8d18fd12fc02951ea3998ba42bfc6664daaf8a0d2298cad43cdc21358c7c82f37654275dc2fea8c858adbe97bac92828b498a5a237004db6f)",
        "type": "Either<Signature, Either<Signature, Signature>>"
    }
}"#,
    lock_time: 0,
    sequence: 25920,
};

const HASH_LOOP: Example = Example {
    description: r#"Test how fast your browser is with this explosive program."#,
    program: r#"// Add counter to streaming hash and finalize when the loop exists
fn hash_counter_8(ctx: Ctx8, unused: (), byte: u8) -> Either<u256, Ctx8> {
    let new_ctx: Ctx8 = jet::sha_256_ctx_8_add_1(ctx, byte);
    match jet::all_8(byte) {
        true => Left(jet::sha_256_ctx_8_finalize(new_ctx)),
        false => Right(new_ctx),
    }
}

// Add counter to streaming hash and finalize when the loop exists
fn hash_counter_16(ctx: Ctx8, unused: (), bytes: u16) -> Either<u256, Ctx8> {
    let new_ctx: Ctx8 = jet::sha_256_ctx_8_add_2(ctx, bytes);
    match jet::all_16(bytes) {
        true => Left(jet::sha_256_ctx_8_finalize(new_ctx)),
        false => Right(new_ctx),
    }
}

fn main() {
    // Hash bytes 0x00 to 0xff
    let ctx: Ctx8 = jet::sha_256_ctx_8_init();
    let out: Either<u256, Ctx8> = for_while::<hash_counter_8>(ctx, ());
    let expected: u256 = 0x40aff2e9d2d8922e47afd4648e6967497158785fbd1da870e7110266bf944880;
    assert!(jet::eq_256(expected, unwrap_left::<Ctx8>(out)));

    // Hash bytes 0x0000 to 0xffff
    // This takes ~10 seconds on my computer
    // let ctx: Ctx8 = jet::sha_256_ctx_8_init();
    // let out: Either<u256, Ctx8> = for_while::<hash_counter_16>(ctx, ());
    // let expected: u256 = 0x281f79f89f0121c31db2bea5d7151db246349b25f5901c114505c18bfaa50ba1;
    // assert!(jet::eq_256(expected, unwrap_left::<Ctx8>(out)));
}"#,
    witness: "{}",
    lock_time: 0,
    sequence: 0,
};

/// Names must be unique because they serve as primary keys.
const EXAMPLES: [(&str, Example); 8] = [
    ("✍️️ P2PK", P2PK),
    ("✍️ P2PKH", P2PKH),
    ("✍️ P2MS", P2MS),
    ("✍️ SIGHASH_ANYPREVOUT", SIGHASH_ANYPREVOUT),
    ("⚡ HTLC", HTLC),
    ("💸 Hodl vault", HOLD_VAULT),
    ("📜 Last will", LAST_WILL),
    ("🧨 Hash loop", HASH_LOOP),
];

/// Iterate over the example names.
pub fn keys() -> impl ExactSizeIterator<Item = &'static str> {
    EXAMPLES.into_iter().map(|(name, _)| name)
}

/// Get the example of the given `name`.
pub fn get(name: &str) -> Option<Example> {
    EXAMPLES
        .into_iter()
        .find_map(|(found_name, found_example)| match found_name == name {
            true => Some(found_example),
            false => None,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn name_primary_key() {
        assert_eq!(keys().len(), keys().collect::<HashSet<_>>().len());
    }
}
