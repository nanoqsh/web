use crate::app::{state::*, Id};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Account<S>
where
    S: State,
{
    #[serde(rename = "_id")]
    id: S::Id,
    name: String,
    balance: u64,
}

impl Account<New> {
    pub fn new(name: String) -> Self {
        Self {
            id: (),
            name,
            balance: 0,
        }
    }

    pub fn saved(self, id: Id) -> Account<Saved> {
        self.map(|_| id)
    }
}

impl Account<Saved> {
    pub fn put_balance(&mut self, amount: u64) -> u64 {
        const MAX_BALANCE: u64 = 100_000;

        let new_balance = match self.balance.checked_add(amount) {
            None => return amount,
            Some(new_balance) => new_balance,
        };

        let (new_balance, change) = if new_balance > MAX_BALANCE {
            (MAX_BALANCE, new_balance - MAX_BALANCE)
        } else {
            (new_balance, 0)
        };

        self.balance = new_balance;
        change
    }
}

impl<S> Account<S>
where
    S: State,
{
    pub fn balance(&self) -> u64 {
        self.balance
    }

    fn map<U, F>(self, f: F) -> Account<U>
    where
        U: State,
        F: FnOnce(S::Id) -> U::Id,
    {
        Account {
            id: f(self.id),
            name: self.name,
            balance: self.balance,
        }
    }
}
