mod id;
pub mod state;

use crate::{account::Account, prelude::*};
pub use id::Id;
use mongodb::{
    bson::doc,
    error::{TRANSIENT_TRANSACTION_ERROR, UNKNOWN_TRANSACTION_COMMIT_RESULT},
    options::{Acknowledgment, ReadConcern, TransactionOptions, WriteConcern},
    Client,
};
use std::time::Duration;

/// An application state.
pub struct App {
    cli: Client,
}

impl App {
    const DB: &'static str = "ndb";
    const ACCOUNTS: &'static str = "accounts";

    pub fn new(cli: Client) -> Self {
        Self { cli }
    }

    pub async fn save_account(&self, acc: Account<New>) -> Account<Saved> {
        let acc = Account::saved(acc, Id::new());

        self.cli
            .database(Self::DB)
            .collection::<Account<Saved>>(Self::ACCOUNTS)
            .insert_one(&acc, None)
            .await
            .expect("Inserting failed");

        acc
    }

    pub async fn get_account(&self, id: &str) -> Option<Account<Saved>> {
        self.cli
            .database(Self::DB)
            .collection(Self::ACCOUNTS)
            .find_one(doc! { "_id": id }, None)
            .await
            .expect("Finding failed")
    }

    pub async fn put_balance_with_transaction(&self, id: &str, amount: u64) -> Option<u64> {
        let accounts = self
            .cli
            .database(Self::DB)
            .collection::<Account<Saved>>(Self::ACCOUNTS);

        let options = TransactionOptions::builder()
            .read_concern(ReadConcern::majority())
            .write_concern(WriteConcern::builder().w(Acknowledgment::Majority).build())
            .build();

        let mut session = self
            .cli
            .start_session(None)
            .await
            .expect("Session starting failed");

        let change = loop {
            session
                .start_transaction(options.clone())
                .await
                .expect("Transaction starting failed");

            let mut acc = accounts
                .find_one_with_session(doc! { "_id": id }, None, &mut session)
                .await
                .expect("Finding failed")?;

            let change = acc.put_balance(amount);
            let new_balance = acc.balance() as i64;

            let res = accounts
                .update_one_with_session(
                    doc! { "_id": id },
                    doc! {
                        "$set": {
                            "balance": new_balance,
                        }
                    },
                    None,
                    &mut session,
                )
                .await;

            match res {
                Ok(_) => break change,
                Err(err) => {
                    if err.contains_label(TRANSIENT_TRANSACTION_ERROR) {
                        session.abort_transaction().await.expect("Aborting failed");
                        rocket::tokio::time::sleep(Duration::from_millis(5)).await;
                        continue;
                    }

                    panic!("Committing failed: {:?}", err);
                }
            }
        };

        loop {
            match session.commit_transaction().await {
                Ok(()) => break,
                Err(err) => {
                    if err.contains_label(UNKNOWN_TRANSACTION_COMMIT_RESULT) {
                        rocket::tokio::time::sleep(Duration::from_millis(5)).await;
                        continue;
                    }

                    panic!("Committing failed: {:?}", err);
                }
            }
        }

        Some(change)
    }

    pub async fn put_balance(&self, id: &str, amount: u64) -> Option<u64> {
        let accounts = self
            .cli
            .database(Self::DB)
            .collection::<Account<Saved>>(Self::ACCOUNTS);

        let mut acc = accounts
            .find_one(doc! { "_id": id }, None)
            .await
            .expect("Finding failed")?;

        let change = acc.put_balance(amount);
        let new_balance = acc.balance() as i64;

        accounts
            .update_one(
                doc! { "_id": id },
                doc! {
                    "$set": {
                        "balance": new_balance,
                    }
                },
                None,
            )
            .await
            .expect("Updating failed");

        Some(change)
    }

    pub async fn clear_accounts(&self) -> u64 {
        self.cli
            .database(Self::DB)
            .collection::<Account<Saved>>(Self::ACCOUNTS)
            .delete_many(doc! {}, None)
            .await
            .expect("Deleting failed")
            .deleted_count
    }
}
