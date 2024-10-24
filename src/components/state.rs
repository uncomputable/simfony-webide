use std::num::NonZeroU32;

use leptos::{use_context, SignalGetUntracked};
use leptos_router::ParamsMap;

use crate::components::run_window::{HashedData, SigningKeys, TxEnv};
use crate::transaction::TxParams;

/// [`leptos_router::Params`] with simpler error handling via [`Option`].
pub trait FromParams: Sized {
    /// [`leptos_router::Params::from_map`] that returns `Option<Self>`
    /// instead of `Result<Self, ParamsError>`.
    fn from_map(map: &ParamsMap) -> Option<Self>;
}

pub trait ToParams {
    /// Convert the value into route parameters and route values.
    fn to_params(&self) -> impl Iterator<Item = (&'static str, String)>;
}

impl FromParams for SigningKeys {
    fn from_map(map: &ParamsMap) -> Option<Self> {
        let key_offset = map.get("seed").and_then(|s| s.parse::<u32>().ok())?;
        let key_count = map.get("keys").and_then(|s| s.parse::<NonZeroU32>().ok())?;
        Some(Self::new(key_offset, key_count))
    }
}

impl ToParams for SigningKeys {
    fn to_params(&self) -> impl Iterator<Item = (&'static str, String)> {
        [
            ("seed", self.key_offset.get_untracked().to_string()),
            ("keys", self.key_count.get_untracked().to_string()),
        ]
        .into_iter()
    }
}

impl FromParams for HashedData {
    fn from_map(map: &ParamsMap) -> Option<Self> {
        map.get("hashes")
            .and_then(|s| s.parse::<u32>().ok())
            .map(Self::new)
    }
}

impl ToParams for HashedData {
    fn to_params(&self) -> impl Iterator<Item = (&'static str, String)> {
        [("hashes", self.hash_count.get_untracked().to_string())].into_iter()
    }
}

impl FromParams for TxParams {
    fn from_map(map: &ParamsMap) -> Option<Self> {
        let txid = map.get("txid").and_then(|s| s.parse().ok())?;
        let vout = map.get("vout").and_then(|s| s.parse().ok())?;
        let value_in = map.get("value").and_then(|s| s.parse().ok())?;
        let recipient_address = map.get("recipient").and_then(|s| s.parse().ok());
        let fee = map.get("fee").and_then(|s| s.parse().ok())?;
        let lock_time = map.get("lock_time").and_then(|s| s.parse().ok())?;
        let sequence = map.get("sequence").and_then(|s| s.parse().ok())?;

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
}

impl ToParams for TxParams {
    fn to_params(&self) -> impl Iterator<Item = (&'static str, String)> {
        let mut params = vec![
            ("txid", self.txid.to_string()),
            ("vout", self.vout.to_string()),
            ("value", self.value_in.to_string()),
            ("fee", self.fee.to_string()),
            ("lock_time", self.lock_time.to_string()),
            ("sequence", self.sequence.to_string()),
        ];
        if let Some(address) = &self.recipient_address {
            params.push(("recipient", address.to_string()));
        }
        params.into_iter()
    }
}

pub fn stateful_url() -> Option<String> {
    web_sys::window().map(|window| {
        let location = window.location();
        let origin = location.origin().unwrap_or_default();
        let pathname = location.pathname().unwrap_or_default();
        let mut url = format!("{}{}", origin, pathname);

        let tx_params = use_context::<TxEnv>()
            .expect("transaction environment should exist in context")
            .params
            .get_untracked();
        let signing_keys =
            use_context::<SigningKeys>().expect("signing keys should exist in context");
        let hashed_data = use_context::<HashedData>().expect("hashed data should exist in context");

        for (param, value) in tx_params
            .to_params()
            .chain(signing_keys.to_params())
            .chain(hashed_data.to_params())
        {
            url.push('?');
            url.push_str(param);
            url.push('=');
            url.push_str(value.as_str());
        }

        url
    })
}
