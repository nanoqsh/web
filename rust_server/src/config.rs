use crate::{mongo::Config as MongoConfig, prelude::*};

#[derive(Deserialize)]
pub struct Config {
    mongo: MongoConfig,
}

impl Config {
    pub fn mongo(&self) -> &MongoConfig {
        &self.mongo
    }
}
