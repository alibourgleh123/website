use crate::config::USE_HTTPS;

use super::{get_database_connection, hash_account_details, Account, Role};

use actix_web::{cookie::Cookie, post, web, HttpResponse, Responder};
use chrono::Local;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

// RegisterResponse errors follow the index of this enum
// For example error code 2 = InvalidPassword
pub enum RegistrationError {
    InvalidUsername,
    InvalidPassword,
    InvalidToken,
    UsernameAlreadyExists,
    DatabaseError
}

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    password: String,
    token: String
}

#[derive(Serialize)]
struct RegisterResponse {
    server_returned_an_error: bool,
    error_code: u8
}

#[post("/register")]
pub async fn register_endpoint(info: web::Json<RegisterRequest>) -> actix_web::Result<impl Responder> {
    let connection = get_database_connection().unwrap();

    let register_result = register(&connection, &info.username, &info.password, &info.token);

    let error_code = match register_result {
        Ok(_) => 0,
        Err(RegistrationError::InvalidUsername) => 1,
        Err(RegistrationError::InvalidPassword) => 2,
        Err(RegistrationError::InvalidToken) => 3,
        Err(RegistrationError::UsernameAlreadyExists) => 4,
        Err(RegistrationError::DatabaseError) => 5,
    };

    // Auth success
    if error_code == 0 {
        let auth_cookie = Cookie::build("auth_token", &info.token)
                    .path("/")
                    .http_only(true)
                    .secure(USE_HTTPS)  // Set to true if using HTTPS
                    .finish();

        return Ok(HttpResponse::Found()
                .append_header(("Location", "/ar/main"))
                .cookie(auth_cookie)
                .finish());
    }

    Ok(HttpResponse::InternalServerError().json(RegisterResponse {
        server_returned_an_error: error_code != 0,
        error_code,
    }))
}

pub fn register(connection: &Connection, username: &str, password: &str, token: &str) -> Result<(), RegistrationError> {
    if username.chars().count() < 3 || username.chars().count() > 16 {
        return Err(RegistrationError::InvalidUsername);
    }

    if username.chars().count() < 3 || username.chars().count() > 16 {
        return Err(RegistrationError::InvalidPassword);
    }

    if token.chars().count() == 0 || token.chars().count() > 64 {
        return Err(RegistrationError::InvalidToken);
    }

    // TODO: use account struct only if the user wants to hash, To improve performance
    let mut account = Account {
        username: username.to_string(),
        password: password.to_string()
    };

    if crate::config::HASH_IMPORTANT_INFORMATION {
        hash_account_details(&mut account);
    }

    // Check if the username already exists
    let mut statement = connection.prepare(
        "SELECT EXISTS (
            SELECT 1 
            FROM accounts 
            WHERE username = ?1
        )",
    ).unwrap();
    
    let username_exists: bool = statement.query_row(
        params![username],
        |row| row.get(0),
    ).unwrap();

    if username_exists {
        return Err(RegistrationError::UsernameAlreadyExists);
    }

    // Get current time 
    let now = Local::now();
    let formatted_time = now.format("%Y/%m/%d %I:%M:%S %p").to_string();

    match connection.execute(
        "INSERT INTO accounts (username, password, token, role, creation_date) VALUES (?1, ?2, ?3, ?4, ?5)",
        (account.username, account.password, token, Role::Guest.to_string(), formatted_time), // By default every account is a guest
    ) {
        Ok(_) => Ok(()),
        Err(_) => Err(RegistrationError::DatabaseError),
    }
}
