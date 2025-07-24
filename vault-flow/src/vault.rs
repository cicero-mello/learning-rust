use crate::crypt::aes_gcm::decrypt_content;
use crate::crypt::filename;
use crate::crypt;
use crate::crypt::argon2::{self};
use crate::file_manipulation::{create_file, list_files, read_vault_file};

use aes_gcm::{
    aead::{ Nonce },
    Aes256Gcm
};
use ::argon2::password_hash::SaltString;

pub fn create(name: &str, password: &str) {
    let encoded_name = filename::encode(&name);
    let argon2_response = argon2::generate_key(&password, None);

    let key = argon2_response.key;
    let salt = argon2_response.salt;

    let aes_response = crypt::aes_gcm::encrypt_content(
        "placeholder content 📄", key, None
    );

    let cipher_text = aes_response.cipher_text;
    let nonce= aes_response.nonce;

    let mut content: Vec<u8> = Vec::new();
    content.extend_from_slice(&salt);
    content.extend_from_slice(&nonce);
    content.extend_from_slice(&cipher_text);

    create_file(".vts/", &encoded_name, &content);
    println!("Vault Created!");
}

pub fn get_content (vault_name: &str, password: &str) -> String {
    let encoded_name = filename::encode(vault_name);
    let file_data = read_vault_file(".vts", &encoded_name);

    let salt_string = SaltString::encode_b64(&file_data.salt).expect("error");

    let key = argon2::generate_key(password, Some(&salt_string)).key;
    let nonce = Nonce::<Aes256Gcm>::from_slice(&file_data.nonce);

    let decrypted = decrypt_content(file_data.cipher_text, key, nonce);

    return decrypted;
}

pub fn list_all() -> Vec<String> {
    let files = list_files(".vts").expect("Error");
    let decoded_files = files.into_iter().map(
        |file_name| filename::decode(&file_name)
    ).collect::<Vec<String>>();

    return decoded_files;
}

pub fn already_exists (vault_name: &str) -> bool {
    let all_vaults = list_all();
    for vault in all_vaults {
        if vault == vault_name {
            return true
        }
    }
    return false;
}
