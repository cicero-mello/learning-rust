use crate::{crypt::diceware::generate, vault::already_exists};

mod crypt;
mod file_manipulation;
mod vault;

fn main() {
    print!("\x1B[2J\x1B[1;1H\n");

    println!("» Generating Social Media Vault Pass:");
    let dice_social_media = generate(8);
    println!("Diceware: {}", dice_social_media);

    println!("\n» Creating Vault 'Social Media':");
    vault::create("Social Media", &dice_social_media);


    println!("\n» Generating Bank Accounts Vault Pass:");
    let dice_bank_accounts = generate(8);
    println!("Diceware: {}", dice_bank_accounts);

    println!("\n» Creating Vault 'Bank Accounts':");
    vault::create("Bank Accounts", &dice_bank_accounts);


    println!("\n» Listing All Vaults:");
    let a = vault::list_all();
    println!("{:?}", a);


    println!("\n» Social Media Vault Content:");
    let social_media_content = vault::get_content("Social Media", &dice_social_media);
    println!("{}", social_media_content);


    println!("\n» Bank Accounts Vault Content:");
    let bank_account_content = vault::get_content("Bank Accounts", &dice_bank_accounts);
    println!("{}", bank_account_content);


    println!("\n» Vault 'Bank Accounts' Already Exists?");
    println!("{}", already_exists("Bank Accounts"));

    println!("\n» Vault 'Social Media' Already Exists?");
    println!("{}", already_exists("Social Media"));

    println!("\n» Vault 'House Docs' Already Exists?");
    println!("{}", already_exists("House Docs"));
}
