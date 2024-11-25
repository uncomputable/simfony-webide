use std::collections::HashMap;

use crate::transaction::TxParams;
use elements::hashes::{sha256, Hash};
use elements::secp256k1_zkp as secp256k1;
use simfony::elements::hashes::HashEngine;
use simfony::num::U256;
use simfony::simplicity::Preimage32;
use simfony::str::WitnessName;
use simfony::types::TypeConstructible;
use simfony::value::ValueConstructible;
use simfony::{elements, ResolvedType, Value};

#[derive(Clone, Copy, Debug)]
pub struct Example {
    description: &'static str,
    program: &'static str,
    compute_args: fn(&[secp256k1::XOnlyPublicKey], &[sha256::Hash]) -> simfony::Arguments,
    compute_witness:
        fn(&[secp256k1::Keypair], &[Preimage32], secp256k1::Message) -> simfony::WitnessValues,
    lock_time: u32,
    sequence: u32,
}

impl Example {
    #[allow(dead_code)]
    pub fn description(self) -> &'static str {
        self.description
    }

    pub fn template_text(self) -> &'static str {
        self.program
    }

    pub fn arguments(
        self,
        public_keys: &[secp256k1::XOnlyPublicKey],
        hashes: &[sha256::Hash],
    ) -> simfony::Arguments {
        (self.compute_args)(public_keys, hashes)
    }

    pub fn witness(
        self,
        secret_keys: &[secp256k1::Keypair],
        preimages: &[Preimage32],
        sighash_all: secp256k1::Message,
    ) -> simfony::WitnessValues {
        (self.compute_witness)(secret_keys, preimages, sighash_all)
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

fn p2pk_args(
    public_keys: &[secp256k1::XOnlyPublicKey],
    _hashes: &[sha256::Hash],
) -> simfony::Arguments {
    simfony::Arguments::from(HashMap::from([(
        WitnessName::from_str_unchecked("ALICE_PUBLIC_KEY"),
        Value::u256(U256::from_byte_array(public_keys[0].serialize())),
    )]))
}

fn p2pk_witness(
    secret_keys: &[secp256k1::Keypair],
    _preimages: &[Preimage32],
    sighash_all: secp256k1::Message,
) -> simfony::WitnessValues {
    simfony::WitnessValues::from(HashMap::from([(
        WitnessName::from_str_unchecked("ALICE_SIGNATURE"),
        Value::byte_array(secret_keys[0].sign_schnorr(sighash_all).serialize()),
    )]))
}

const P2PK: Example = Example {
    description: r#"Pay to public key.

The coins move if the person with the given public key signs the transaction."#,
    program: r#"fn main() {
    jet::bip_0340_verify((param::ALICE_PUBLIC_KEY, jet::sig_all_hash()), witness::ALICE_SIGNATURE)
}"#,
    compute_args: p2pk_args,
    compute_witness: p2pk_witness,
    lock_time: 0,
    sequence: 0,
};

fn p2pkh_args(
    public_keys: &[secp256k1::XOnlyPublicKey],
    _hashes: &[sha256::Hash],
) -> simfony::Arguments {
    let pk_hash = sha256::Hash::hash(&public_keys[0].serialize());
    simfony::Arguments::from(HashMap::from([(
        WitnessName::from_str_unchecked("ALICE_PUBLIC_KEY_HASH"),
        Value::u256(U256::from_byte_array(pk_hash.to_byte_array())),
    )]))
}

fn p2pkh_witness(
    secret_keys: &[secp256k1::Keypair],
    _preimages: &[Preimage32],
    sighash_all: secp256k1::Message,
) -> simfony::WitnessValues {
    let alice_pk = secret_keys[0].x_only_public_key().0;
    simfony::WitnessValues::from(HashMap::from([
        (
            WitnessName::from_str_unchecked("ALICE_PUBLIC_KEY"),
            Value::u256(U256::from_byte_array(alice_pk.serialize())),
        ),
        (
            WitnessName::from_str_unchecked("ALICE_SIGNATURE"),
            Value::byte_array(secret_keys[0].sign_schnorr(sighash_all).serialize()),
        ),
    ]))
}

