mod account;
mod app;
mod config;
mod mongo;
mod reply;

mod prelude {
    pub use crate::app::{state::*, App};
    pub use serde::{Deserialize, Serialize};
}

use crate::{account::Account, mongo::ConnectMongo, prelude::*, reply::Reply};
use rocket::{get, post, routes, serde::json::Json, State};

#[derive(Deserialize)]
struct Info {
    name: String,
}

#[post("/make", data = "<info>")]
async fn make(app: &State<App>, info: Json<Info>) -> Reply<Account<Saved>> {
    let Json(Info { name }) = info;
    let acc = Account::new(name);
    let acc = app.save_account(acc).await;
    Reply::Ok(acc)
}

#[get("/balance/<id>")]
async fn balance(app: &State<App>, id: &str) -> Reply<Option<u64>> {
    let res = app.get_account(id).await;
    Reply::Ok(res.map(|acc| acc.balance()))
}

#[derive(Deserialize)]
struct Balance {
    amount: u64,
}

#[post("/put/<id>", data = "<info>")]
async fn put(app: &State<App>, id: &str, info: Json<Balance>) -> Reply<Option<u64>> {
    let Json(Balance { amount }) = info;
    let change = app.put_balance_with_transaction(id, amount).await;
    Reply::Ok(change)
}

#[post("/put_fast/<id>", data = "<info>")]
async fn put_fast(app: &State<App>, id: &str, info: Json<Balance>) -> Reply<Option<u64>> {
    let Json(Balance { amount }) = info;
    let change = app.put_balance(id, amount).await;
    Reply::Ok(change)
}

#[derive(Serialize)]
struct Clear {
    count: u64,
}

#[post("/clear")]
async fn clear(app: &State<App>) -> Reply<Clear> {
    let count = app.clear_accounts().await;
    Reply::Ok(Clear { count })
}

#[get("/")]
async fn index() -> &'static str {
    "Welcome!"
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .connect_mongo(App::new)
        .await
        .mount("/", routes![index, make, balance, put, put_fast, clear])
        .launch()
        .await;
}
