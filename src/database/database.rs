use rusqlite::Connection;

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self, rusqlite::Error> {
        let conn = Connection::open(path).unwrap();
        Ok(Self { conn })
    }

    pub fn create_tables(&self) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY, email TEXT NOT NULL UNIQUE, password TEXT NOT NULL
            )",
            [],
        )?;

        self.conn
            .execute("CREATE TABLE IF NOT EXISTS products (
                id TEXT PRIMARY KEY, name TEXT NOT NULL, price REAL NOT NULL, quantity INTEGER NOT NULL
            )", [])
            ?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS account (
                id TEXT PRIMARY KEY, user_id TEXT NOT NULL, balance REAL, 
                FOREIGN KEY(user_id) REFERENCES users(id)
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS transactions (
                    id TEXT PRIMARY KEY, 
                    user_id INTEGER NOT NULL, 
                    product_id INTEGER, 
                    transaction_type TEXT NOT NULL, 
                    transaction_date TEXT NOT NULL, 
                    transaction_amount REAL NOT NULL,
                    FOREIGN KEY (user_id) REFERENCES users(id),
                    FOREIGN KEY (product_id) REFERENCES products(id)
                )",
            [],
        )?;

        Ok(())
    }


    
}
