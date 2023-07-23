use VENDING_MACHINE::{
    database::database::Database,
    menus::menus::login_register_menu
    };
use rusqlite::Error;



fn main() -> Result<(), Error> {
    /* let mut item = Saldo::new();
 */
    let db = Database::new("database.db")?;

    let _ = db.create_tables();

    login_register_menu(&db);

    Ok(())
}
