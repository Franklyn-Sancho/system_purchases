// account.rs


use rusqlite::params;
use uuid::Uuid;
use crate::database::database::Database;

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
        Self::update_account(db, account);
    }

    pub fn create_account(db: &Database, user_id: &str) -> Account {
        let account_id = Uuid::new_v4().to_string();
        db.conn
            .execute(
                "INSERT INTO account (id ,user_id, balance) VALUES (?1, ?2, 0)",
                params![account_id, user_id],
            )
            .unwrap();

        Account::new(account_id, user_id.to_string(), 0.0)
    }

    pub fn get_account_by_user(db: &Database, user_id: &str) -> Option<Account> {
        db.conn
            .query_row(
                "SELECT id, balance FROM account WHERE user_id = ?1",
                [user_id],
                |row| Ok(Account::new(row.get(0)?, user_id.to_string(), row.get(1)?)),
            )
            .ok()
    }

    fn update_account(db: &Database, account: &Account) {
        match db.conn.execute(
            "UPDATE account SET balance = ?1 WHERE id = ?2",
            params![account.balance, account.id],
        ) {
            Ok(_) => (),
            Err(e) => println!("Erro ao atualizar a conta: {}", e),
        }
    }
}

