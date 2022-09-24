use std::io;

fn add_coins() {
    let mut total_coins = 0;

    loop {
        println!("Insira uma moeda: ");

        let mut coin = String::new();

        io::stdin()
            .read_line(&mut coin)
            .expect("Falha ao ler entrada");

        let coin: u32 = coin.trim().parse().expect("erro ao inserir moeda");

        if coin > 0 {
            total_coins = coin + total_coins;
            println!("Você inseriu {coin} e seu saldo é {total_coins}")
        } else if coin == 0 {
            buy_product()
        } else {
            println!("Valor inserido é inválido")
        }
    }
}

fn buy_product() {
    println!("Escolha um produto");

    let mut produto = String::new();

    io::stdin()
        .read_line(&mut produto)
        .expect("falha na entrada");

    if produto != "" {
        println!("Obrigado pela preferência, volte sempre")
    } else {
        println!("Produto inexistente")
    }
}

fn main() {
    add_coins();
    buy_product();
}
