struct Rectangle {
    width: u32,
    height: u32,
    name: String,
}

impl Rectangle {
    // "Methods" possuem &self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn bigger_then(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }

    // "Associated Functions" não possuem &self
    fn create_square(size: u32, name: String) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
            name
        }
    }
}

fn main() {
    print!("\x1B[2J\x1B[1;1H\n");

    let mut r_1 = Rectangle {
        width: 10,
        height: 20,
        name: "My Rectangle".to_string()
    };
    println!("w: {} | h: {} | n: {}", r_1.width, r_1.height, r_1.name);

    r_1.name = "Our Rectangle".to_string();

    println!("New Name: {}", r_1.name);

    println!("Area: {}", r_1.area());

    let r_2 = Rectangle {
        name: "small".to_string(),
        width: 1,
        height: 2
    };

    println!("R1 > R2 ? {}", r_1.bigger_then(&r_2));
    println!("R2 > R1 ? {}", r_2.bigger_then(&r_1));

    let square = Rectangle::create_square(10, "My Square".to_string());
    println!("w: {} | h: {} | n: {}", square.width, square.height, square.name);
}
