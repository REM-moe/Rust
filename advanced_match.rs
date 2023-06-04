#[derive(Debug)]
enum Discount {
    Flat(i32),
    Percent(f32),
}

#[derive(Debug)]
struct Ticket {
    event: String,
    tic_price: i32,
}

fn main() {
    let tic_price: i32 = 300;
    match tic_price {
        300 => println!("Paid 300"),
        other => println!("Paid {:?}", other),
    };

    let ticket = Ticket {
        event: String::from("Race"),
        tic_price: tic_price,
    };

    let flat: Discount = Discount::Flat(50);

    match flat {
        Discount::Flat(100) => println!("Lucky User!!!"),
        Discount::Percent(100.0) => println!("Thats Free!"),
        Discount::Flat(other_amnt) => println!(" Discount: {:?}", other_amnt),
        _ => (),
    };
    // Matching on structs

    match ticket {
        Ticket { tic_price, .. } => println!("Ignore Everything except {:?}", tic_price),
        Ticket {
            tic_price: 300,
            event,
        } => println!("Match On value: {:?} event: {:?}", tic_price, event),
    }
}
