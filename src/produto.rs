use crate::saldo::Saldo;
use std::io;

pub struct Produto {
    name: String,
    value: f64,
}

pub struct Produtos {
    pub produtos: Vec<Produto>,
}

impl Produtos {
    pub fn new() -> Self {
        Self {
            produtos: vec![
                Produto {
                    name: "biscoito".to_string(),
                    value: 1.5,
                },
                Produto {
                    name: "soda".to_string(),
                    value: 1.5,
                },
                Produto {
                    name: "chocolate".to_string(),
                    value: 1.5,
                },
            ],
        }
    }

    pub fn buy_product(&mut self, saldo: &mut Saldo) {
        loop {
            println!("Escolha um produto: ");
            for (i, p) in self.produtos.iter().enumerate() {
                println!("{} - {} (R${})", i + 1, p.name, p.value);
            }
            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("falha na entrada");
            let choice = match choice.trim().parse::<usize>() {
                Ok(num) => num,
                Err(_) => continue,
            };
            if choice > self.produtos.len() || choice == 0 {
                println!("Produto não encontrado");
                continue;
            }
            let produto = &self.produtos[choice - 1];
            if saldo.saldo >= produto.value {
                let saldo_atual = saldo.saldo - produto.value;
                println!("seu saldo é de {}", saldo.saldo)
            }
        }
    }
}
