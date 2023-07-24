use bcrypt::{hash, verify, DEFAULT_COST};
use regex::Regex;
use uuid::Uuid;

use crate::{
    database::database::Database, 
    models::user_model::User,
    models::account_model::Account,
    utils::read_input::read_input, 
    menus::menus::{vending_machine_admin_menu, 
    machine_vending_menu},
};

fn is_valid_email(email: &str) -> bool {
    let email_re =
        Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$")
            .unwrap();
    email_re.is_match(email)
}

fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap()
}

fn verify_password(password: &str, password_hash: &str) -> bool {
    verify(password, password_hash).unwrap()
}

pub fn register_user(db: &Database) {
    let email = read_input("Digite o seu endereço de email: ");

    if User::find_by_email(db, &email).is_some() {
        println!("Erro ao registrar usuário, verifique seus dados");
        return;
    }

    // Validate the email address
    if !is_valid_email(&email) {
        println!("Endereço de email inválido");
        return;
    }

    let password = read_input("Digite a sua senha: ");

    let password_hash = hash_password(&password);

    let user_id = Uuid::new_v4().to_string();

    User::create_user(db, &user_id, &email, &password_hash, false);

    println!("usuário registrado com sucesso");

}



pub fn authenticate(db: &Database) -> Option<User> {
    println!("bem vindo a vending machine");

    let email = read_input("Digite seu email cadastrado: ");
    let password = read_input("Digite a sua senha: ");

    if let Some(user) = User::find_by_email(db, &email) {
        if verify_password(&password, &user.password_hash) {
            let mut account = Account::get_account_by_user(db, &user.id)
                .unwrap_or_else(|| Account::create_account(db, &user.id));
            if user.is_admin {
                vending_machine_admin_menu(db)
            } else {
                machine_vending_menu(db, &mut account)
            }
            return Some(user);
        }
    }

    None
}

