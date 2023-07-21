use std::io::{self, Write};

pub fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Falha ao ler entrada");
    value.trim().to_string()
}