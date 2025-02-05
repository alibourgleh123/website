/*
 * This setting controls if stuff passwords are hashed and salted or not,
 * Hashing and salting is very important for website security, It stands as the last defense against hackers,
 * and doesnt let them get away with the passwords if the database got leaked, But its very heavy on server resources, 
 * Hashing is a very intensive operation on the CPU, anyways because I dont care about security, its off by default
 * default to not waste my precious performance on useless things (FYI I always use my pc with windows defender off
 * and if my system didnt setup a firewall I dont setup one because its annoying)
 * 
 * WARNING: ANY USER THATS REGISTERED ON TO THE SERVER WHILE HASHING WAS ENABLED WILL NEED TO REGISTER AGAIN IF YOU DISABLED HASHING!
 * AND VICE VERSA!
 */
pub const HASH_IMPORTANT_INFORMATION: bool = false;

