struct SomeStruct {
    some_int: i32,
    some_float: f32,
}

impl SomeStruct {
    fn some_function(&self) {
        println!("Int = {:?}, Float = {:?}", self.some_int, self.some_float);
    }
    fn create_some() -> Self {
        Self {
            some_int: 1,
            some_float: 3.0,
        }
    }
    fn some_inrement(&mut self) {
        self.some_int += 1100;
        self.some_float -= 200.0;
    }
}

fn main() {
    let some_struct = SomeStruct {
        some_int: 3,
        some_float: 4.0,
    };
    some_struct.some_function();
    let mut second = SomeStruct::create_some();
    second.some_function();
    second.some_inrement();
    second.some_function();
}
