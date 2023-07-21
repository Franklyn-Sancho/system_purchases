use crate::database::database::Database;
use rusqlite::params;

pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
}

impl User {
    pub fn new(id: String, email: String, password_hash: String) -> Self {
        Self {
            id,
            email,
            password_hash,
        }
    }

    pub fn create_user(db: &Database, user_id: &str, email: &str, password_hash: &str) {
        db.conn
            .execute(
                "INSERT INTO users (id, email, password) VALUES (?1, ?2, ?3)",
                params![user_id, email, password_hash],
            ).unwrap();
    }

    pub fn find_by_email(db: &Database, email: &str) -> Option<Self> {
        let mut stmt = db
            .conn
            .prepare("SELECT id, password FROM users WHERE email = ?1")
            .unwrap();
        let user_iter = stmt
            .query_map([email], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap();

        let user_data: Option<(String, String)> = user_iter.map(|x| x.unwrap()).next();
        if let Some((user_id, password_hash)) = user_data {
            Some(Self::new(user_id, email.to_string(), password_hash))
        } else {
            None
        }
    }
}
