use std::io::{self, Write};

use VENDING_MACHINE::{
    database::database::Database,
    menus::menus::login_register_menu, models::user_model::User
    };
use rusqlite::Error;



//struct dos produtos
struct Produto {
    name: String,
    value: f64,
}

//struct do Saldo
struct Saldo {
    saldo: f64,
    produtos: Vec<Produto>,
    coins: [f64; 6],
}

//* métodos da struct Saldo por implementação
// preciso dividir os métodos de saldo e produto para deixar a aplicação mais limpa
impl Saldo {
    fn new() -> Self {
        Self {
            saldo: 0.0,
            produtos: vec![
                Produto {
                    name: "biscoito".to_string(),
                    value: 1.5,
                },
                Produto {
                    name: "soda".to_string(),
                    value: 1.0,
                },
                Produto {
                    name: "chocolate".to_string(),
                    value: 2.5,
                },
            ],
            coins: [0.01, 0.05, 0.10, 0.25, 0.50, 1.00],
        }
    }

    //verifica se o valor de entrada é um produto -> se for, executa o buy_product
    fn is_product(&self, input: &str) -> bool {
        self.produtos.iter().any(|p| p.name.trim() == input.trim())
    }

    //verifica se é uma moeda -> se for, executa o add_coin
    fn is_coin(&self, input: &str) -> bool {
        self.coins.contains(&input.trim().parse().unwrap())
    }

    //função responsável por adicionar moedas no saldo
    fn add_coins(&mut self, value: f64) {
        if self.coins.contains(&value) {
            self.saldo += value;
            println!("Você inseriu {} e seu saldo é {}", value, self.saldo)
        } else {
            println!("Valor inválido")
        }
    }

    //função responsável por comprar produtos
    fn buy_product(&mut self, product_name: &str) {
        let verificar_produto = self
            .produtos
            .iter()
            .find(|p| p.name.trim() == product_name.trim());

        match verificar_produto {
            Some(p) => {
                if self.saldo >= p.value {
                    let saldo_atual = self.saldo - p.value;
                    println!("seu salto atual é: {}", saldo_atual);
                    self.saldo = saldo_atual;
                } else {
                    println!("Saldo insuficiente")
                }
            }
            None => println!("Produto não encontrado"),
        }
    }
}

fn main() -> Result<(), Error> {
    /* let mut item = Saldo::new();
 */
    let db = Database::new("database.db")?;

    User::create_user(&db, "admin_id", "admin@admin.com", "password_hash", true);

    db.create_tables();

    login_register_menu(&db);

    /* loop {
        let mut input = String::new();

        print!("Insira uma moeda ou compre um produto: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler entrada");

        if item.is_product(&input) {
            item.buy_product(&input);
        } else if item.is_coin(&input) {
            let value: f64 = input.trim().parse().unwrap();
            item.add_coins(value);
        } else {
            println!("Entrada inválida!");
        }
    } */

    Ok(())
}
