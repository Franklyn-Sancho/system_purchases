use rusqlite::Transaction;

use crate::{database::database::Database, models::account_model::update_account};

#[derive(Debug)]
pub struct Account {
    pub id: String,
    pub user_id: String,
    pub balance: f64,
    pub coins: [f64; 6],
}

impl Account {
    pub fn new(id: String, user_id: String ,balance: f64) -> Self {
        Self {
            id,
            user_id,
            balance,
            coins: [2.00, 5.00, 10.00, 20.00, 50.00, 100.00],
        }
    }

    pub fn is_valid_coin(&self, value: f64) -> bool {
        self.coins.contains(&value)
    }

    pub fn update_balance(db: &Database, account: &mut Account, value: f64) {
        account.balance += value;
        update_account(db, account);
    }
}
