use std::sync::Arc;

use elements::hashes;
use elements::secp256k1_zkp;
use hex_conservative::{DisplayHex, FromHex};
use leptos::{
    component, create_rw_signal, ev, event_target_value, html, use_context, view, with, For,
    IntoView, NodeRef, RwSignal, Signal, SignalGet, SignalGetUntracked, SignalSet, SignalUpdate,
    View,
};
use simfony::{elements, simplicity};

use crate::components::copy_to_clipboard::CopyToClipboard;

#[derive(Copy, Clone, Debug)]
pub struct SigningKeys {
    key_count: RwSignal<u32>,
    secret_keys: Signal<Vec<secp256k1_zkp::Keypair>>,
}

impl SigningKeys {
    pub fn new(key_count: u32) -> Self {
        let key_count = create_rw_signal(key_count);
        let secret_keys = Signal::derive(move || -> Vec<secp256k1_zkp::Keypair> {
            let mut index = 0;
            (0..key_count.get())
                .map(|_| {
                    let (key, new_index) = new_key(index);
                    index = new_index;
                    key
                })
                .collect()
        });
        Self {
            key_count,
            secret_keys,
        }
    }

    pub fn push_key(&self) {
        self.key_count.update(|n| *n += 1);
    }

    pub fn pop_key(&self) {
        let n = self.key_count.get();
        if 1 < n {
            self.key_count.set(n - 1);
        }
    }

    pub fn public_keys(self) -> Signal<Vec<secp256k1_zkp::XOnlyPublicKey>> {
        let secret_keys = self.secret_keys;
        Signal::derive(move || {
            with!(|secret_keys| {
                secret_keys
                    .iter()
                    .map(|key| key.x_only_public_key().0)
                    .collect()
            })
        })
    }

    pub fn signatures(
        self,
        message: Signal<secp256k1_zkp::Message>,
    ) -> Signal<Vec<secp256k1_zkp::schnorr::Signature>> {
        let secret_keys = self.secret_keys;
        Signal::derive(move || {
            with!(|secret_keys| {
                secret_keys
                    .iter()
                    .map(|key| key.sign_schnorr(message.get()))
                    .collect()
            })
        })
    }
}

