
use std::process::exit;

use rusqlite::{Connection, Result};

pub fn init_database() {
    // Crashing the program on error is fine here i guess
    let conn = Connection::open("accounts.db").expect("failed to open accounts.db");

    if conn.execute(
        "CREATE TABLE IF NOT EXISTS accounts (
            id    INTEGER PRIMARY KEY,
            username  TEXT NOT NULL,
            password  TEXT NOT NULL,
            token     TEXT NOT NULL,
            role      TEXT NOT NULL,
            creation_date TEXT NOT NULL
        )",
        ()).is_err() {
        println!("failed to create accounts table in the database, try to delete accounts.db file and rerun the program.");
        exit(-1);
    }

    if conn.execute(
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
        println!("failed to create form table in the database, try to delete accounts.db file and rerun the program.");
        exit(-1);
    }

    if conn.execute("
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
        println!("failed to create consultations table in the database, try to delete accounts.db file and rerun the program.");
        exit(-1);
    }
}

pub fn get_database_connection() -> Result<Connection> {
    Ok(Connection::open("accounts.db")?)
}