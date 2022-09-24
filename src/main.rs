use std::io;

struct TotalCoins {
    value: f64
}

/* struct Produto {
    name: String,
    value: f64,
} */


fn add_coins() {
    let coins = [0.05, 0.10, 0.25, 0.50, 1.00];

    let mut total_coins = TotalCoins {value: 0.0};

    loop {
        println!("Insira uma moeda: ");

        let mut coin = String::new();

        io::stdin()
            .read_line(&mut coin)
            .expect("Falha ao ler entrada");

        let coin: f64 = coin.trim().parse().expect("erro ao inserir moeda");

        if coins.contains(&coin) {
            total_coins.value = coin + total_coins.value;
            println!("Você inseriu {} e seu saldo é {}", coin, total_coins.value);
        } else if coin == 0.0 {
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

    println!("Você comprou {produto}")


}

fn main() {
    add_coins();
    buy_product();
}
