use std::io;
use rand::Rng;

fn get_user_input_number() -> u32 {
    loop {
        let mut handle_number = String::new();
        io::stdin().read_line(&mut handle_number).expect("Fail to read");

        let parse_result = handle_number.trim().parse::<u32>();

        match parse_result {
            Ok(number_parsed) => {
                return number_parsed;
            },
            Err(_) => {
                println!("\nInvalid Input! Try again:");
            }
        };
    };
}

fn main() {
    print!("\x1B[2J\x1B[1;1H\n");

    println!("I thinking in a number 🤔.");
    println!("(It's between 1 and 100)\n");
    println!("Type your guess:");

    let thought_number = rand::rng().random_range(0..=100);
    let mut attempts: u8 = 0;

    loop {
        let handle_number = get_user_input_number();
        attempts += 1;

        if handle_number == thought_number {
            break;
        }
        println!("\nWRONG! 🤣🤣🤣");
        if handle_number > thought_number {
            println!("Try a small number:");
        }
        else if handle_number < thought_number {
            println!("Try a bigger number:");
        }
    };

    println!("\n🥳 You Win! 🎉");
    println!("Total Attempts: {}\n", attempts);
}
