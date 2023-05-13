use std::io;

struct Produto {
    name: String,
    value: f64,
}

struct Saldo {
    saldo: f64,
}

impl Saldo {
    fn add_coins(&mut self, value: f64) {
        let coins = [0.01, 0.05, 0.10, 0.25, 0.50, 1.00];

        if coins.contains(&value) {
            self.saldo += value;
            println!("Você inseriu {} e seu saldo é {}", value, self.saldo)
        } else if value == 0.0 {
            self.buy_product();
        } else {
            println!("Valor inválido")
        }
    }

    fn buy_product(&mut self) {
        let produtos = vec![
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
        ];

        loop {
            println!("Escolha um produto: ");

            let mut choice = String::new();

            io::stdin()
                .read_line(&mut choice)
                .expect("falha na entrada");

            let verificar_produto = produtos.iter().find(|p| p.name.trim() == choice.trim());

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
}

fn main() {
    let mut saldo = Saldo { saldo: 0.0 };

    println!("Escolha a sua opção: ");
    println!("1 - inserir moeda");
    println!("2 - comprar produto");

    let mut option = String::new();

    io::stdin()
        .read_line(&mut option)
        .expect("falha na entrada");

    match option.trim() {
        "1" => loop {
            println!("Insira uma moeda: ");

            let mut value = String::new();

            io::stdin()
                .read_line(&mut value)
                .expect("Falha ao ler entrada");

            let value: f64 = match value.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            saldo.add_coins(value);
        },
        "2" => saldo.buy_product(),
        _ => println!("Opção inválida!"),
    }
}
