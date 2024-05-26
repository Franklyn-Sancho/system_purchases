use crate::{
    actions::{
        deposit::deposit_input, product::{create_product, delete_product, display_products, update_product}, purchase::{purchase_product, select_product}
    },
    authenticate::auth::{authenticate, register_user},
    database::database::Database,
    models::{account_model::Account, product_model::Product, transactions_model::Transactions},
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
                    authenticate(db);
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
                    let transactions =
                        Transactions::get_transactions_to_user(db, account.user_id.clone());
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
    loop {
        println!("Bem vindo ao menu de administrador da aplicação");
        println!("Escolha a sua opção: ");
        println!("1 - Inserir novos produtos");
        println!("2 - Lista de Produtos");
        println!("3 - Atualizar produtos");
        println!("4 - Deletar Produto");
        println!("5 - histórico de transações");
        println!("6 - Logs de erros");
        println!("7 - Controles de usuarios");
        print!("Insira a sua opção aqui: ");

        let option = read_input("");
        if let Ok(option) = option.parse() {
            match option {
                1 => create_product(db),
                2 => display_products(db),
                3 => update_product(db),
                4 => delete_product(db),
                5 => {
                    let transactions = Transactions::get_all_transactions_to_admin(db);
                    for transaction in transactions {
                        println!("{}", transaction);
                    }
                }
                _ => print!("Opção inválida"),
            }
        } else {
            println!("Opção inválida, tente novamente")
        }
    }
}
