
use std::process::exit;

use rusqlite::{params, Connection, Result};

use crate::{accounts_managment::{register::register, Role}, config};

pub fn init_database() {
    // Crashing the program on error is fine here
    let connection = Connection::open("accounts.db").expect("failed to open accounts.db");

    if connection.execute(
        "CREATE TABLE IF NOT EXISTS accounts (
            id    INTEGER PRIMARY KEY,
            username  TEXT NOT NULL,
            password  TEXT NOT NULL,
            token     TEXT NOT NULL,
            role      TEXT NOT NULL,
            creation_date TEXT NOT NULL
        )",
        ()).is_err() {
        eprintln!("failed to create accounts table in the database, try to delete accounts.db file and rerun the program.");
        exit(-1);
    }

    if connection.execute(
        "CREATE TABLE IF NOT EXISTS form (
            id    INTEGER PRIMARY KEY,
            name          TEXT NOT NULL,
            speciality    TEXT NOT NULL,
            residence     TEXT NOT NULL,
            phone_number  TEXT NOT NULL,
            email         TEXT NOT NULL,
            more          TEXT NOT NULL,
            time          TEXT NOT NULL,
            uuid          TEXT NOT NULL
        )",
        ()).is_err() {
        eprintln!("failed to create form table in the database, try to delete accounts.db file and rerun the program.");
        exit(-1);
    }

    if connection.execute("
    CREATE TABLE IF NOT EXISTS consultations (
        id              INTEGER PRIMARY KEY,
        targeted_doctor TEXT NOT NULL,
        name            TEXT NOT NULL,
        sur_name        TEXT NOT NULL,
        email           TEXT NOT NULL,
        phone_number    TEXT NOT NULL,
        issue           TEXT NOT NULL,
        time            TEXT NOT NULL,
        uuid            TEXT NOT NULL
    )", ()).is_err() {
        eprintln!("failed to create consultations table in the database, try to delete accounts.db file and rerun the program.");
        exit(-1);
    }

    if config::CREATE_ADMIN_ACCOUNT_IF_IT_DOESNT_EXIST {
        let mut statement = match connection.prepare(
            "SELECT EXISTS (
                SELECT 1 
                FROM accounts 
                WHERE username = ?1
            )",
        ) {
            Ok(statement) => statement,
            Err(e) => {
                eprintln!("failed to prepare the statement in init_database() function, here is the error {}", e);
                exit(-1);
            }
        };

        // Check if the account exists
        let username_exists: bool = match statement.query_row(
            params![config::ADMIN_ACCOUNT_USERNAME],
            |row| row.get(0),
        ) {
            Ok(access) => access,
            Err(e) => {
                eprintln!("failed to check if USERNAME exists in database, here is the error: {}", e);
                exit(-1);
            }
        };

        if !username_exists {
            if let Err(e) = register(&connection, config::ADMIN_ACCOUNT_USERNAME, config::ADMIN_ACCOUNT_PASSWORD, "سيد البراغل", Role::Admin) {
                eprintln!("we are init_database function, register failed!, here is the error:\n{:?}", e);
                exit(-1);
            }
        }
    }
}

pub fn get_database_connection() -> Result<Connection> {
    Ok(Connection::open("accounts.db")?)
}