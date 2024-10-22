use elements::hashes::Hash;
use simfony::elements;

use crate::transaction::TxParams;

#[derive(Clone, Copy, Debug)]
pub struct Example {
    description: &'static str,
    program: &'static str,
    lock_time: u32,
    sequence: u32,
}

impl Example {
    #[allow(dead_code)]
    pub fn description(self) -> &'static str {
        self.description
    }

    pub fn program_text(self) -> &'static str {
        self.program
    }

    #[cfg(test)]
    pub fn satisfied(self) -> simfony::SatisfiedProgram {
        let compiled =
            simfony::CompiledProgram::new(self.program).expect("example program should compile");
        let witness_values =
            <simfony::WitnessValues as simfony::parse::ParseFromStr>::parse_from_str(
                self.program_text(),
            )
            .expect("example witness should parse");
        compiled
            .satisfy(&witness_values)
            .expect("example program should be satisfied")
    }

    pub fn params(self) -> TxParams {
        TxParams {
            txid: elements::Txid::all_zeros(),
            vout: 0,
            value_in: 100_000,
            recipient_address: None,
            fee: 1_000,
            lock_time: elements::LockTime::from_consensus(self.lock_time),
            sequence: elements::Sequence::from_consensus(self.sequence),
        }
    }
}

const P2PK: Example = Example {
    description: r#"Pay to public key.

The coins move if the person with the given public key signs the transaction."#,
    program: r#"mod witness {
    const SIG: Signature = 0x1d7d93f350e2db564f90da49fb00ee47294bb6d8f061929818b26065a3e50fdd87e0e8ab45eecd04df0b92b427e6d49a5c96810c23706566e9093c992e075dc5;
}

fn main() {
    let pk: Pubkey = 0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798; // 1 * G
    let msg: u256 = jet::sig_all_hash();
    jet::bip_0340_verify((pk, msg), witness::SIG)
}"#,
    lock_time: 0,
    sequence: 0,
};

const P2PKH: Example = Example {
    description: r#"Pay to public key hash.

The coins move if the person with the public key that matches the given hash signs the transaction."#,
    program: r#"mod witness {
    const PK: Pubkey = 0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798;
    const SIG: Signature = 0xcdd52dd650ad4628d0813e9c4533abb21de4c6ee69ef4512c60ffb1c93d1dbb12ef081b1d60e3ecdd23155efe45a52d19088b6cf8073dac8cd356b1eddb5ce8f;
}

fn sha2(string: u256) -> u256 {
    let hasher: Ctx8 = jet::sha_256_ctx_8_init();
    let hasher: Ctx8 = jet::sha_256_ctx_8_add_32(hasher, string);
    jet::sha_256_ctx_8_finalize(hasher)
}

fn main() {
    let pk: Pubkey = witness::PK;
    let expected_pk_hash: u256 = 0x132f39a98c31baaddba6525f5d43f2954472097fa15265f45130bfdb70e51def; // sha2(1 * G)
    let pk_hash: u256 = sha2(pk);
    assert!(jet::eq_256(pk_hash, expected_pk_hash));

    let msg: u256 = jet::sig_all_hash();
    jet::bip_0340_verify((pk, msg), witness::SIG)
}"#,
    lock_time: 0,
    sequence: 0,
};

const P2MS: Example = Example {
    description: r#"Pay to multisig.

The coins move if 2 of 3 people agree to move them. These people provide their signatures, of which exactly 2 are required."#,
    program: r#"mod witness {
    const MAYBE_SIGS: [Option<Signature>; 3] =
        [Some(0x2747bff425b0a94b12b60aa9c4e6091c8a0adbdd2536780023ab6d4883bdddaa43587a8e29bbaccbd71001ef5ba3cd06f26f0347773761124dfa2a20ada8aeb0), None, Some(0x768db05f0402104962f2cd0a1f235cde45b284a664123b9f4606d119b95feb011a28e33d2af5c6b565107e8edc5326a33dbf3f7e2c97d959e21345749c2c244c)];
}

fn not(bit: bool) -> bool {
    <u1>::into(jet::complement_1(<bool>::into(bit)))
}

fn checksig(pk: Pubkey, sig: Signature) {
    let msg: u256 = jet::sig_all_hash();
    jet::bip_0340_verify((dbg!(pk), msg), sig);
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
    check2of3multisig(pks, witness::MAYBE_SIGS);
}"#,
    lock_time: 0,
    sequence: 0,
};

const SIGHASH_ANYPREVOUT: Example = Example {
    description: r#"Pay to public key with SIGHASH_ANYPREVOUT.

The coins move if the person with the given public key signs the transaction.
The transaction input can be exchanged by a third party with a "similar" input while the signature remains valid."#,
    program: r#"mod witness {
    const SIG: Signature = 0x1d63cf0bc063d44f7546c67d7a3abcd15ef5a64aa4bb2a04e342a0a512b63d82e88638e2f85e21da67d1f709bbea6043aa59b47a3ed7e6147190ac92438824aa;
}

fn main() {
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
    let msg: u256 = dbg!(jet::sha_256_ctx_8_finalize(ctx));

    let pk: Pubkey = 0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798; // 1 * G
    jet::bip_0340_verify((pk, msg), witness::SIG);
}"#,
    lock_time: 0,
    sequence: 0,
};

