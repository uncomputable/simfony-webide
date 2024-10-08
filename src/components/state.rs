use leptos::{create_rw_signal, use_context, SignalGet, SignalGetUntracked};
use leptos_router::ParamsMap;

use crate::components::program_window::ProgramText;
use crate::components::run_window::{HashedData, SigningKeys, TxEnv};

/// [`leptos_router::Params`] with simpler error handling via [`Option`].
pub trait FromParams: Sized {
    /// [`leptos_router::Params::from_map`] that returns `Option<Self>`
    /// instead of `Result<Self, ParamsError>`.
    fn from_map(map: &ParamsMap) -> Option<Self>;
}

pub trait ToParam {
    /// Convert the value into a route parameter and a route value.
    fn to_param(&self) -> (&'static str, String);
}

impl FromParams for ProgramText {
    fn from_map(map: &ParamsMap) -> Option<Self> {
        map.get("program")
            .map(String::as_str)
            .and_then(lz_str::decompress_from_encoded_uri_component)
            .and_then(|v| String::from_utf16(&v).ok())
            .map(create_rw_signal)
            .map(Self)
    }
}

impl ToParam for ProgramText {
    fn to_param(&self) -> (&'static str, String) {
        (
            "program",
            lz_str::compress_to_encoded_uri_component(&self.0.get()),
        )
    }
}

impl FromParams for TxEnv {
    fn from_map(map: &ParamsMap) -> Option<Self> {
        let value = map.get("env").and_then(|s| s.parse::<u64>().ok())?;
        let lock_time = (value >> 32) as u32;
        let sequence = value as u32;
        Some(TxEnv::new(lock_time, sequence))
    }
}

impl ToParam for TxEnv {
    fn to_param(&self) -> (&'static str, String) {
        let lock_time = self.lock_time.get_untracked().to_consensus_u32();
        let sequence = self.sequence.get_untracked().to_consensus_u32();
        let value = ((lock_time as u64) << 32) | (sequence as u64);
        ("env", value.to_string())
    }
}

impl FromParams for SigningKeys {
    fn from_map(map: &ParamsMap) -> Option<Self> {
        map.get("key_count")
            .and_then(|s| s.parse::<u32>().ok())
            .map(Self::new)
    }
}

impl ToParam for SigningKeys {
    fn to_param(&self) -> (&'static str, String) {
        ("key_count", self.key_count.get_untracked().to_string())
    }
}

impl FromParams for HashedData {
    fn from_map(map: &ParamsMap) -> Option<Self> {
        map.get("hash_count")
            .and_then(|s| s.parse::<u32>().ok())
            .map(Self::new)
    }
}

impl ToParam for HashedData {
    fn to_param(&self) -> (&'static str, String) {
        ("hash_count", self.hash_count.get_untracked().to_string())
    }
}

pub fn stateful_url() -> Option<String> {
    web_sys::window().map(|window| {
        let location = window.location();
        let origin = location.origin().unwrap_or_default();
        let pathname = location.pathname().unwrap_or_default();
        let mut url = format!("{}{}", origin, pathname);

        let program = use_context::<ProgramText>().expect("program text should exist in context");
        let tx_env =
            use_context::<TxEnv>().expect("transaction environment should exist in context");
        let signing_keys =
            use_context::<SigningKeys>().expect("signing keys should exist in context");
        let hashed_data = use_context::<HashedData>().expect("hashed data should exist in context");

        let params_values = [
            program.to_param(),
            tx_env.to_param(),
            signing_keys.to_param(),
            hashed_data.to_param(),
        ];

        for (param, value) in params_values {
            url.push('?');
            url.push_str(param);
            url.push('=');
            url.push_str(value.as_str());
        }

        url
    })
}
