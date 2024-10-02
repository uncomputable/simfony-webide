use hashes::{sha256, Hash};
use hex_conservative::DisplayHex;
use leptos::{
    component, create_rw_signal, use_context, view, with, For, IntoView, RwSignal, Signal,
    SignalGet, SignalSet, SignalUpdate, View,
};
use simfony::elements::hashes;

use crate::components::copy_to_clipboard::CopyToClipboard;

#[derive(Clone, Copy, Debug)]
pub struct HashedData {
    pub hash_count: RwSignal<u32>,
    pub preimages: Signal<Vec<[u8; 32]>>,
}

impl Default for HashedData {
    fn default() -> Self {
        Self::new(1)
    }
}

impl HashedData {
    pub fn new(hash_count: u32) -> Self {
        let hash_count = create_rw_signal(hash_count);
        let preimages = Signal::derive(move || -> Vec<[u8; 32]> {
            (0..hash_count.get()).map(new_preimage).collect()
        });
        Self {
            hash_count,
            preimages,
        }
    }

    pub fn push_hash(&self) {
        self.hash_count.update(|n| *n += 1);
    }

    pub fn pop_hash(&self) {
        let n = self.hash_count.get();
        if 1 < n {
            self.hash_count.set(n - 1);
        }
    }

    pub fn hashes(self) -> Signal<Vec<sha256::Hash>> {
        let preimages = self.preimages;
        Signal::derive(move || {
            with!(|preimages| {
                preimages
                    .iter()
                    .map(|preimage| sha256::Hash::hash(preimage))
                    .collect()
            })
        })
    }
}

fn new_preimage(index: u32) -> [u8; 32] {
    let mut preimage = [0; 32];
    preimage[28..].copy_from_slice(&index.to_be_bytes());
    preimage
}

#[component]
pub fn HashStoreTab() -> impl IntoView {
    view! {
        <div>
            <CopyHashesToClipboard />
            <CopyPreimagesToClipboard />
        </div>
    }
}

#[component]
fn CopyHashesToClipboard() -> impl IntoView {
    let hashed_data = use_context::<HashedData>().expect("hashed data should exist in context");
    let copy_single_hash = move |(index, hash): (usize, sha256::Hash)| -> View {
        let label = format!("Hash {}", index);
        let hash_hex = format!("0x{}", hash.to_byte_array().as_hex());

        view! {
            <CopyToClipboard content=hash_hex>
                {label}
                <i class="far fa-copy"></i>
            </CopyToClipboard>
        }
    };

    view! {
        <div>
            <h3 class="program-title">
                Hashes
            </h3>
            <div class="button-row">
                <For
                    each=move || hashed_data.hashes().get().into_iter().enumerate()
                    key=|(_index, hash)| *hash
                    children=copy_single_hash
                />
                <button
                    class="push-button"
                    type="button"
                    on:click=move |_| hashed_data.push_hash()
                >
                    <i class="fas fa-plus"></i>
                    More
                </button>
                <button
                    class="pop-button"
                    type="button"
                    on:click=move |_| hashed_data.pop_hash()
                >
                    <i class="fas fa-minus"></i>
                    Less
                </button>
            </div>
        </div>
    }
}

#[component]
fn CopyPreimagesToClipboard() -> impl IntoView {
    let hashed_data = use_context::<HashedData>().expect("hashed data should exist in context");
    let copy_single_preimage = move |(index, preimage): (usize, [u8; 32])| -> View {
        let label = format!("Pre {}", index);
        let preimage_hex = format!("0x{}", preimage.as_hex());

        view! {
            <CopyToClipboard content=preimage_hex>
                {label}
                <i class="far fa-copy"></i>
            </CopyToClipboard>
        }
    };

    view! {
        <div>
            <h3 class="program-title">
                Preimages
            </h3>
            <div class="button-row">
                <For
                    each=move || hashed_data.preimages.get().into_iter().enumerate()
                    key=|(_index, preimage)| *preimage
                    children=copy_single_preimage
                />
                <button
                    class="push-button"
                    type="button"
                    on:click=move |_| hashed_data.push_hash()
                >
                    <i class="fas fa-plus"></i>
                    More
                </button>
                <button
                    class="pop-button"
                    type="button"
                    on:click=move |_| hashed_data.pop_hash()
                >
                    <i class="fas fa-minus"></i>
                    Less
                </button>
            </div>
        </div>
    }
}