fn new_key(start_index: u32) -> (secp256k1_zkp::Keypair, u32) {
    let mut offset = 1;
    loop {
        let index = start_index + offset;
        let mut secret_key_bytes = [0u8; 32];
        secret_key_bytes[28..].copy_from_slice(&index.to_be_bytes());
        match secp256k1_zkp::Keypair::from_seckey_slice(secp256k1_zkp::SECP256K1, &secret_key_bytes)
        {
            Ok(keypair) => return (keypair, index),
            Err(..) => {
                offset += 1;
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum SignedDataMode {
    SighashAll,
    ThirtyTwoBytes,
    HashPreimageBytes,
}

#[derive(Clone, Copy, Debug)]
pub struct SignedData {
    mode: RwSignal<SignedDataMode>,
    sighash_all: Signal<hashes::sha256::Hash>,
    thirty_two_bytes: RwSignal<[u8; 32]>,
    hash_preimage_bytes: RwSignal<Vec<u8>>,
}

impl SignedData {
    pub fn new(
        tx_env: Signal<simplicity::jet::elements::ElementsEnv<Arc<elements::Transaction>>>,
    ) -> Self {
        let sighash_all =
            Signal::derive(move || with!(|tx_env| { tx_env.c_tx_env().sighash_all() }));
        Self {
            mode: create_rw_signal(SignedDataMode::SighashAll),
            sighash_all,
            thirty_two_bytes: create_rw_signal([0; 32]),
            hash_preimage_bytes: create_rw_signal(vec![]),
        }
    }

    pub fn message(self) -> Signal<secp256k1_zkp::Message> {
        Signal::derive(move || match self.mode.get() {
            SignedDataMode::SighashAll => self.sighash_all.get().into(),
            SignedDataMode::ThirtyTwoBytes => {
                secp256k1_zkp::Message::from_digest(self.thirty_two_bytes.get())
            }
            SignedDataMode::HashPreimageBytes => {
                secp256k1_zkp::Message::from_hashed_data::<hashes::sha256::Hash>(
                    self.hash_preimage_bytes.get().as_ref(),
                )
            }
        })
    }
}

#[component]
pub fn KeyStoreTab() -> impl IntoView {
    view! {
        <div>
            <CopyPublicKeysToClipboard />
            <CopySignaturesToClipboard/>
            <SelectSignedData />
        </div>
    }
}

#[component]
fn CopyPublicKeysToClipboard() -> impl IntoView {
    let signing_keys = use_context::<SigningKeys>().expect("signing keys should exist in context");
    let copy_single_public_key =
        move |(index, key): (usize, secp256k1_zkp::XOnlyPublicKey)| -> View {
            let label = format!("Key {}", index);
            let xonly_hex = format!("0x{}", key.serialize().as_hex());

            view! {
                <CopyToClipboard label=label content=xonly_hex />
            }
        };

    view! {
        <div>
            <h3 class="program-title">
                Public Keys
            </h3>
            <div class="button-row">
                <For
                    each=move || signing_keys.public_keys().get().into_iter().enumerate()
                    key=|(_index, key)| *key
                    children=copy_single_public_key
                />
                <button
                    class="push-button"
                    type="button"
                    on:click=move |_| signing_keys.push_key()
                >
                    <i class="fas fa-plus"></i>
                    More
                </button>
                <button
                    class="pop-button"
                    type="button"
                    on:click=move |_| signing_keys.pop_key()
                >
                    <i class="fas fa-minus"></i>
                    Less
                </button>
            </div>
        </div>
    }
}

#[component]
fn CopySignaturesToClipboard() -> impl IntoView {
    let signing_keys = use_context::<SigningKeys>().expect("signing keys should exist in context");
    let signed_data = use_context::<SignedData>().expect("signed data should exist in context");

    let copy_single_signature =
        move |(index, signature): (usize, secp256k1_zkp::schnorr::Signature)| -> View {
            let label = format!("Sig {}", index);
            let signature_hex = format!("0x{}", signature.serialize().as_hex());

            view! {
                <CopyToClipboard label=label content=signature_hex />
            }
        };

    view! {
        <div>
            <h3 class="program-title">
                Signatures
            </h3>
            <div class="button-row">
                <For
                    each=move || signing_keys.signatures(signed_data.message()).get().into_iter().enumerate()
                    key=|(_index, signature)| *signature
                    children=copy_single_signature
                />
                <button
                    class="push-button"
                    type="button"
                    on:click=move |_| signing_keys.push_key()
                >
                    <i class="fas fa-plus"></i>
                    More
                </button>
                <button
                    class="pop-button"
                    type="button"
                    on:click=move |_| signing_keys.pop_key()
                >
                    <i class="fas fa-minus"></i>
                    Less
                </button>
            </div>
        </div>
    }
}

#[component]
fn SelectSignedData() -> impl IntoView {
    let signed_data = use_context::<SignedData>().expect("signed data should exist in context");
    let thirty_two_bytes_is_insane = create_rw_signal(false);
    let hash_preimage_bytes_is_insane = create_rw_signal(false);

    let sighash_all_initial_checked =
        matches!(signed_data.mode.get_untracked(), SignedDataMode::SighashAll);
    let thirty_two_bytes_initial_checked = matches!(
        signed_data.mode.get_untracked(),
        SignedDataMode::ThirtyTwoBytes
    );
    let hash_preimage_bytes_initial_checked = matches!(
        signed_data.mode.get_untracked(),
        SignedDataMode::HashPreimageBytes
    );
    let thirty_two_bytes_initial_value = format!(
        "0x{}",
        signed_data.thirty_two_bytes.get_untracked().as_hex()
    );
    let hash_preimage_bytes_initial_value = format!(
        "0x{}",
        signed_data.hash_preimage_bytes.get_untracked().as_hex()
    );

    let sighash_all_radio_ref = NodeRef::<html::Input>::new();
    let thirty_two_bytes_text_ref = NodeRef::<html::Input>::new();
    let hash_preimage_bytes_text_ref = NodeRef::<html::Input>::new();

    let select_sighash_all = move |_event: ev::Event| {
        signed_data.mode.set(SignedDataMode::SighashAll);
    };
    let select_thirty_two_bytes = move |_event: ev::Event| {
        signed_data.mode.set(SignedDataMode::ThirtyTwoBytes);
    };
    let select_hash_preimage_bytes = move |_event: ev::Event| {
        signed_data.mode.set(SignedDataMode::HashPreimageBytes);
    };
    let update_thirty_two_bytes = move |event: ev::Event| match <[u8; 32]>::from_hex(
        event_target_value(&event)
            .as_str()
            .trim()
            .trim_start_matches("0x"),
    ) {
        Ok(bytes) => {
            signed_data.thirty_two_bytes.set(bytes);
            thirty_two_bytes_text_ref
                .get()
                .expect("<input> should be mounted")
                .set_custom_validity("");
            thirty_two_bytes_is_insane.set(false);
        }
        Err(..) => {
            sighash_all_radio_ref
                .get()
                .expect("<input> should be mounted")
                .set_checked(true);
            thirty_two_bytes_text_ref
                .get()
                .expect("<input> should be mounted")
                .set_custom_validity("Expected exactly 64 hex digits");
            thirty_two_bytes_is_insane.set(true);
        }
    };
    let update_hash_preimage_bytes = move |event: ev::Event| match <Vec<u8>>::from_hex(
        event_target_value(&event)
            .as_str()
            .trim()
            .trim_start_matches("0x"),
    ) {
        Ok(bytes) => {
            signed_data.hash_preimage_bytes.set(bytes);
            hash_preimage_bytes_text_ref
                .get()
                .expect("<input> should be mounted")
                .set_custom_validity("");
            hash_preimage_bytes_is_insane.set(false);
        }
        Err(..) => {
            sighash_all_radio_ref
                .get()
                .expect("<input> should be mounted")
                .set_checked(true);
            hash_preimage_bytes_text_ref
                .get()
                .expect("<input> should be mounted")
                .set_custom_validity("Expected even number of hex digits");
            hash_preimage_bytes_is_insane.set(true);
        }
    };

    view! {
        <div>
            <h3 class="program-title">
                Signed Data
            </h3>
            <fieldset class="button-col">
                <label>
                    <input
                        type="radio"
                        name="signed_data"
                        checked=sighash_all_initial_checked
                        on:change=select_sighash_all
                        node_ref=sighash_all_radio_ref
                    />
                    SIGHASH_ALL
                </label>
                <label>
                    <input
                        type="radio"
                        name="signed_data"
                        checked=thirty_two_bytes_initial_checked
                        on:change=select_thirty_two_bytes
                        disabled=thirty_two_bytes_is_insane
                    />
                    raw bytes
                    <input
                        type="text"
                        placeholder="Enter hex bytes"
                        on:input=update_thirty_two_bytes
                        value=thirty_two_bytes_initial_value
                        node_ref=thirty_two_bytes_text_ref
                    />
                </label>
                <label>
                    <input
                        type="radio"
                        name="signed_data"
                        checked=hash_preimage_bytes_initial_checked
                        on:change=select_hash_preimage_bytes
                        disabled=hash_preimage_bytes_is_insane
                    />
                    byte hash
                    <input
                        type="text"
                        placeholder="Enter hex bytes"
                        on:input=update_hash_preimage_bytes
                        value=hash_preimage_bytes_initial_value
                        node_ref=hash_preimage_bytes_text_ref
                    />
                </label>
            </fieldset>
        </div>
    }
}
