enum Colours {
    Red,
    Yellow,
    Blue,
    Green,
}

impl Colours {
    fn fav_color(color: &Colours) -> &str {
        match color {
            Colours::Red => "Favoutite color: Red",
            Colours::Blue => "Favoutite color: Blue",
            Colours::Green => "Favoutite color: Green",
            Colours::Yellow => "Favoutite color: Yellow",
        }
    }
}

struct Person {
    name: String,
    age: i32,
    fav_color: Colours,
}

impl Person {
    fn new(name: &str, age: i32, color: Colours) -> Self {
        Self {
            name: name.to_owned(),
            age: age,
            fav_color: color,
        }
    }

    fn name(n: &Self) {
        println!("Name: {:?}", n.name);
    }

    fn age(a: &Self) {
        println!("Age: {:?}", a.age);
    }
}

fn print_info(prsn: &Vec<Person>) {
    for info in prsn {
        if info.age < 10 {
            Person::name(&info);
            Person::age(&info);
            println!("{:?}\n", Colours::fav_color(&info.fav_color));
        } else {
            println!("{:?}, older than 10 \n", info.name);
        }
    }
}

fn main() {
    let person = vec![
        Person::new("abhay", 19, Colours::Green),
        Person::new("xyz", 23, Colours::Yellow),
        Person::new("qwerty", 5, Colours::Blue),
        Person::new("abcd", 14, Colours::Red),
    ];
    print_info(&person);
}
