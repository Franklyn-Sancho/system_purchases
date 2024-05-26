use prettytable::{row, Table};
use uuid::Uuid;

use crate::{
    database::database::Database, models::product_model::Product, utils::read_input::read_input,
};


pub fn display_products(db: &Database) {
    let mut stmt = db.conn.prepare(
        "SELECT name, price, quantity FROM products"
    ).unwrap();
    let products_iter = stmt.query_map([], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,

        ))
    })
    .unwrap();

    let mut table = Table::new();
    table.add_row(row!["Nome", "Price", "Quantity"]);
    for product in products_iter {
        let (name, price, quatity): (String, f64, i32) = product.unwrap();
        table.add_row(row![name, price, quatity]);
    }
    table.printstd()
}

pub fn create_product(db: &Database) {
    let product_id = Uuid::new_v4().to_string();
    let name_product = read_input("Digite o nome do produto: ");
    let price = read_input("Digite o preço do produto: ");
    let quantity = read_input("Digite a quantidade: ");

    Product::insert_product(
        db,
        &product_id,
        &name_product,
        price.parse().unwrap(),
        quantity.parse().unwrap(),
    );
}

pub fn update_product(db: &Database) {

    display_products(db);

    let product_id = read_input("Digite o ID do produto que deseja atualizar: ");
    let new_name = read_input("Digite o novo nome do produto: ");
    let new_price = read_input("Digite o novo preço: ");
    let new_quantity = read_input("Digite a nova quantidade: ");

    Product::update_product_by_name(
        db,
        &product_id,
        &new_name,
        new_price.parse().unwrap(),
        new_quantity.parse().unwrap(),
    );
}

pub fn delete_product(db: &Database) {

    display_products(db);

    let product_name = read_input("Digite o NOME do produto que deseja deletar: ");

    Product::delete_product_by_name(db, &product_name)
}