const HTLC: Example = Example {
    description: r#"Hash Time-Locked contract.

The recipient can spend the coins by providing the secret preimage of a hash.
The sender can cancel the transfer after a fixed block height.

HTLCs enable two-way payment channels and multi-hop payments, such as on the Lightning network."#,
    program: r#"mod witness {
    const COMPLETE_OR_CANCEL: Either<(u256, Signature), Signature> =
        Left((0x0000000000000000000000000000000000000000000000000000000000000000, 0x9d59801a7d276756c61aac762f2009ccf48a824941aacf993bd33e383b77c8de54d346824635bceb6455574b4c2150a6205f99bb7bf8d195bd05ec7e5613583c));
}

fn sha2(string: u256) -> u256 {
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
    match witness::COMPLETE_OR_CANCEL {
        Left(preimage_sig: (u256, Signature)) => {
            let (preimage, recipient_sig): (u256, Signature) = preimage_sig;
            complete_spend(preimage, recipient_sig);
        },
        Right(sender_sig: Signature) => cancel_spend(sender_sig),
    }
}"#,
    lock_time: 0,
    sequence: 0,
};

const HOLD_VAULT: Example = Example {
    description: r#"Lock your coins until the Bitcoin price exceeds a threshold.

An oracle signs a message with the current block height and the current Bitcoin price.
The block height is compared with a minimum height to prevent the use of old data.
The transaction is timelocked to the oracle height,
which means that the transaction becomes valid after the oracle height."#,
    program: r#"mod witness {
    const ORACLE_HEIGHT: u32 = 1000;
    const ORACLE_PRICE: u32 = 100000;
    const ORACLE_SIG: Signature = 0x90231b8de96a1f940ddcf406fe8389417ca8fb0b03151608e2f94b31b443a7e0d26a12e437df69028f09027c37d5f6742a10c1e8864061d119b8bbce962d26d3;
    const OWNER_SIG: Signature = 0x34e36ba57db85d092c6cd7b820209839aa9a3936f5d843700b1f4d9b0b29a8ca64b13cdb6e1aef1bab64e712e9ee663cbddb5f269ba85b87af6ef5f2cba3a327;
}

fn checksig(pk: Pubkey, sig: Signature) {
    let msg: u256 = jet::sig_all_hash();
    jet::bip_0340_verify((pk, msg), sig);
}

fn checksigfromstack(pk: Pubkey, bytes: [u32; 2], sig: Signature) {
    let [word1, word2]: [u32; 2] = bytes;
    let hasher: Ctx8 = jet::sha_256_ctx_8_init();
    let hasher: Ctx8 = jet::sha_256_ctx_8_add_4(hasher, word1);
    let hasher: Ctx8 = jet::sha_256_ctx_8_add_4(hasher, word2);
    let msg: u256 = dbg!(jet::sha_256_ctx_8_finalize(hasher));
    jet::bip_0340_verify((pk, msg), sig);
}

fn main() {
    let min_height: Height = 1000;
    let oracle_height: Height = witness::ORACLE_HEIGHT;
    assert!(jet::le_32(min_height, oracle_height));
    jet::check_lock_height(oracle_height);

    let target_price: u32 = 100000; // laser eyes until 100k
    let oracle_price: u32 = witness::ORACLE_PRICE;
    assert!(jet::le_32(target_price, oracle_price));

    let oracle_pk: Pubkey = 0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798; // 1 * G
    let oracle_sig: Signature = witness::ORACLE_SIG;
    checksigfromstack(oracle_pk, [oracle_height, oracle_price], oracle_sig);

    let owner_pk: Pubkey = 0xc6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5; // 2 * G
    let owner_sig: Signature = witness::OWNER_SIG;
    checksig(owner_pk, owner_sig);
}"#,
    lock_time: 1000,
    sequence: 0,
};

const LAST_WILL: Example = Example {
    description: r#"The inheritor can spend the coins if the owner doesn't move the them for 180 days.
The owner has to repeat the covenant when he moves the coins with his hot key.
The owner can break out of the covenant with his cold key."#,
    program: r#"mod witness {
    const INHERIT_OR_NOT: Either<Signature, Either<Signature, Signature>> =
        Left(0x940cd8a1261a08e677eeefab5d93ba617a92a45c7eaab8cd03c1675b3bd0b1c9bb81ce0301d74fa46fd5cb8402b758462cacfb8b08b9b28e13eb97adb963fa6c);
}

fn checksig(pk: Pubkey, sig: Signature) {
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
    match witness::INHERIT_OR_NOT {
        Left(inheritor_sig: Signature) => inherit_spend(inheritor_sig),
        Right(cold_or_hot: Either<Signature, Signature>) => match cold_or_hot {
            Left(cold_sig: Signature) => cold_spend(cold_sig),
            Right(hot_sig: Signature) => refresh_spend(hot_sig),
        },
    }
}"#,
    lock_time: 0,
    sequence: 25920,
};

const HASH_LOOP: Example = Example {
    description: r#"Test how fast your browser is with this explosive program."#,
    program: r#"mod witness {}

// Add counter to streaming hash and finalize when the loop exists
fn hash_counter_8(ctx: Ctx8, unused: (), byte: u8) -> Either<u256, Ctx8> {
    let new_ctx: Ctx8 = jet::sha_256_ctx_8_add_1(ctx, dbg!(byte));
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
        assert_eq!(
            keys().len(),
            keys().collect::<HashSet<&'static str>>().len()
        );
    }
}
