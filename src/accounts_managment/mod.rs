pub mod login;
pub mod register;
pub mod misc_endpoints;

use rusqlite::Connection;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use serde::Serialize;


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

pub enum Role {
    Admin,
    Guest
}

#[derive(Serialize)]
pub struct UserInfo {
    user_exists: bool,
    username: String,
    role: String,
    creation_date: String,
}

pub fn get_user_info(connection: &Connection, token: String) -> UserInfo {
    let mut statement = match connection.prepare("SELECT username, role, creation_date FROM accounts WHERE token = ?") {
        Ok(statement) => statement,
        Err(e) => {
            eprintln!("We are the main_handler function, We have failed to prepare the statement, here is the error:\n{}", e);
            // Default case if no user is found
            return UserInfo {
                user_exists: false,
                username: "حدث خطأ بالخادم".to_string(),
                role: "حرامي سيارات".to_string(),
                creation_date: "ظظظ".to_string(),
            };
        }
    };

    let user_info_iter = statement.query_map([&token], |row| {
        Ok(UserInfo {
            user_exists: true,
            username: row.get(0)?,
            role: row.get(1)?,
            creation_date: row.get(2)?,
        })
    });

    match user_info_iter {
        Ok(mut iter) => {
            if let Some(Ok(user_info)) = iter.next() {
                return user_info;
            }
        }
        Err(e) => {
            eprintln!("We are the main_handler function, We have a database error: {}", e);
        }
    }

    // Default case if no user is found
    UserInfo {
        user_exists: false,
        username: "Unknown".to_string(),
        role: "حرامي سيارات".to_string(),
        creation_date: "ظظظ".to_string(),
    }
}


// Leave these ugly beings below
impl Role {
    pub fn to_string(&self) -> String {
        match self {
            Role::Admin => "Admin".to_string(),
            Role::Guest => "Guest".to_string(),
        }
    }
}

impl From<String> for Role {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Admin" => Role::Admin,
            "Guest" => Role::Guest,
            _ => Role::Guest
        }
    }
}

impl From<&str> for Role {
    fn from(value: &str) -> Self {
        match value {
            "Admin" => Role::Admin,
            "Guest" => Role::Guest,
            _ => Role::Guest
        }
    }
}