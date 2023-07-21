use crate::{
    database::database::Database,
    utils::read_input::read_input,
};

use super::account::Account;

pub fn deposit(db: &Database, account: &mut Account, value: f64) -> Result<(), String> {
    if !account.is_valid_coin(value) {
        return Err(format!("Só aceitamos cédulas de {:?}", account.coins));
    }

    Account::update_balance(db, account, value);

    Ok(())
}

pub fn deposit_input(db: &Database, account: &mut Account) {
    loop {
        let value =
            read_input("Quando você deseja depositar (digite 0 para retornar ao menu inicial): ");
        if value == "0" {
            break;
        }
        if let Ok(value) = value.parse::<f64>() {
            Account::update_balance(db, account, value);
            println!(
                "Você depositou {:.2} e seu saldo é de {:.2}",
                value, account.balance
            )
        } else {
            println!("Valor inválido, tente novamente")
        }
    }
}
