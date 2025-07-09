/*
    Obs. sobre Funções:
    - Não há parâmetros opcionais em Rust.
      (mas da pra chegar em um algo assim
      utilizando Structs, verei mais adiante)
    - sempre é necessário inferir o tipo tanto
      dos parâmetros, quanto do retorno.
    - existem 3 formas de "return"
*/

fn sum(first_value: i32, second_value: i32) -> i32 {
    first_value + second_value
}

fn subtract(first_value: i32, second_value: i32) -> i32 {
    return first_value - second_value;
}

fn multiply(first_value: i32, second_value: i32) -> i32 {
    return first_value * second_value
}

fn main() {
    print!("\x1B[2J\x1B[1;1H\n");

    println!("3 + 5 = {}", sum(3, 5));
    println!("3 - 4 = {}", subtract(3, 4));
    println!("3 * 2 = {}", multiply(3, 2));

    let fruits: u8 = {
        let apples: u8 = 4;
        let bananas: u8 = 3;

        apples + bananas
    };

    println!("{}", fruits);
}

/*
    Todo escopo, por padrão, retorna a ultima linha
    caso a mesma tenha algo que possa ser retornado
    e não termine em ";", por isso, "sum" funciona,
    assim como "fruits".

    Ainda assim, no caso das funções,
    é possível usar a escrita "return",
    pra dar aquele up na legibilidade.
    (Apenas nas funções! o uso de return em
    value_from_scope não funcionará!)
*/
