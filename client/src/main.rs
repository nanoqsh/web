mod config;
mod route;
mod tester;

mod prelude {
    pub use serde::{Deserialize, Serialize};
}

use crate::{
    config::Config,
    tester::{Clear, Reply, Tester},
};
use futures::future;
use std::{error::Error, fs::File, io::Read};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::read();

    let cert = {
        let mut buf = Vec::new();
        File::open(&config.tls.certs)?.read_to_end(&mut buf)?;
        reqwest::Certificate::from_pem(&buf)?
    };

    let url = config.url();
    let cli = reqwest::ClientBuilder::new()
        .add_root_certificate(cert)
        .build()?;

    let fast = std::env::args().any(|arg| arg == "fast");
    let tester = Tester::new(url, cli, fast);
    let clear = tester.clear().await?;
    if let Reply::Ok(Clear { count }) = clear {
        println!("cleared: {}", count);
    }

    let acc = tester.make("nano").await?.ok();
    let tasks = (0..100).map(|_| tester.put(&acc, 1));
    let changes = future::join_all(tasks).await;
    assert!(changes.into_iter().all(|c| c.unwrap().ok() == 0));

    let balance = tester.balance(&acc).await?.ok();
    println!("balance = {}", balance);

    Ok(())
}
