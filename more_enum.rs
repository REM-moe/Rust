#[derive(Debug)]
enum AnotherEnum {
    New,
    Holiday(String),
}

#[derive(Debug)]
enum Discount {
    Percent(f32),
    Flat(i32),
    Promo(AnotherEnum),
    Custom(String),
}

impl Discount {
    fn data(info: &Self) {
        println!("{:?}", info);
    }
}

fn main() {
    let a: Discount = Discount::Percent(23.0);
    let b: Discount = Discount::Promo(AnotherEnum::Holiday("Summer".to_owned()));
    let c = Discount::Promo(AnotherEnum::Holiday(String::from("Winter")));
    Discount::data(&a);
    Discount::data(&b);
    Discount::data(&c);
}
