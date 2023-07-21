use crate::database::database::Database;
use rusqlite::params;

pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub is_admin: bool,
}

impl User {
    pub fn new(id: String, email: String, password_hash: String, is_admin: bool) -> Self {
        Self {
            id,
            email,
            password_hash,
            is_admin,
        }
    }

    pub fn create_user(
        db: &Database,
        user_id: &str,
        email: &str,
        password_hash: &str,
        is_admin: bool,
    ) {
        db.conn
            .execute(
                "INSERT INTO users (id, email, password, is_admin) VALUES (?1, ?2, ?3, ?4)",
                params![user_id, email, password_hash, is_admin as i32],
            )
            .unwrap();
    }

    pub fn find_by_email(db: &Database, email: &str) -> Option<Self> {
        let mut stmt = db
            .conn
            .prepare("SELECT id, password, is_admin FROM users WHERE email = ?1")
            .unwrap();
        let user_iter = stmt
            .query_map([email], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
            .unwrap();

        let user_data: Option<(String, String, i32)> = user_iter.map(|x| x.unwrap()).next();
        if let Some((user_id, password_hash, is_admin)) = user_data {
            Some(Self::new(
                user_id,
                email.to_string(),
                password_hash,
                is_admin != 0,
            ))
        } else {
            None
        }
    }
}
