#[derive(Debug, Copy, Clone)]
enum Position {
    Manager,
}

#[derive(Debug, Copy, Clone)]
struct EmpInfo {
    position: Position,
    age: i32,
}

fn print_struct(info: EmpInfo) {
    println!("{:?}", info);
}

fn new(age: i32, pos: Position) -> EmpInfo {
    EmpInfo {
        age: age,
        position: pos,
    }
}

fn print_position(data: Position) {
    println!("{:?}", data);
}

fn main() {
    let emp1: EmpInfo = new(20, Position::Manager);
    print_struct(emp1);
    print_position(emp1.position);
    println!("{:?}", emp1);
}
