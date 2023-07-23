mod tests;

pub mod account {
    pub mod account;
    pub mod deposit;
}

pub mod purchase {
    pub mod purchase;
}

pub mod database {
    pub mod database;
}

pub mod menus {
    pub mod menus;
}

pub mod models {
    pub mod user_model;
    pub mod product_model;
    pub mod account_model;
    pub mod transactions_model;
}

pub mod authenticate {
    pub mod auth;
}

pub mod utils {
    pub mod read_input;
}