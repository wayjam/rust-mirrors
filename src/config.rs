extern crate serde_json;

use serde::Deserialize;
use std::cell::Cell;
use std::cell::RefCell;
use std::fs;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Default, Debug)]
struct CratesConfig {
    pub dl: String,
    pub api: String,
}

#[derive(Deserialize, Default, Debug)]
struct RustupConfig {
    pub upstream: String,
    pub origin_prefix: String,
    pub proxy_prefix: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct Config {
    #[serde(default)]
    pub debug: Cell<bool>,
    #[serde(default)]
    host: RefCell<String>,
    #[serde(default)]
    pub port: Cell<u16>,
    #[serde(default)]
    pub crates: CratesConfig,
    #[serde(default)]
    pub rustup: RustupConfig,
}

impl Config {
    pub fn from_file(path: &str) -> Self {
        if fs::metadata(path).is_ok() {
            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);

            return serde_json::from_reader(reader).expect("JSON was not well-formatted");
        } else {
            return Config {
                debug: Cell::new(true),
                host: RefCell::new("127.0.0.1".to_string()),
                port: Cell::new(8080),
                ..Config::default()
            };
        }
    }

    pub fn set_debug(self, debug: bool) -> Self {
        self.debug.set(debug);
        self
    }

    pub fn set_host(self, host: &str) -> Self {
        *self.host.borrow_mut() = host.to_string();
        self
    }

    pub fn set_port(self, port: u16) -> Self {
        self.port.set(port);
        self
    }

    pub fn get_host(self) -> &'static str {
        let x = self.host.borrow();
        return "asd";
    }

    pub fn get_port(self) -> u16 {
        self.port.get()
    }
}
