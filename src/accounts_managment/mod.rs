pub mod login;
pub mod register;
pub mod endpoints;

use std::process::exit;

use rusqlite::{Connection, Result};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

#[derive(Debug)]
pub struct Account {
    username: String,
    password: String
}

pub fn hash_account_details(account: &mut Account) {
    let password_salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    account.password = argon2.hash_password(account.password.as_bytes(), &password_salt).expect("").to_string();
}

pub fn verify(unhashed_string: &str, hashed_string: &str) -> bool {
    let parsed_hash = PasswordHash::new(&hashed_string).expect("Unable to make password hash");
    Argon2::default().verify_password(unhashed_string.as_bytes(), &parsed_hash).is_ok()
}

pub fn init_database() {
    let conn = Connection::open("accounts.db").expect("failed to open accounts.db");

    if conn.execute(
        "CREATE TABLE IF NOT EXISTS accounts (
            id    INTEGER PRIMARY KEY,
            username  TEXT NOT NULL,
            password  TEXT NOT NULL,
            token     TEXT NOT NULL,
            type      TEXT NOT NULL
        )",
        (), // empty list of parameters.
    ).is_err() {
        println!("failed to create accounts table in the database, try to delete accounts.db file and rerun the program.");
        exit(-1);
    }
}

pub fn get_database_connection() -> Result<Connection> {
    Ok(Connection::open("accounts.db")?)
}