const P2PKH: Example = Example {
    description: r#"Pay to public key hash.

The coins move if the person with the public key that matches the given hash signs the transaction."#,
    program: r#"fn sha2(string: u256) -> u256 {
    let hasher: Ctx8 = jet::sha_256_ctx_8_init();
    let hasher: Ctx8 = jet::sha_256_ctx_8_add_32(hasher, string);
    jet::sha_256_ctx_8_finalize(hasher)
}

fn main() {
    let pk: Pubkey = witness::ALICE_PUBLIC_KEY;
    let expected_pk_hash: u256 = param::ALICE_PUBLIC_KEY_HASH;
    let pk_hash: u256 = sha2(pk);
    assert!(jet::eq_256(pk_hash, expected_pk_hash));

    let msg: u256 = jet::sig_all_hash();
    jet::bip_0340_verify((pk, msg), witness::ALICE_SIGNATURE)
}"#,
    compute_args: p2pkh_args,
    compute_witness: p2pkh_witness,
    lock_time: 0,
    sequence: 0,
};

fn p2ms_args(
    public_keys: &[secp256k1::XOnlyPublicKey],
    _hashes: &[sha256::Hash],
) -> simfony::Arguments {
    simfony::Arguments::from(HashMap::from([
        (
            WitnessName::from_str_unchecked("ALICE_PUBLIC_KEY"),
            Value::u256(U256::from_byte_array(public_keys[0].serialize())),
        ),
        (
            WitnessName::from_str_unchecked("BOB_PUBLIC_KEY"),
            Value::u256(U256::from_byte_array(public_keys[1].serialize())),
        ),
        (
            WitnessName::from_str_unchecked("CHARLIE_PUBLIC_KEY"),
            Value::u256(U256::from_byte_array(public_keys[2].serialize())),
        ),
    ]))
}

fn p2ms_witness(
    secret_keys: &[secp256k1::Keypair],
    _preimages: &[Preimage32],
    sighash_all: secp256k1::Message,
) -> simfony::WitnessValues {
    let alice_sig = Value::some(Value::byte_array(
        secret_keys[0].sign_schnorr(sighash_all).serialize(),
    ));
    let bob_sig = Value::none(ResolvedType::byte_array(64));
    let charlie_sig = Value::some(Value::byte_array(
        secret_keys[2].sign_schnorr(sighash_all).serialize(),
    ));
    let ty = alice_sig.ty().clone();
    let signatures = Value::array([alice_sig, bob_sig, charlie_sig], ty);
    simfony::WitnessValues::from(HashMap::from([(
        WitnessName::from_str_unchecked("SIGNATURES_2_OF_3"),
        signatures,
    )]))
}

const P2MS: Example = Example {
    description: r#"Pay to multisig.

The coins move if 2 of 3 people agree to move them. These people provide their signatures, of which exactly 2 are required."#,
    program: r#"fn not(bit: bool) -> bool {
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
        param::ALICE_PUBLIC_KEY,
        param::BOB_PUBLIC_KEY,
        param::CHARLIE_PUBLIC_KEY,
    ];
    check2of3multisig(pks, witness::SIGNATURES_2_OF_3);
}"#,
    compute_args: p2ms_args,
    compute_witness: p2ms_witness,
    lock_time: 0,
    sequence: 0,
};

/*
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
*/

fn htlc_args(
    public_keys: &[secp256k1::XOnlyPublicKey],
    hashes: &[sha256::Hash],
) -> simfony::Arguments {
    simfony::Arguments::from(HashMap::from([
        (
            WitnessName::from_str_unchecked("ALICE_PUBLIC_KEY"),
            Value::u256(U256::from_byte_array(public_keys[0].serialize())),
        ),
        (
            WitnessName::from_str_unchecked("BOB_PUBLIC_KEY"),
            Value::u256(U256::from_byte_array(public_keys[1].serialize())),
        ),
        (
            WitnessName::from_str_unchecked("EXPECTED_HASH"),
            Value::u256(U256::from_byte_array(hashes[0].to_byte_array())),
        ),
    ]))
}

