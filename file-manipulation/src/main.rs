mod utils;
use std::io::stdin;

fn main() {
    print!("\x1B[2J\x1B[1;1H\n");

    println!("Let's create a file! 😎📄\n");

    println!("Type your text:");
    let mut text = String::new();
    stdin().read_line(&mut text).expect("Error");

    println!("\nType your file name:");
    let mut file_name = String::new();
    stdin().read_line(&mut file_name).expect("Error");

    println!("\nType the folder to put the file in:");
    println!("(ex: outputs/secrets)");
    let mut folder = String::new();
    stdin().read_line(&mut folder).expect("Error");

    utils::create_file(
        folder.as_str().trim(),
        file_name.as_str().trim(),
        text.as_str()
    );
}
