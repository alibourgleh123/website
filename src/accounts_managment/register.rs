use rusqlite::{Connection, Error as RusqliteError};
use std::fmt::{self, write};

use super::{hash_account_details, Account};

#[derive(Debug)]
pub enum RegistrationError {
    InvalidUsername,
    UsernameAlreadyExists,
    InvalidPassword,
    InvalidToken,
    DatabaseError(RusqliteError),
}

impl fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegistrationError::InvalidUsername => write!(f, "Invalid username."),
            RegistrationError::UsernameAlreadyExists => write!(f, "Username already exists."),
            RegistrationError::InvalidPassword => write!(f, "Invalid password."),
            RegistrationError::InvalidToken => write!(f, "Invalid Token"),
            RegistrationError::DatabaseError(err) => write!(f, "Database error: {}", err),
        }
    }
}

impl From<RusqliteError> for RegistrationError {
    fn from(err: RusqliteError) -> Self {
        RegistrationError::DatabaseError(err)
    }
}

pub fn register(connection: &Connection, username: &str, password: &str, token: &str) -> Result<(), RegistrationError> {
    if username.chars().count() <= 3 {
        return Err(RegistrationError::InvalidUsername);
    }

    if username.chars().count() <= 3 {
        return Err(RegistrationError::InvalidPassword);
    }

    if token.chars().count() == 0 {
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

    match connection.execute(
        "INSERT INTO accounts (username, password, token, type) VALUES (?1, ?2, ?3, ?4)",
        (account.username, account.password, token, "Guest"), // By default every account is a guest
    ) {
        Ok(_) => Ok(()),
        Err(RusqliteError::SqliteFailure(err, _)) if err.extended_code == rusqlite::ffi::SQLITE_CONSTRAINT_UNIQUE => {
            Err(RegistrationError::UsernameAlreadyExists)
        }
        Err(e) => Err(e.into()),
    }
}