fn htlc_witness(
    secret_keys: &[secp256k1::Keypair],
    preimages: &[Preimage32],
    sighash_all: secp256k1::Message,
) -> simfony::WitnessValues {
    let alice_sig = secret_keys[0].sign_schnorr(sighash_all);
    let complete_or_cancel = Value::left(
        Value::product(
            Value::u256(U256::from_byte_array(preimages[0])),
            Value::byte_array(alice_sig.serialize()),
        ),
        ResolvedType::byte_array(64),
    );
    simfony::WitnessValues::from(HashMap::from([(
        WitnessName::from_str_unchecked("COMPLETE_OR_CANCEL"),
        complete_or_cancel,
    )]))
}

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
    assert!(jet::eq_256(hash, param::EXPECTED_HASH));
    let recipient_pk: Pubkey = param::ALICE_PUBLIC_KEY;
    checksig(recipient_pk, recipient_sig);
}

fn cancel_spend(sender_sig: Signature) {
    let timeout: Height = 1000;
    jet::check_lock_height(timeout);
    let sender_pk: Pubkey = param::BOB_PUBLIC_KEY;
    checksig(sender_pk, sender_sig)
}

fn main() {
    match witness::COMPLETE_OR_CANCEL {
        Left(preimage_and_sig: (u256, Signature)) => {
            let (preimage, recipient_sig): (u256, Signature) = preimage_and_sig;
            complete_spend(preimage, recipient_sig);
        },
        Right(sender_sig: Signature) => cancel_spend(sender_sig),
    }
}"#,
    compute_args: htlc_args,
    compute_witness: htlc_witness,
    lock_time: 0,
    sequence: 0,
};

fn hodl_vault_args(
    public_keys: &[secp256k1::XOnlyPublicKey],
    _hashes: &[sha256::Hash],
) -> simfony::Arguments {
    simfony::Arguments::from(HashMap::from([
        (
            WitnessName::from_str_unchecked("MIN_HEIGHT"),
            Value::u32(1000),
        ),
        (
            WitnessName::from_str_unchecked("TARGET_PRICE"),
            Value::u32(100_000),
        ),
        (
            WitnessName::from_str_unchecked("ALICE_PUBLIC_KEY"),
            Value::u256(U256::from_byte_array(public_keys[0].serialize())),
        ),
        (
            WitnessName::from_str_unchecked("BOB_PUBLIC_KEY"),
            Value::u256(U256::from_byte_array(public_keys[1].serialize())),
        ),
    ]))
}

fn hodl_vault_witness(
    secret_keys: &[secp256k1::Keypair],
    _preimages: &[Preimage32],
    sighash_all: secp256k1::Message,
) -> simfony::WitnessValues {
    let mut witness_values = HashMap::new();
    let oracle_height = 1000;
    witness_values.insert(
        WitnessName::from_str_unchecked("ORACLE_HEIGHT"),
        Value::u32(oracle_height),
    );
    let oracle_price = 100_000;
    witness_values.insert(
        WitnessName::from_str_unchecked("ORACLE_PRICE"),
        Value::u32(oracle_price),
    );
    let mut hasher = sha256::HashEngine::default();
    hasher.input(&oracle_height.to_be_bytes());
    hasher.input(&oracle_price.to_be_bytes());
    let oracle_hash =
        secp256k1::Message::from_digest(sha256::Hash::from_engine(hasher).to_byte_array());
    witness_values.insert(
        WitnessName::from_str_unchecked("ALICE_SIGNATURE"),
        Value::byte_array(secret_keys[0].sign_schnorr(oracle_hash).serialize()),
    );
    witness_values.insert(
        WitnessName::from_str_unchecked("BOB_SIGNATURE"),
        Value::byte_array(secret_keys[1].sign_schnorr(sighash_all).serialize()),
    );
    simfony::WitnessValues::from(witness_values)
}

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
    let msg: u256 = dbg!(jet::sha_256_ctx_8_finalize(hasher));
    jet::bip_0340_verify((pk, msg), sig);
}

