/*
    Em Rust, tem enum a rodo.
    Por exemplo, uma função com tipo de retorno variável,
    acaba retornando um enum que contem os tipos de retorno.
    Uma ação que pode gerar erro, retorna um enum que contem
    Ok (+ dados, quando der certo), ou Err (+ dados sobre o erro).

    Por isso, enums aqui são fundamentais, e tbm poderosos, pois
    a linguagem tem outros recursos que se relacionam a enums,
    como o "match".

    - Enums podem guardar dados em cada item (após o enum
    ser declarado).
*/

enum ChemicalElement {
    Oxygen,
    Carbon,
    Silver,
    Gold
}

// Função externa para obter os símbolos baseado no enum
fn get_chemical_symbol(chemical_element: ChemicalElement) -> &'static str {
    match chemical_element {
        ChemicalElement::Carbon => "C",
        ChemicalElement::Oxygen => "O",
        ChemicalElement::Silver => "Ag",
        ChemicalElement::Gold => "Au"
    }
}

// Função vinculada diretamente ao próprio enum
impl ChemicalElement {
    fn symbol(&self) -> &'static str {
        match self {
            ChemicalElement::Carbon => "C",
            ChemicalElement::Oxygen => "O",
            ChemicalElement::Silver => "Ag",
            ChemicalElement::Gold => "Au"
        }
    }

    /*
        "match" exige passar todos os possíveis valores
        pra cada item do enum de forma explicita, a não ser
        que seja utilizado o "_".
    */
    fn is_gas(&self) -> bool {
        match self {
            ChemicalElement::Oxygen => true,
            _ => false
        }
    }
}

use ChemicalElement::*; // permite usar os valores do enum sem o prefixo
use std::fmt; // Explico esse lá em baixo

fn main() {
    print!("\x1B[2J\x1B[1;1H\n");

    let oxygen = Oxygen;

    println!("Chemical Elements:\n");
    println!("Carbon: {}", get_chemical_symbol(ChemicalElement::Carbon));
    println!("Oxygen: {}", get_chemical_symbol(oxygen));
    println!("Silver: {}", get_chemical_symbol(Silver));
    println!("Gold:   {}\n", Gold.symbol());

    println!("Gold is gas? {}", Gold.is_gas());
    println!("Oxygen is gas? {}\n", Oxygen.is_gas());


    /*
        Abaixo, um ex de enum onde cada item pode ser vinculado
        um tipo de valor.
    */
    enum Message {
        Text(String),
        Image { url: String, width: u32, height: u32 },
        Reaction(char)
    }

    // Criando variáveis seguindo o enum (junto de seus tipos personalizados):
    let chat_message_1 = Message::Text("Heey, look out this photo!".to_string());
    let chat_message_2 = Message::Image {
        url: "imgfoda.com/mtofoda.png".to_string(),
        width: 1920,
        height: 1080
    };
    let chat_message_3 = Message::Reaction('😎');

    /*
        O print tem uma "interface" definida a respeito
        dos valores que serão exibidos (o nome correto
        pra isso em Rust é "trait").

        Portanto, se tentar fazer o print de algo vinculado
        a um enum personalizado, não funciona, conflita com a trait
        definida.

        Para solucionar isso, nós "estendemos" a Trait,
        basicamente, criamos regras de comportamento para
        cada um dos itens personalizados do nosso enum.
        Assim, podemos realizar o println! depois.
     */
    impl fmt::Display for Message {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Message::Text(text) => write!(f, "{}", text),
                Message::Image { url, width, height } => {
                    write!(f, "📷 Image: {} ({}x{})", url, width, height)
                },
                Message::Reaction(emoji) => write!(f, "Reacted with '{}'", emoji)
            }
        }
    }

    println!("\nChat:");
    println!("me: {}", chat_message_1);
    println!("me: {}", chat_message_2);
    println!("me: {}", chat_message_3);
}
