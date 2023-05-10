use std::io;

struct Produto {
    name: String,
    value: f64,
}

fn add_coins(mut saldo: f64) {
    let coins = [0.01, 0.05, 0.10, 0.25, 0.50, 1.00];

    loop {
        println!("Insira uma moeda: ");

        let mut value = String::new();

        io::stdin()
            .read_line(&mut value)
            .expect("Falha ao ler entrada");

        let value: f64 = value.trim().parse().expect("valor inválidos");

        if coins.contains(&value) {
            saldo = value + saldo;
            println!("Você inseriu {} e seu saldo é {}", value, saldo)
        } else if value == 0.0 {
            buy_product(saldo);
        } else {
            println!("Valor inválido")
        }
    }
}

fn buy_product(_saldo: f64) {
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
                if _saldo >= p.value {
                    let saldo_atual = _saldo - p.value;
                    println!("seu salto atual é: {}", saldo_atual)
                } else {
                    println!("Saldo insuficiente")
                }
            }
            None => println!("Produto não encontrado"),
        }
    }
}

fn main() {
    let saldo = 0.0;

    println!("Escolha a sua opção: ");
    println!("1 - inserir moeda");
    println!("2 - comprar produto");

    let mut option = String::new();

    io::stdin()
        .read_line(&mut option)
        .expect("falha na entrada");

    if option.contains("1") {
        add_coins(saldo)
    } else if option.contains("2") {
        buy_product(saldo)
    } else {
        println!("Opção inválida!")
    }
}
