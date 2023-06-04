fn main() {
    let owned_string = "Owned String".to_owned();
    let sstring = String::from("another");
    printf_data(&owned_string);
    printf_data(&sstring);
    let msg = "Abhay".to_owned();
    let word = Employee::new(msg);
    Employee::print_name(&word);
}

fn printf_data(data: &str) {
    println!("{:?}", data);
}

struct Employee {
    name: String,
}

impl Employee {
    fn print_name(wrd: &Self) {
        println!("Employee Name: {:?}", wrd.name);
    }

    fn new(n: String) -> Self {
        Self { name: n }
    }
}
