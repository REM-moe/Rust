enum BoxColor {
    Red,
    Blue,
    Green,
    Yellow,
}

struct Box {
    height: f32,
    width: f32,
    color: BoxColor,
}

impl Box {
    fn get_color(&self) {
        match self.color {
            BoxColor::Red => println!("Red colored Box!!"),
            BoxColor::Blue => println!("Blue colored Box!!"),
            BoxColor::Green => println!("Green colored Box!!"),
            BoxColor::Yellow => println!("Yellow colored Box!!"),
        }
    }
    fn get_attrs(&self) {
        println!(
            "Height Of Box: {}\n Width of Box: {}",
            self.height, self.width
        );
    }
    fn create(h: f32, w: f32, clr: BoxColor) -> Self {
        Self {
            height: h,
            width: w,
            color: clr,
        }
    }
}

fn main() {
    let box1 = Box {
        height: 23.9,
        width: 14.8,
        color: BoxColor::Red,
    };
    let box2 = Box::create(33.0, 44.9, BoxColor::Yellow);
    box1.get_attrs();
    box1.get_color();
    box2.get_attrs();
    box2.get_color();
    print!("----------------------------------------------------------------------------------");
}
