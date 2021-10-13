use crate::prelude::*;
use mongodb::{options::ClientOptions, Client};
use rocket::async_trait;

#[derive(Deserialize)]
pub struct Config {
    user: String,
    pass: String,
    host: String,
    port: u16,
    workers: Option<u32>,
}

impl Config {
    pub fn connection_string(&self) -> String {
        format!(
            "mongodb://{}:{}@{}:{}",
            self.user, self.pass, self.host, self.port
        )
    }

    pub fn workers(&self) -> Option<u32> {
        self.workers
    }
}

#[async_trait]
pub trait ConnectMongo {
    async fn connect_mongo<F, S>(self, f: F) -> Self
    where
        F: FnOnce(Client) -> S + Send,
        S: Send + Sync + 'static;
}

#[async_trait]
impl ConnectMongo for rocket::Rocket<rocket::Build> {
    async fn connect_mongo<F, S>(self, f: F) -> Self
    where
        F: FnOnce(Client) -> S + Send,
        S: Send + Sync + 'static,
    {
        let config: crate::config::Config =
            self.figment().extract().expect("Config reading failed");

        let mongo = config.mongo();
        let connection_string = mongo.connection_string();
        let mut options = ClientOptions::parse(connection_string)
            .await
            .expect("Options parsing failed");

        options.retry_writes = Some(false);
        options.max_pool_size = mongo.workers();

        let client = Client::with_options(options).expect("Client creation failed");
        self.manage(f(client))
    }
}
