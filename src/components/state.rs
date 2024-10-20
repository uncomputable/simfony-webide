use leptos::{use_context, SignalGetUntracked, SignalWith};
use leptos_router::ParamsMap;

use crate::components::program_window::Program;
use crate::components::run_window::{HashedData, SigningKeys};

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

impl FromParams for Program {
    fn from_map(map: &ParamsMap) -> Option<Self> {
        map.get("program")
            .map(String::as_str)
            .and_then(lz_str::decompress_from_encoded_uri_component)
            .and_then(|v| String::from_utf16(&v).ok())
            .map(Self::new)
    }
}

impl ToParam for Program {
    fn to_param(&self) -> (&'static str, String) {
        self.text
            .with(|text| ("program", lz_str::compress_to_encoded_uri_component(text)))
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

        let program = use_context::<Program>().expect("program should exist in context");
        let signing_keys =
            use_context::<SigningKeys>().expect("signing keys should exist in context");
        let hashed_data = use_context::<HashedData>().expect("hashed data should exist in context");

        let params_values = [
            program.to_param(),
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
