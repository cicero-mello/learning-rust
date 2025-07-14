use std::time::{Duration, Instant};
use argon2::{
    Algorithm,
    Argon2,
    Params,
    PasswordHasher,
    PasswordVerifier,
    Version,
    password_hash::{rand_core::OsRng, SaltString}
};

pub struct BenchmarkResult {
    pub memory_cost_in_kib: u32,
    pub iterations: u32,
    pub average_hash_time: Duration,
    pub average_verify_time: Duration,
}

pub fn benchmark_argon2id() -> BenchmarkResult {
    println!("Starting Benchmark █ █ █\n");

    let benchmark_executions: u32 = 5;

    let memory_cost_in_kib: u32 = 19 * 1024;
    let iterations: u32 = 1;
    let parallelism: u32 = 1;

    let params = Params::new(
        memory_cost_in_kib,
        iterations,
        parallelism,
        Some(32)
    ).expect("Erro ao criar Params");

    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        params
    );

    let password = "benchmark_password".as_bytes();

    let mut total_hash_time = Duration::ZERO;
    let mut total_verify_time = Duration::ZERO;

    for _ in 0..benchmark_executions {
        let salt = SaltString::generate(&mut OsRng);

        let start_hashing = Instant::now();
        let hash = argon2.hash_password(password, &salt).expect("Hash Error");
        let hash_time = start_hashing.elapsed();
        total_hash_time += hash_time;

        let start_verifying = Instant::now();
        argon2.verify_password(password, &hash).expect("Verify Hash Error");
        let verify_time = start_verifying.elapsed();
        total_verify_time += verify_time;
    }

    let average_hash_time = total_hash_time / benchmark_executions;
    let average_verify_time = total_verify_time / benchmark_executions;

    println!(
        "Memory Cost: {} KiB\nIterations: {}\nParallelism: {}
        \nAverage Time:\nHash Generation: {:?}\nHash Verification: {:?}\n",
        memory_cost_in_kib, iterations, parallelism, average_hash_time, average_verify_time
    );

    BenchmarkResult {
        memory_cost_in_kib,
        iterations,
        average_hash_time,
        average_verify_time
    }
}

// Esse teste é um valor estimado baseado em
// linearidade, não corresponde aos valores reais de tempo,
// porém, servem pra ao menos dar uma noção
pub fn estimate_times(
    benchmark_base: &BenchmarkResult,
    custom_memory_cost_in_kib: u32,
    custom_iterations: u32,
) -> (Duration, Duration) {
    let scale =
        (custom_memory_cost_in_kib as f64 * custom_iterations as f64)
        /(benchmark_base.memory_cost_in_kib as f64 * benchmark_base.iterations as f64)
    ;

    let estimated_hash_generation_time = Duration::from_secs_f64(
        benchmark_base.average_hash_time.as_secs_f64() * scale
    );
    let estimated_hash_verification_time = Duration::from_secs_f64(
        benchmark_base.average_verify_time.as_secs_f64() * scale
    );

    return (
        estimated_hash_generation_time,
        estimated_hash_verification_time
    );
}
