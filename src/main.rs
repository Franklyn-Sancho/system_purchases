use std::{io, collections::btree_map::Values};

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

fn buy_product(mut saldo: f64) {

    
      let chocolate =  Produto {name: String::from("chocolate"), value: 2.50};

    /* let chocolate = Produto { name: String::from("chocolate"), value: 2.50}; */

    loop {
        println!("Escolha um produto: ");

        let mut produto = String::new();

        io::stdin()
            .read_line(&mut produto)
            .expect("falha na entrada");

        if  produto.contains("chocolate")
        {
            saldo = saldo - chocolate.value;
            println!("Você comprou {} e seu troco é {}: ", chocolate.name, saldo);
        } else {
            println!("Produto Inválido")
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
