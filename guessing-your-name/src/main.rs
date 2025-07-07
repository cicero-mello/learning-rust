use std::io;

fn main() {
    println!("I will guess your name 😤!");
    println!("First, tell me: how your name looks when reversed?");
    println!("(ex: John, looks like Nhoj)\n");

    let mut reverse_name = String::new();

    io::stdin()
        .read_line(&mut reverse_name)
        .expect("Failed to read the your reverse name.")
    ;

    let mut normal_name = String::new();

    for (i, letter) in reverse_name.trim().chars().rev().enumerate() {
        if i == 0 {
            normal_name.extend(letter.to_uppercase());
        }
        else {
            normal_name.extend(letter.to_lowercase());
        }
    }

    println!("\nYour name is: {}! 😌", normal_name);
}
