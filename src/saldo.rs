use std::io;

pub struct Saldo {
    pub saldo: f64,
}

impl Saldo {
    pub fn add_coins(&mut self, value: f64) {
        let coins = [0.01, 0.05, 0.10, 0.25, 0.50, 1.00];
        if coins.contains(&value) {
            self.saldo += value;
            println!("Você inseriu {} e seu saldo é {}", value, self.saldo)
        } else if value == 0.0 {
            println!("Saldo atual é {}", self.saldo);
        } else {
            println!("Valor inválido")
        }
    }
}
