use chrono::Utc;
use prettytable::{Table, row};
use rusqlite::params;
use uuid::Uuid;

use crate::{
    models::{account_model::Account, product_model::Product, transactions_model::{Transactions, TransactionKind}}, 
    database::database::Database, utils::read_input::read_input, 
};

pub fn select_product(db: &Database) -> String {
    display_products(&db);
    read_input("Digite o nome do produto que deseja comprar: ")
}


pub fn purchase_product(
    account: &mut Account,
    db: &Database,
    product_id: &str,
) -> Result<(), String> {
    // Verifica se o produto existe
    let (price, product_quantity) = match get_product_details(db, product_id) {
        Some((price, quantity)) => (price, quantity),
        None => return Err("Produto não encontrado".to_string()),
    };
    // Verifica se há quantidade suficiente do produto
    if !has_sufficient_quantity(product_quantity) {
        return Err("Produto esgotado".to_string());
    }
    // Verifica se o usuário tem saldo suficiente
    if !has_sufficient_balance(account, price) {
        return Err("Saldo insuficiente".to_string());
    }
    // Atualiza o saldo do usuário
    Account::update_balance(db, account, -price);
    // Atualiza a quantidade do produto no banco de dados
    update_product_quantity(db, product_id, product_quantity - 1);

    create_purchase_transaction(db, account, product_id, price);

    Ok(())
}

fn get_product_details(db: &Database, product_id: &str) -> Option<(f64, i32)> {
    Product::get_product_by_id(db, product_id).map(|(_, _, price, quantity) | (price, quantity))
}

fn has_sufficient_quantity(quantity: i32) -> bool {
    quantity >=1
}

fn has_sufficient_balance(account: &Account, price: f64) -> bool {
    account.balance >= price
}

fn update_product_quantity(db: &Database, product_id: &str, new_quantity: i32) {
    db.conn
        .execute(
            "UPDATE products SET quantity = ?1 WHERE id = ?2",
            params![new_quantity, product_id],
        )
        .unwrap();
}

fn display_products(db: &Database) {
    let mut stmt = db.conn.prepare(
        "SELECT name, price, quantity FROM products"
    ).unwrap();
    let products_iter = stmt.query_map([], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,

        ))
    })
    .unwrap();

    let mut table = Table::new();
    table.add_row(row!["Nome", "Price", "Quantity"]);
    for product in products_iter {
        let (name, price, quatity): (String, f64, i32) = product.unwrap();
        table.add_row(row![name, price, quatity]);
    }
    table.printstd()
}

pub fn create_purchase_transaction(
    db: &Database,
    account: &Account,
    product_id: &str,
    value: f64,
) {
    let transaction = Transactions {
        transaction_id: Uuid::new_v4().to_string(),
        user_id: account.user_id.clone(),
        product_id: product_id.to_string(),
        transaction_type: TransactionKind::Purchase,
        transaction_date: Utc::now(),
        transaction_amount: value as f32,
    };
    Transactions::create_transactions(db, &transaction);
}

