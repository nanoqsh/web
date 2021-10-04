use serde::Deserialize;
use std::{fs::File, io::Read, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct Addr {
    host: String,
    port: u16,
}

impl Addr {
    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

#[derive(Debug, Deserialize)]
pub struct Tls {
    pub certs: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    addr: Addr,
    pub tls: Tls,
}

impl Config {
    pub fn read() -> Self {
        let mut file = File::open("./Config.toml").expect("File opening failed");
        let mut buf = String::new();
        file.read_to_string(&mut buf).expect("File reading failed");
        toml::from_str(&buf).expect("Config parsing failed")
    }

    pub fn url(&self) -> String {
        format!("https://{}:{}", self.addr.host(), self.addr.port())
    }
}