fn main() {
    let oracle_height: Height = witness::ORACLE_HEIGHT;
    assert!(jet::le_32(param::MIN_HEIGHT, oracle_height));
    jet::check_lock_height(oracle_height);

    let oracle_price: u32 = witness::ORACLE_PRICE;
    assert!(jet::le_32(param::TARGET_PRICE, oracle_price));

    let oracle_pk: Pubkey = param::ALICE_PUBLIC_KEY;
    let oracle_sig: Signature = witness::ALICE_SIGNATURE;
    checksigfromstack(oracle_pk, [oracle_height, oracle_price], oracle_sig);

    let owner_pk: Pubkey = param::BOB_PUBLIC_KEY;
    let owner_sig: Signature = witness::BOB_SIGNATURE;
    checksig(owner_pk, owner_sig);
}"#,
    compute_args: hodl_vault_args,
    compute_witness: hodl_vault_witness,
    lock_time: 1000,
    sequence: 0,
};

fn last_will_args(
    public_keys: &[secp256k1::XOnlyPublicKey],
    _hashes: &[sha256::Hash],
) -> simfony::Arguments {
    simfony::Arguments::from(HashMap::from([
        (
            WitnessName::from_str_unchecked("ALICE_PUBLIC_KEY"),
            Value::u256(U256::from_byte_array(public_keys[0].serialize())),
        ),
        (
            WitnessName::from_str_unchecked("BOB_PUBLIC_KEY"),
            Value::u256(U256::from_byte_array(public_keys[1].serialize())),
        ),
        (
            WitnessName::from_str_unchecked("CHARLIE_PUBLIC_KEY"),
            Value::u256(U256::from_byte_array(public_keys[2].serialize())),
        ),
    ]))
}

fn last_will_witness(
    secret_keys: &[secp256k1::Keypair],
    _preimages: &[Preimage32],
    sighash_all: secp256k1::Message,
) -> simfony::WitnessValues {
    let alice_sig = Value::byte_array(secret_keys[0].sign_schnorr(sighash_all).serialize());
    let inherit_or_not = Value::left(
        alice_sig,
        ResolvedType::either(ResolvedType::byte_array(64), ResolvedType::byte_array(64)),
    );
    simfony::WitnessValues::from(HashMap::from([(
        WitnessName::from_str_unchecked("INHERIT_OR_NOT"),
        inherit_or_not,
    )]))
}

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
    let inheritor_pk: Pubkey = param::ALICE_PUBLIC_KEY;
    checksig(inheritor_pk, inheritor_sig);
}

fn cold_spend(cold_sig: Signature) {
    let cold_pk: Pubkey = param::BOB_PUBLIC_KEY;
    checksig(cold_pk, cold_sig);
}

fn refresh_spend(hot_sig: Signature) {
    let hot_pk: Pubkey = param::CHARLIE_PUBLIC_KEY;
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
    compute_args: last_will_args,
    compute_witness: last_will_witness,
    lock_time: 0,
    sequence: 25920,
};

fn empty_args(
    _public_keys: &[secp256k1::XOnlyPublicKey],
    _hashes: &[sha256::Hash],
) -> simfony::Arguments {
    simfony::Arguments::default()
}

fn empty_witness(
    _secret_keys: &[secp256k1::Keypair],
    _preimages: &[Preimage32],
    _sighash_all: secp256k1::Message,
) -> simfony::WitnessValues {
    simfony::WitnessValues::default()
}

const HASH_LOOP: Example = Example {
    description: r#"Test how fast your browser is with this explosive program."#,
    program: r#"// Add counter to streaming hash and finalize when the loop exists
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
    compute_args: empty_args,
    compute_witness: empty_witness,
    lock_time: 0,
    sequence: 0,
};

/// Names must be unique because they serve as primary keys.
const EXAMPLES: [(&str, Example); 7] = [
    ("✍️️ P2PK", P2PK),
    ("✍️ P2PKH", P2PKH),
    ("✍️ P2MS", P2MS),
    // ("✍️ SIGHASH_ANYPREVOUT", SIGHASH_ANYPREVOUT),
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
