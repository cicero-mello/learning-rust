use aes_gcm::{
    Aes256Gcm,
    aead::{
        // AeadCore,
        // OsRng,
        Nonce
    }
};

use rand::RngCore;
use std::time::{ SystemTime, UNIX_EPOCH };

// Geração de Nonce Default
// pub fn generate_nonce () -> Nonce<Aes256Gcm> {
//     let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
//     return nonce;
// }

// Geração de Nonce Customizada Baseada em Timestamp (6 bytes) + Aleatoriedade (6 bytes)
/*
    Garante a não colisão (a menos que a maquina volte no tempo)
    sem precisar comparar valores anteriores, e ainda possui um
    valor moderado de entropia gerado pelos 6 bytes random.
    Além disso, o tempo embutido facilita pra auditar caso
    surja um problema de repetição (por bug, feitiço, ou maldição)
 */
pub fn generate_nonce() -> Nonce<Aes256Gcm> {
    // Tempo desde a Unix epoch em milissegundos (u64)
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).expect("Error");
    let timestamp_ms = duration.as_millis() as u64;

    // Converte o timestamp para bytes e usa apenas os 6 mais significativos
    let timestamp_bytes = &timestamp_ms.to_be_bytes()[2..8]; // 6 bytes

    // Gera 6 bytes aleatórios
    let mut random_bytes = [0u8; 6];
    rand::rng().fill_bytes(&mut random_bytes);

    // Constrói o vetor de 12 bytes para o nonce
    let mut nonce_bytes = [0u8; 12];
    nonce_bytes[..6].copy_from_slice(timestamp_bytes); // primeiros 6 bytes = timestamp
    nonce_bytes[6..].copy_from_slice(&random_bytes);   // últimos 6 bytes = aleatório

    Nonce::<Aes256Gcm>::from_slice(&nonce_bytes).to_owned()
}
