use std::num::NonZeroUsize;

use elements::secp256k1_zkp as secp256k1;
use hashes::{sha256, Hash};
use hex_conservative::DisplayHex;
use leptos::{
    component, create_rw_signal, use_context, view, For, IntoView, RwSignal, SignalGet, SignalSet,
    SignalUpdate, View,
};
use secp256k1::rand::{self, SeedableRng};
use simfony::elements::secp256k1_zkp::rand::Rng;
use simfony::elements::{self, hashes};
use simfony::num::U256;
use simfony::simplicity::Preimage32;

use crate::components::copy_to_clipboard::CopyToClipboard;

#[derive(Clone, Copy, Debug)]
pub struct HashedData {
    pub random_seed: U256,
    pub hash_count: RwSignal<NonZeroUsize>,
    pub preimages: [Preimage32; 26],
    pub hashes: [sha256::Hash; 26],
}

impl Default for HashedData {
    fn default() -> Self {
        Self::new(U256::from_byte_array(rand::random()), NonZeroUsize::MIN)
    }
}

impl HashedData {
    pub fn new(random_seed: U256, hash_count: NonZeroUsize) -> Self {
        let hash_count = create_rw_signal(hash_count);
        let mut rng = rand::rngs::StdRng::from_seed(random_seed.to_byte_array());
        let preimages = std::array::from_fn(|_| {
            let mut preimage = [0; 32];
            rng.fill(&mut preimage);
            preimage
        });
        let hashes = std::array::from_fn(|index| sha256::Hash::hash(&preimages[index]));
        Self {
            random_seed,
            hash_count,
            preimages,
            hashes,
        }
    }

    pub fn push_hash(&self) {
        let n = self.hash_count.get().get();
        if n < 26 {
            self.hash_count.update(|n| *n = n.saturating_add(1));
        }
    }

    pub fn pop_hash(&self) {
        let n = self.hash_count.get().get();
        if let Some(n_minus_one) = NonZeroUsize::new(n - 1) {
            self.hash_count.set(n_minus_one);
        }
    }
}

#[component]
pub fn HashStoreTab() -> impl IntoView {
    view! {
        <div class="tab-content hash-store-tab">
            <CopyHashesToClipboard />
            <CopyPreimagesToClipboard />
        </div>
    }
}

#[component]
fn CopyHashesToClipboard() -> impl IntoView {
    let hashed_data = use_context::<HashedData>().expect("hashed data should exist in context");
    let copy_single_hash = move |index: usize| -> View {
        let label = format!("Hash {}", index);
        let hash_hex = move || format!("0x{}", hashed_data.hashes[index].to_byte_array().as_hex());

        view! {
            <CopyToClipboard content=hash_hex class="copy-button">
                {label}
                <i class="far fa-copy"></i>
            </CopyToClipboard>
        }
    };

    view! {
        <div>
            <p class="tab-description">
                "The secret preimages are stored in the browser's local storage. Anyone with access to these preimages can sweep your coins."
            </p>

            <div class="tab-title-group">
                <h3 class="tab-title">
                    Hashes
                </h3>

                <div class="button-row is-small">
                    <button
                        class="flat-button bordered"
                        type="button"
                        on:click=move |_| hashed_data.push_hash()
                    >
                        <i class="fas fa-plus"></i>
                        More
                    </button>
                    <button
                        class="flat-button bordered"
                        type="button"
                        on:click=move |_| hashed_data.pop_hash()
                    >
                        <i class="fas fa-minus"></i>
                        Less
                    </button>
                </div>
            </div>

            <div class="button-row is-small">
                <For
                    each=move || 0..hashed_data.hash_count.get().get()
                    key=|index| *index
                    children=copy_single_hash
                />
            </div>
        </div>
    }
}

#[component]
fn CopyPreimagesToClipboard() -> impl IntoView {
    let hashed_data = use_context::<HashedData>().expect("hashed data should exist in context");
    let copy_single_preimage = move |index: usize| -> View {
        let label = format!("Pre {}", index);
        let preimage_hex = move || format!("0x{}", hashed_data.preimages[index].as_hex());

        view! {
            <CopyToClipboard content=preimage_hex class="copy-button">
                {label}
                <i class="far fa-copy"></i>
            </CopyToClipboard>
        }
    };

    view! {
        <div>
            <div class="tab-title-group">
                <h3 class="tab-title">
                    Preimages
                </h3>

                <div class="button-row is-small">
                    <button
                        class="flat-button bordered"
                        type="button"
                        on:click=move |_| hashed_data.push_hash()
                    >
                        <i class="fas fa-plus"></i>
                        More
                    </button>
                    <button
                        class="flat-button bordered"
                        type="button"
                        on:click=move |_| hashed_data.pop_hash()
                    >
                        <i class="fas fa-minus"></i>
                        Less
                    </button>
                </div>
            </div>

            <div class="button-row is-small">
                <For
                    each=move || 0..hashed_data.hash_count.get().get()
                    key=|index| *index
                    children=copy_single_preimage
                />
            </div>
        </div>
    }
}
