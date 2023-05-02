1struct Drinks{
    flavour : Flavours,
    fluid_quant : f64,
}
enum Flavours{
    Sweet,
    Sour,
    Salty
}

fn drink(drink: Drinks){
    match drink.flavour {
        Flavours::Sweet => println!("Sweet, just like you!!"),
        Flavours::Sour => println!("This is Sour!!"),
        Flavours::Salty => println!("This is Salty!!")
    }
    println!("Quantity: {:#}", drink.fluid_quant);
}

fn main(){
    let mango = Drinks{
        flavour: Flavours::Sweet,
        fluid_quant: 69.00
    };
    drink(mango);
}
