use chrono::{DateTime, Utc, TimeZone};
use rusqlite::{
    params,
    types::{FromSql, FromSqlError, FromSqlResult, ValueRef},
};

use chrono_tz::Tz;

use crate::database::database::Database;

#[derive(Debug)]
pub enum TransactionKind {
    Deposit,
    Purchase,
}

pub struct Transactions {
    pub transaction_id: String,
    pub user_id: String,
    pub product_id: String,
    pub transaction_type: TransactionKind,
    pub transaction_date: DateTime<Utc>,
    pub transaction_amount: f32,
}

impl FromSql for TransactionKind {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str()? {
            "Deposit" => Ok(TransactionKind::Deposit),
            "Purchase" => Ok(TransactionKind::Purchase),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl Transactions {
    pub fn create_transactions(db: &Database, transaction: &Transactions) {
        let formatted_date = transaction
            .transaction_date
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        if let Err(e) = db.conn.execute(
            "INSERT INTO transactions (transaction_id, user_id, product_id, transaction_type, transaction_date, transaction_amount) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                transaction.transaction_id,
                transaction.user_id,
                transaction.product_id,
                format!("{:?}", transaction.transaction_type),
                formatted_date,
                transaction.transaction_amount
            ],
        ) {
            println!("Erro ao criar transação: {}", e);
        }
    }

    pub fn get_transactions_to_user(db: &Database, user_id: String) -> Vec<String> {
        let tz: Tz = "America/Sao_Paulo".parse().unwrap();
        let mut stmt = db.conn
            .prepare(
                "SELECT products.name, transactions.transaction_type, transactions.transaction_date, transactions.transaction_amount FROM transactions LEFT JOIN products ON transactions.product_id = products.id WHERE transactions.user_id = ?1",
            )
            .unwrap();
        let rows = stmt
            .query_map(params![user_id], |row| {
                let product_name: String = row.get(0).unwrap_or("".to_string());
                let transaction_type: String = row.get(1).unwrap();
                let transaction_date: String = row.get(2).unwrap();
                let transaction_date = Utc
                    .datetime_from_str(&transaction_date, "%Y-%m-%d %H:%M:%S")
                    .expect(&format!("Failed to parse date: {}", transaction_date));
                let transaction_date = transaction_date.with_timezone(&tz);
                let transaction_amount: f64 = row.get(3).unwrap();
                Ok(format!(
                    "{} | {} | {} | {:.2}",
                    transaction_date.to_rfc3339(),
                    transaction_type,
                    product_name,
                    transaction_amount
                ))
            })
            .unwrap();

        rows.map(|row| row.unwrap()).collect()
    }

    pub fn get_all_transactions_to_admin(db: &Database) -> Vec<String> {
        let tz: Tz = "America/Sao_Paulo".parse().unwrap();
        let mut stmt = db.conn
            .prepare(
                "SELECT transactions.user_id, products.name, transactions.transaction_type, transactions.transaction_date, transactions.transaction_amount FROM transactions LEFT JOIN products ON transactions.product_id = products.id",
            )
            .unwrap();
        let rows = stmt
            .query_map([], |row| {
                let user_id: String = row.get(0).unwrap();
                let product_name: String = row.get(1).unwrap_or("".to_string());
                let transaction_type: String = row.get(2).unwrap();
                let transaction_date: String = row.get(3).unwrap();
                let transaction_date = Utc
                    .datetime_from_str(&transaction_date, "%Y-%m-%d %H:%M:%S")
                    .expect(&format!("Failed to parse date: {}", transaction_date));
                let transaction_date = transaction_date.with_timezone(&tz);
                let transaction_amount: f64 = row.get(4).unwrap();
                Ok(format!(
                    "{} | {} | {} | {} | {:.2}",
                    user_id,
                    transaction_date.to_rfc3339(),
                    transaction_type,
                    product_name,
                    transaction_amount
                ))
            })
            .unwrap();
    
        rows.map(|row| row.unwrap()).collect()
    }
}







