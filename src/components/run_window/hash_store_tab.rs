use hashes::Hash;
use hex_conservative::DisplayHex;
use leptos::{
    component, create_rw_signal, use_context, view, For, IntoView, RwSignal, SignalGet,
    SignalUpdate, View,
};
use simfony::elements::hashes;

use crate::components::copy_to_clipboard::CopyToClipboard;
use crate::util::{Counter26, HashedData};

#[derive(Copy, Clone, Debug, Default)]
pub struct HashCount(pub RwSignal<Counter26>);

impl HashCount {
    pub fn new(n: Counter26) -> Self {
        Self(create_rw_signal(n))
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
    let hash_count = use_context::<HashCount>().expect("hash count should exist in context");
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
                        on:click=move |_| hash_count.0.update(Counter26::saturating_increment)
                    >
                        <i class="fas fa-plus"></i>
                        More
                    </button>
                    <button
                        class="flat-button bordered"
                        type="button"
                        on:click=move |_| hash_count.0.update(Counter26::saturating_decrement)
                    >
                        <i class="fas fa-minus"></i>
                        Less
                    </button>
                </div>
            </div>

            <div class="button-row is-small">
                <For
                    each=move || 0..hash_count.0.get().get()
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
    let hash_count = use_context::<HashCount>().expect("hash count should exist in context");
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
                        on:click=move |_| hash_count.0.update(Counter26::saturating_increment)
                    >
                        <i class="fas fa-plus"></i>
                        More
                    </button>
                    <button
                        class="flat-button bordered"
                        type="button"
                        on:click=move |_| hash_count.0.update(Counter26::saturating_decrement)
                    >
                        <i class="fas fa-minus"></i>
                        Less
                    </button>
                </div>
            </div>

            <div class="button-row is-small">
                <For
                    each=move || 0..hash_count.0.get().get()
                    key=|index| *index
                    children=copy_single_preimage
                />
            </div>
        </div>
    }
}
