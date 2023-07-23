use crate::{
    account::{account::Account, deposit::deposit_input},
    authenticate::auth::{authenticate, register_user},
    database::database::Database,
    models::{
        account_model::{create_account, get_account_by_user},
        product_model::Product, transactions_model::{create_transactions, get_transactions, Transactions},
    },
    purchase::purchase::{purchase_product, select_product},
    utils::read_input::read_input,
};

pub fn login_register_menu(db: &Database) {
    loop {
        println!("Escolha a sua opção: ");
        println!("1 - Register");
        println!("2 - Login");
        println!("3 - Sair");
        print!("Insira a sua opção aqui: ");

        let option = read_input("");
        if let Ok(option) = option.parse() {
            match option {
                1 => {
                    register_user(db);
                }
                2 => {
                    if let Some(user) = authenticate(db) {
                        let mut account = get_account_by_user(db, &user.id)
                            .unwrap_or_else(|| create_account(db, &user.id));
                        if user.is_admin {
                            vending_machine_admin_menu(db)
                        } else {
                            machine_vending_menu(db, &mut account)
                        }
                        break;
                    } else {
                        println!("Nome de usuário ou senha inválidos");
                    }
                }
                3 => break,
                _ => println!("Opção inválida"),
            }
        } else {
            println!("opção inválida, tente novamente")
        }
    }
}

pub fn machine_vending_menu(db: &Database, account: &mut Account) {
    loop {
        println!("Seu saldo atual é: {:.2}", account.balance);
        println!("Escolha a sua opção: ");
        println!("1 - Depositar");
        println!("2 - Comprar produto");
        println!("3 - Histórico de transações");
        print!("Insira a sua opção aqui: ");

        let option = read_input("");
        if let Ok(option) = option.parse() {
            match option {
                1 => deposit_input(db, account),
                2 => {
                    let product_name = select_product(db);
                    if let Some((product_id, _, _)) =
                        Product::get_product_by_name(&db, &product_name)
                    {
                        match purchase_product(account, &db, &product_id) {
                            Ok(_) => println!("Compra realizada com sucesso!"),
                            Err(e) => println!("Erro ao realizar a compra: {}", e),
                        }
                    } else {
                        println!("Produto não encontrado");
                    }
                }
                3 => {
                    let transactions = get_transactions(db, account.user_id.clone());
                    for transaction in transactions {
                        println!("{}", transaction);
                    }
                }
                _ => println!("Opção inválida"),
            }
        } else {
            println!("Opção inválida, tente novamente")
        }
    }
}

pub fn vending_machine_admin_menu(db: &Database) {
    println!("Bem vindo ao menu de administrador da aplicação");
    println!("Escolha a sua opção: ");
    println!("1 - Inserir novos produtos");
    println!("2 - histórico de transações");
    println!("3 - Logs de erros");
    println!("4 - Controles de usuarios");
    print!("Insira a sua opção aqui: ");

    let option = read_input("");
    if let Ok(option) = option.parse() {
        match option {
            1 => {
                Product::create_product(db)
            }
            2 => {
                /* let transactions = get_transactions(db);
                    for transaction in transactions {
                        println!("{:?}", transaction)
                    } */
            }
            _ => print!("Opção inválida"),
        }
    }
}
