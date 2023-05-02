fn print_things(items: &Vec<i32>) {
    for num in items {
        match num {
            30 => println!("Thirty"),
            _ => println!("{:?}", num),
        }
    }
}

fn main() {
    let items = vec![10, 20, 30, 40];
    print_things(&items);
    println!("Number Of Items: {:?}", items.len());
}
