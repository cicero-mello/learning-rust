use std::io::stdin;

mod utils;
use crate::utils::{
    benchmark_argon2id,
    estimate_times
};

fn main() {
    print!("\x1B[2J\x1B[1;1H\n");

    let base = benchmark_argon2id();

    let test_params = vec![
        (19 * 1024, 1),   // 19 MiB, 1 iterações
        (19 * 1024, 2),   // 19 MiB, 2 iterações
        (64 * 1024, 2),   // 64 MiB, 2 iterações
        (128 * 1024, 3),  // 128 MiB, 3 iterações
        (256 * 1024, 4),  // 256 MiB, 4 iterações
    ];

    println!("────────────────────────────────────────────────────────────");
    println!("Simplified Time Estimates\n");

    for (memory_cost, iterations) in test_params {
        let (
            estimated_hash_generation_time,
            estimated_hash_verification_time
        ) = estimate_times(&base, memory_cost, iterations);
        println!(
            "Memory Cost: {} KiB & Iterations: {} & Parallelism: 1",
            memory_cost, iterations
        );
        println!("Hash Generation: {:?}", estimated_hash_generation_time);
        println!("Hash Verification: {:?}\n", estimated_hash_verification_time);
    };

    println!("────────────────────────────────────────────────────────────\n");

    println!("Press ENTER to exit");
    let mut buffer = String::new();
    let _ = stdin().read_line(&mut buffer);
}
