use std::io;

use crypto::scrypt;
use log::error;

pub fn hash_password(password: &str) -> io::Result<String> {
    scrypt::scrypt_simple(password, &scrypt::ScryptParams::new(14, 16, 1))
}

pub fn check_password(password: &str, hash: &str) -> bool {
    scrypt::scrypt_check(password, hash).unwrap_or_else(|e| {
        error!("Error checking password: {}", e);
        false
    })
}
