use std::io::{self, Write};

//struct dos produtos
struct Produto {
    name: String,
    value: f64,
}

//struct do Saldo
struct Saldo {
    saldo: f64,
    produtos: Vec<Produto>,
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
        }
    }

    //verifica se o valor de entrada é um produto -> se for, executa o buy_product
    fn is_product(&self, input: &str) -> bool {
        self.produtos.iter().any(|p| p.name.trim() == input.trim())
    }

    //verifica se é uma moeda -> se for, executa o add_coin
    fn is_coin(input: &str) -> bool {
        let coins = ["0.01", "0.05", "0.10", "0.25", "0.50", "1.00"];
        coins.contains(&input.trim())
    }

    //função responsável por adicionar moedas no saldo
    fn add_coins(&mut self, value: f64) {
        let coins = [0.01, 0.05, 0.10, 0.25, 0.50, 1.00];

        if coins.contains(&value) {
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

fn main() {
    let mut saldo = Saldo::new();

    loop {
        let mut input = String::new();

        print!("Insira uma moeda ou compre um produto: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler entrada");

        if saldo.is_product(&input) {
            saldo.buy_product(&input);
        } else if Saldo::is_coin(&input) {
            let value: f64 = input.trim().parse().unwrap();
            saldo.add_coins(value);
        } else {
            println!("Entrada inválida!");
        }
    }
}
