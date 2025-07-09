// "use" são como os imports
// std = standard | io = input/output
use std::io;

fn main() {

    // Trick p/ "limpar" o terminal (coloca a proxima linha no topo)
    print!("\x1B[2J\x1B[1;1H\n");

    // Borda bonitinha
    println!("{}\n", "▀".repeat(56));

    println!("I will guess your name 😤!");
    println!("First, tell me: how your name looks when reversed?");
    println!("(ex: John, looks like Nhoj)\n");

    // Criação de um tipo String (é diferente de "str")
    let mut reverse_name = String::new();

    io::stdin() // <- Acessa a entrada padrão (teclado), retorna um obj de leitura
        .read_line(&mut reverse_name) // <- Lê a digitação até usuario pressionar Enter, e armazena em "reverse_name"
        .expect("Failed to read the your reverse name.") // <- Mensagem de erro padrão
    ;
    // "&mut reverse_name" é uma referência mutável à estrutura String completa (de "reverse_name").
    // & = serve pra pegar a ref de uma variável
    // mut = diz que a variável pode ser alterada
    // &mut = permite alterar o conteúdo a qual a referencia indica

    let mut normal_name = String::new();


    for (i, letter) in reverse_name.trim().chars().rev().enumerate() {
        if i == 0 {
            normal_name.extend(letter.to_uppercase());
        }
        else {
            normal_name.extend(letter.to_lowercase());
        }
    }

    println!("\nYour name is: {}! 😌\n", normal_name);

    // Borda bonitinha
    println!("{}\n", "▄".repeat(56));
}

/*
    reverse_name.trim() = remove vazios do inicio e fim da string

    .chars() = transforma a String em um iterador de caracteres (char).
    cada letra vira um item individual, respeitando acentuação e Unicode.

    .rev() = inverte a ordem dos caracteres.

    .enumerate() = adiciona um índice a cada caractere.
    Ex. de resultado do iterador: (0, 'A'), (1, 'n'), (2, 'a'), etc.

    -----

    .extend = serve p/ add vários items de uma vez em uma coleção, String...
    ali é necessário pois o to_lowercase e to_uppercase retornam uma coleção
    em vez de apenas um char. Isso ocorre pois alguns caracteres, quando
    passam por tal função, retorna algo maior que um char.
    ex.:

    let s = 'ß'.to_uppercase().collect::<String>();
    println!("{}", s); // imprime "SS"

    caso fosse o cenário de adicionar só 1 char (garantido),
    em vez de usar "extend", poderia ser usado "push"
*/
