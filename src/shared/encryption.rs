use magic_crypt::{new_magic_crypt, MagicCryptTrait};

use super::types::ApiErrors;

pub fn encrypt_data(data: &String) -> String {
    // Encrypt Password with Symmetric Key
    let mc = new_magic_crypt!("magickey", 256); // TODO: Make Key an env
    mc.encrypt_to_base64(data)
}

pub fn decrypt_password(encrypted: &String) -> Result<String, ApiErrors> {
    let mc = new_magic_crypt!("magickey", 256);
    match mc.decrypt_base64_to_string(encrypted) {
        Ok(res) => Ok(res),
        Err(_) => Err(ApiErrors::ServerError("There was an issue decrypting".to_string()))
    }
}