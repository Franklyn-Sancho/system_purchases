use crate::database::database::Database;
use rusqlite::params;

pub struct Product {
    pub product_id: String,
    pub name_product: String,
    pub price: f64,
    pub quantity: i32,
}

impl Product {
    pub fn new(product_id: String, name_product: String, price: f64, quantity: i32) -> Self {
        Self {
            product_id,
            name_product,
            price,
            quantity,
        }
    }

    pub fn insert_product(
        db: &Database,
        product_id: &str,
        name_product: &str,
        price: f64,
        quantity: i32,
    ) {
        db.conn
            .execute(
                "INSERT INTO products(id, name, price, quantity) VALUES (?1, ?2, ?3, ?4)",
                params![product_id, name_product, price, quantity],
            )
            .unwrap();
    }

    pub fn get_product_by_id(
        db: &Database,
        product_id: &str,
    ) -> Option<(String, String, f64, i32)> {
        let mut stmt = db
            .conn
            .prepare("SELECT id, name, price, quantity FROM products WHERE id = ?1")
            .unwrap();
        let product_iter = stmt
            .query_map([product_id], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
            })
            .unwrap();

        let product_data: Option<(String, String, f64, i32)> =
            product_iter.map(|x| x.unwrap()).next();
        if let Some((id, name, price, quantity)) = product_data {
            Some((id, name, price, quantity))
        } else {
            None
        }
    }

    pub fn get_product_by_name(db: &Database, name_product: &str) -> Option<(String, f64, i32)> {
        let mut stmt = db
            .conn
            .prepare("SELECT id, price, quantity FROM products WHERE name = ?1")
            .unwrap();
        let product_iter = stmt
            .query_map([name_product], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?))
            })
            .unwrap();

        let result = product_iter.map(|x| x.unwrap()).next();
        result
    }

    pub fn update_product_by_name(
        db: &Database,
        product_name: &str,
        new_name: &str,
        new_price: f64,
        new_quantity: i32,
    ) {
        db.conn
            .execute(
                "UPDATE products SET name = ?1, price = ?2, quantity = ?3 WHERE name = ?4",
                params![new_name, new_price, new_quantity, product_name],
            )
            .unwrap();
    }

    pub fn update_product_quantity(db: &Database, product_id: &str, new_quantity: i32) {
        db.conn
            .execute(
                "UPDATE products SET quantity = ?1 WHERE id = ?2",
                params![new_quantity, product_id],
            )
            .unwrap();
    }

    pub fn delete_product_by_name(db: &Database, product_name: &str) {
        // Use o método `execute` para deletar o produto
        db.conn
            .execute(
                "DELETE FROM products WHERE name = ?1",
                params![product_name],
            )
            .unwrap();
    }
}
