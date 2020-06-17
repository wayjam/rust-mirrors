extern crate serde_json;

use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Default, Debug)]
struct CratesConfig {
    dl: String,
    api: String,
}

#[derive(Deserialize, Default, Debug)]
struct RustupConfig {
    upstream: String,
    origin_prefix: String,
    proxy_prefix: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct Config {
    #[serde(default)]
    pub debug: bool,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub port: u16,
    #[serde(default)]
    crates: CratesConfig,
    #[serde(default)]
    rustup: RustupConfig,
}

impl Config {
    pub fn from_file(path: &str) -> Config {
        if fs::metadata(path).is_ok() {
            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);

            return serde_json::from_reader(reader).expect("JSON was not well-formatted")
        } else {
            return Config {
                debug: true,
                host: String::from("127.0.0.1"),
                port: 8080,
                ..Config::default()
            };
        }
    }
}
