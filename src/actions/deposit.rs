use chrono::Utc;
use uuid::Uuid;

use crate::{
    database::database::Database, 
    utils::read_input::read_input, models::{account_model::Account, transactions_model::{Transactions, TransactionKind}},  
};


pub fn deposit(db: &Database, account: &mut Account, value: f64) -> Result<(), String> {
    if !account.is_valid_coin(value) {
        return Err(format!("Só aceitamos cédulas de {:?}", account.coins));
    }

    deposit_money(db, account, value);

    Ok(())
}

pub fn deposit_money(db: &Database, account: &mut Account, value: f64) {
    // Add the deposit amount to the account balance
    Account::update_balance(db, account, value);

    // Create a new transaction for the deposit
    let transaction = Transactions {
        transaction_id: Uuid::new_v4().to_string(),
        user_id: account.user_id.clone(),
        product_id: "".to_string(),
        transaction_type: TransactionKind::Deposit,
        transaction_date: Utc::now(),
        transaction_amount: value as f32,
    };
    Transactions::create_transactions(db, &transaction)
}


pub fn deposit_input(db: &Database, account: &mut Account) {
    loop {
        let value =
            read_input("Quando você deseja depositar (digite 0 para retornar ao menu inicial): ");
        if value == "0" {
            break;
        }
        if let Ok(value) = value.parse::<f64>() {
            // Adicionar o valor do depósito ao saldo da conta
            deposit_money(db, account, value);
            println!(
                "Você depositou {:.2} e seu saldo é de {:.2}",
                value, account.balance
            )
        } else {
            println!("Valor inválido, tente novamente")
        }
    }
}

