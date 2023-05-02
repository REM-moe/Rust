#[derive(Debug)]
enum Color {
    Red,

    Yellow,
    Green,
}

fn main() {
    let color = Color::Red;

    match color {
        Color::Red => println!("Red Color"),
        Color::Yellow => println!("Yellow Color"),
        Color::Green => println!("Green Color"),
    }
}
