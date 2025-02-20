///
/// This setting controls if passwords are hashed and salted or not,
/// Hashing and salting is very important for website security, It stands as the last defense against hackers,
/// and doesnt let them get away with the passwords if the database got leaked, But its very heavy on server resources, 
/// ----
/// WARNING: ANY USER THATS REGISTERED ON TO THE SERVER WHILE HASHING WAS ENABLED WILL NEED TO REGISTER AGAIN IF YOU DISABLED HASHING!
/// AND VICE VERSA!
/// ----
pub const HASH_IMPORTANT_INFORMATION: bool = true;

///
/// This setting controls whenever HTTPS specific options are enabled or not in the webserver.
///
pub const USE_HTTPS: bool = false;

pub const CREATE_ADMIN_ACCOUNT_IF_IT_DOESNT_EXIST: bool = false;

pub const ADMIN_ACCOUNT_USERNAME: &str = "";
pub const ADMIN_ACCOUNT_PASSWORD: &str = "";