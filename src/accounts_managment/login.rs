use crate::accounts_managment::{get_database_connection, verify, Account};

use actix_web::{post, web, Responder};
use rusqlite::{params, Connection, Result};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

pub fn login_with_token(connection: &Connection, token: &str) -> Result<bool> {
    let mut statement = connection.prepare(
        "SELECT EXISTS (
            SELECT 1 
            FROM accounts 
            WHERE token = ?1
        )",
    )?;
    
    // Check if the account exists
    let grant_access: bool = statement.query_row(
        params![token],
        |row| row.get(0),
    )?;

    Ok(grant_access)
}

#[derive(Deserialize)]
struct loginRequest {
    username: String,
    password: String,
    token: String
}

#[derive(Serialize)]
struct LoginResponse {
    access_granted: bool,
    server_returned_an_error: bool
}

#[post("/login")]
pub async fn login_endpoint(info: web::Json<loginRequest>) -> actix_web::Result<impl Responder> {
    let connection = get_database_connection().unwrap();

    let login = login(&connection, &info.username, &info.password, &info.token);

    match login {
        Ok(access_granted) => {
            return Ok(web::Json(LoginResponse {
                access_granted: access_granted,
                server_returned_an_error: false
            }));
        },
        Err(_) => {
            return Ok(web::Json(LoginResponse {
                access_granted: false,
                server_returned_an_error: true
            }));
        }
    }
}

pub fn login(connection: &Connection, username: &str, password: &str, token: &str) -> Result<bool> {
    if crate::config::HASH_IMPORTANT_INFORMATION {
        /*
         * This proccess is very intensive on performance, but its neccesary because we are hashing and salting
         * the user's password, which means that we need to itarate over every single account in the database and
         * call verify() on it to verify the logins.
         * I have benchmarked verify() function on my ryzen 5 5600 (12 threads 6 cores) cpu and managed to verify
         * 1000 strings in ~5.8 seconds with rayon's loops which we do use here, which is about ~172 strings per
         * second, good enough.
         */
        let grant_access = Arc::new(Mutex::new(false));

        let mut statement = connection.prepare("SELECT username, password FROM accounts")?;
        let accounts_iter = statement.query_map([], |row| {
            Ok(Account {
                username: row.get(0)?,
                password: row.get(1)?,
            })
        })?;

        let accounts: Vec<Account> = accounts_iter.collect::<Result<Vec<Account>, _>>()?;

        accounts.into_par_iter().for_each( |account| {
            if *grant_access.lock().unwrap() {
                // Dont waste computation power if access is already granted
                return;
            }

            // We will use nested if statements here so that we dont waste computational power verifying
            // the password when the username is wrong, we cant use 'continue;' in rayon loops, Rayon is used here
            // to split the verification proccess over all cpu cores, which significantly speeds it up
            if verify(username, &account.username) {
                if verify(password, &account.password) {
                    let mut grant_access = grant_access.lock().unwrap();
                    *grant_access = true;
                }
            }
        });

        if *grant_access.clone().lock().unwrap() {
            connection.execute(
                "UPDATE accounts
                    SET token = ?1
                    WHERE username = ?2;",
                (token, username),
            )?;
        }

        Ok(*grant_access.clone().lock().unwrap())
    } else {
        let mut statement = connection.prepare(
            "SELECT EXISTS (
                SELECT 1 
                FROM accounts 
                WHERE username = ?1 AND password = ?2
            )",
        )?;
        
        // Check if the account exists
        let grant_access: bool = statement.query_row(
            params![username, password],
            |row| row.get(0),
        )?;

        if grant_access {
            connection.execute(
                "UPDATE accounts
                    SET token = ?1
                    WHERE username = ?2;",
                (token, username),
            )?;
        }

        Ok(grant_access)
    }
}