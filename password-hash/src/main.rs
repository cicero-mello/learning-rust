use argon2::{
    Argon2,
    Params,
    Algorithm,
    Version,
    PasswordHasher, // precisa pro "hash_password"
    password_hash::{
        SaltString,
        rand_core::OsRng,
    }
};

fn main() {

    // Define os parâmetros com 19 MiB de memória (em KiB), 2 iterações, 1 thread
    // e tamanho de saída indefinido
    let argon2_params = Params::new(19 * 1024, 2, 1, None).expect("Error");
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, argon2_params);

    let salt = SaltString::generate(&mut OsRng);
    println!("\nSalt:\n{}", salt);

    let password_1 = "jus7_@_n0rm@1_p@55";
    println!("\nPassword:\n{}", password_1);

    let hash_password_1 = argon2.hash_password(password_1.as_bytes(), &salt).expect("Error");
    println!("\nHash:\n{}", hash_password_1);
}
