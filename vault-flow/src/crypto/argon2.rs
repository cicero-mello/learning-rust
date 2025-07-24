use aes_gcm::{Aes256Gcm, Key};
use argon2::{
    Algorithm, Argon2, Params, Version,
    password_hash::{rand_core::OsRng, SaltString},
};

pub struct GenerateKeyResponse {
    pub key: Key<Aes256Gcm>,
    pub salt_string: SaltString,
}

pub fn generate_key(
    password: &str,
    salt_option: Option<&SaltString>,
) -> GenerateKeyResponse {
    let argon2_params = Params::new(19 * 1024, 2, 1, Some(32)).expect("Error");
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, argon2_params);

    let salt_string = match salt_option {
        Some(s) => s.clone(),
        None => SaltString::generate(&mut OsRng),
    };

    let mut output_key = [0u8; 32];
    argon2.hash_password_into(
            password.as_bytes(),
            salt_string.as_str().as_bytes(),
            &mut output_key,
    ).expect("Error");

    let key = Key::<Aes256Gcm>::from_slice(&output_key).clone();

    return GenerateKeyResponse { key, salt_string };
}
