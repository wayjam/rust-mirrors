use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;
use std::sync::Arc;

struct CratesConfig {
    dl: String,
    api: String,
}

struct RustupConfig {
    upstream: String,
    originPrefix: String,
    proxyPrefix: String,
}

#[derive(RustDecodable, RustEncodable)]
struct Config {
    crates: CratesConfig,
    rustup: RustupConfig,
}

static mut CONFIG: Option<Arc<Config>> = None;

impl Config {
    pub fn load(path String) {
        let data = fs::read_to_string(path);
        let c: Config = serde_json::from_str(data).expect("JSON was not well-formatted");
        CONFIG = Some(c);
    }

    pub fn get_instance() -> &'static Config {
        unsafe {
            CONFIG.as_ref.unwrap()
        }
    }
}
