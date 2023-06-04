#[derive(Debug)]
struct Item {
    name: String,
    quantity: i32,
}

impl Item {
    fn new(name: String, quantity: i32) -> Self {
        Self {
            name: name,
            quantity: quantity,
        }
    }
}

fn print_name(name: &str) {
    println!("{:?}", name);
}

fn print_items(items: &Vec<Item>) {
    for data in items {
        print_name(&data.name);
        println!("{:?}", data.quantity);
    }
}

fn main() {
    let reciept = vec![
        Item::new(String::from("mango"), 12),
        Item::new("Apple".to_owned(), 32),
    ];
    print_items(&reciept);
}
