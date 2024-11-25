use std::num::NonZeroUsize;

use leptos::{use_context, SignalGetUntracked, SignalWithUntracked};
use simfony::num::U256;
use web_sys::window;

use crate::components::program_window::Program;
use crate::components::run_window::{HashedData, SigningKeys, TxEnv};
use crate::transaction::TxParams;

/// Get the browser's local storage.
fn local_storage() -> Option<web_sys::Storage> {
    let window = window()?;
    window.local_storage().ok().flatten()
}

/// Read / write an object to / from the browser's local storage.
pub trait LocalStorage: Sized {
    /// Iterate over the keys that make up the object.
    fn keys() -> impl Iterator<Item = &'static str>;

    /// Construct an object from the values of the corresponding keys.
    ///
    /// Return `None` if there are not enough values or
    /// if there is an ill-formatted value.
    fn from_values(values: impl Iterator<Item = String>) -> Option<Self>;

    /// Convert an object into its underlying values, in the order of keys.
    fn to_values(&self) -> impl Iterator<Item = String>;

    /// Load an object from the browser's local storage.
    fn load_from_storage() -> Option<Self> {
        let storage = local_storage()?;
        let values = Self::keys().filter_map(|key| storage.get_item(key).ok().flatten());
        Self::from_values(values)
    }

    /// Store an object in the browser's local storage.
    ///
    /// Replaces any existing value.
    fn store_in_storage(&self) {
        let storage = match local_storage() {
            Some(storage) => storage,
            _ => return,
        };
        for (key, value) in Self::keys().zip(self.to_values()) {
            let _result = storage.set_item(key, value.as_str());
        }
    }
}

/// Store the app's entire state in the browser's local storage.
pub fn update_local_storage() {
    use_context::<Program>()
        .expect("program should exist in context")
        .store_in_storage();
    use_context::<TxEnv>()
        .expect("transaction environment should exist in context")
        .params
        .with_untracked(LocalStorage::store_in_storage);
    use_context::<SigningKeys>()
        .expect("signing keys should exist in context")
        .store_in_storage();
    use_context::<HashedData>()
        .expect("hashed data should exist in context")
        .store_in_storage();
    leptos::logging::log!("Update storage");
}

impl LocalStorage for Program {
    fn keys() -> impl Iterator<Item = &'static str> {
        ["program"].into_iter()
    }

    fn from_values(mut values: impl Iterator<Item = String>) -> Option<Self> {
        values.next().map(Self::new)
    }

    fn to_values(&self) -> impl Iterator<Item = String> {
        [self.text.get_untracked()].into_iter()
    }
}

impl LocalStorage for SigningKeys {
    fn keys() -> impl Iterator<Item = &'static str> {
        ["random_seed", "key_count"].into_iter()
    }

    fn from_values(mut values: impl Iterator<Item = String>) -> Option<Self> {
        let random_seed = values.next().and_then(|s| s.parse::<U256>().ok())?;
        let key_count = values.next().and_then(|s| s.parse::<NonZeroUsize>().ok())?;
        Some(Self::new(random_seed, key_count))
    }

    fn to_values(&self) -> impl Iterator<Item = String> {
        [
            self.random_seed.to_string(),
            self.key_count.get_untracked().to_string(),
        ]
        .into_iter()
    }
}

impl LocalStorage for HashedData {
    fn keys() -> impl Iterator<Item = &'static str> {
        ["random_seed", "hash_count"].into_iter()
    }

    fn from_values(mut values: impl Iterator<Item = String>) -> Option<Self> {
        let random_seed = values.next().and_then(|s| s.parse::<U256>().ok())?;
        let hash_count = values.next().and_then(|s| s.parse::<NonZeroUsize>().ok())?;
        Some(Self::new(random_seed, hash_count))
    }

    fn to_values(&self) -> impl Iterator<Item = String> {
        [
            self.random_seed.to_string(),
            self.hash_count.get_untracked().to_string(),
        ]
        .into_iter()
    }
}

impl LocalStorage for TxParams {
    fn keys() -> impl Iterator<Item = &'static str> {
        [
            "txid",
            "vout",
            "value",
            "recipient",
            "fee",
            "lock_time",
            "sequence",
        ]
        .into_iter()
    }

    fn from_values(mut values: impl Iterator<Item = String>) -> Option<Self> {
        let txid = values.next().and_then(|s| s.parse().ok())?;
        let vout = values.next().and_then(|s| s.parse().ok())?;
        let value_in = values.next().and_then(|s| s.parse().ok())?;
        let recipient_address = values.next().and_then(|s| s.parse().ok());
        let fee = values.next().and_then(|s| s.parse().ok())?;
        let lock_time = values.next().and_then(|s| s.parse().ok())?;
        let sequence = values.next().and_then(|s| s.parse().ok())?;

        Some(Self {
            txid,
            vout,
            value_in,
            recipient_address,
            fee,
            lock_time,
            sequence,
        })
    }

    fn to_values(&self) -> impl Iterator<Item = String> {
        [
            self.txid.to_string(),
            self.vout.to_string(),
            self.value_in.to_string(),
            self.recipient_address
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_default(),
            self.fee.to_string(),
            self.lock_time.to_string(),
            self.sequence.to_string(),
        ]
        .into_iter()
    }
}
