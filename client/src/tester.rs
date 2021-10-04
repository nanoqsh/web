use crate::{prelude::*, route::Route};
use reqwest::{Client, Error};

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Reply<T> {
    Ok(T),
    ClientError(String),
    ServerError,
}

impl<T> Reply<T> {
    pub fn ok(self) -> T {
        match self {
            Reply::Ok(ok) => ok,
            _ => panic!("Unexpected error"),
        }
    }
}

#[derive(Deserialize)]
pub struct Account {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub balance: u64,
}

#[derive(Deserialize)]
pub struct Clear {
    pub count: u64,
}

pub struct Tester {
    rou: Route,
    cli: Client,
    fast: bool,
}

impl Tester {
    pub fn new(url: String, cli: Client, fast: bool) -> Self {
        Self {
            rou: Route::new(url),
            cli,
            fast,
        }
    }

    pub async fn make(&self, name: &str) -> Result<Reply<Account>, Error> {
        #[derive(Serialize)]
        struct Info<'a> {
            name: &'a str,
        }

        let url = self.rou.make();
        let info = Info { name };
        self.cli.post(url).json(&info).send().await?.json().await
    }

    pub async fn balance(&self, acc: &Account) -> Result<Reply<u64>, Error> {
        let url = self.rou.balance(&acc.id);
        self.cli.get(url).send().await?.json().await
    }

    pub async fn put(&self, acc: &Account, amount: u64) -> Result<Reply<u64>, Error> {
        #[derive(Serialize)]
        pub struct Info {
            pub amount: u64,
        }

        let url = self.rou.put(&acc.id, self.fast);
        let info = Info { amount };
        self.cli.post(url).json(&info).send().await?.json().await
    }

    pub async fn clear(&self) -> Result<Reply<Clear>, Error> {
        let url = self.rou.clear();
        self.cli.post(url).send().await?.json().await
    }
}
