use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;

use crate::{
    database::database::Database, models::user_model::User, utils::read_input::read_input,
};

fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap()
}

fn verify_password(password: &str, password_hash: &str) -> bool {
    verify(password, password_hash).unwrap()
}

pub fn register_user(db: &Database) {
    let email = read_input("Digite o seu endereço de email: ");
    let password = read_input("Digite a sua senha: ");

    let password_hash = hash_password(&password);

    let user_id = Uuid::new_v4().to_string();

    User::create_user(db, &user_id, &email, &password_hash);

    println!("Usuário registrado com sucesso! {}", user_id);
}

pub fn authenticate(db: &Database) -> Option<User> {
    println!("Bem Vindo a Vending Machine");

    let email = read_input("Digite seu email cadastrado: ");
    let password = read_input("Digite a sua senha: ");

    if let Some(user) = User::find_by_email(db, &email) {
        if verify_password(&password, &user.password_hash) {
            return Some(user);
        }
    }

    None
}
