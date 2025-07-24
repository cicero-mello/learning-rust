use crate::crypto::aes_gcm::decrypt_content;
use crate::crypto::filename;
use crate::crypto;
use crate::crypto::argon2;
use crate::file_manipulation::{create_file, read_file};

use aes_gcm::{
    aead::{ Nonce },
    Aes256Gcm
};
use ::argon2::password_hash::SaltString;

pub fn create_vault(name: &str, password: &str) {
    let encoded_name = filename::encode(&name);
    let argon2_response = argon2::generate_key(&password, None);

    let key = argon2_response.key;
    let salt_string = argon2_response.salt_string;

    let argon2_2_response = argon2::generate_key(&password, Some(&salt_string));

    println!("sault a2 1:\n{}", &salt_string);
    println!("sault a2 2:\n{}", &argon2_2_response.salt_string);


    // Extrai o salt em bytes crus (binários)
    let mut salt_bytes = vec![0u8; 16]; // tamanho típico do salt
    salt_string.decode_b64(&mut salt_bytes).expect("Failed to decode salt base64");

    println!("Salt Bytes:\n{:?}", salt_bytes);

    let aes_response = crypto::aes_gcm::encrypt_content(&"vc não viu nada 🤫".to_string(), key, None);

    // println!("Nonce:\n{:?}", &aes_response.nonce_96bit);

    let cipher_text = aes_response.cipher_text;
    let nonce_bytes = aes_response.nonce_96bit.as_slice();

    println!("Nonce Bytes:{:?}\n", &nonce_bytes);

    println!("Cipher Text Bytes:\n{:?}", &cipher_text);


    // Monta o conteúdo final: salt binário + nonce + cipher text
    let mut content: Vec<u8> = Vec::new();
    content.extend_from_slice(&salt_bytes);   // 16 bytes normalmente
    content.extend_from_slice(nonce_bytes);   // 12 bytes
    content.extend_from_slice(&cipher_text);  // tamanho variável

    create_file(".vts/", &encoded_name, &content);

    println!("Vault Created!");

    let file_data = read_file(".vts/", &encoded_name);

    let s_string = SaltString::encode_b64(&file_data.salt).expect("error");

    let hash = argon2::generate_key(password, Some(&s_string)).key;

    let n_valid = Nonce::<Aes256Gcm>::from_slice(&file_data.nonce);

    let decripted = decrypt_content(file_data.cipher_text, hash, n_valid);

    println!("\n\n{:?}", decripted);
}
