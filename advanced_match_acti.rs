#[derive(Debug)]
enum Ticketinfo {
    Vip(f32, String),
    Backstage(f32, String),
    Other(f32),
}

fn main() {
    let tickets = vec![
        Ticketinfo::Vip(700.9, String::from("CONCERT")),
        Ticketinfo::Backstage(000.0, String::from("CONCERT")),
        Ticketinfo::Other(300.9),
    ];
    for tic in tickets {
        match tic {
            Ticketinfo::Vip(_, _) => println!("{:?}", tic),
            other => println!("{:?}", other),
        }
    }
}
