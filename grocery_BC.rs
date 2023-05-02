struct GroceryItem {
    id_number: i32,
    quantity: f32,
}

fn get_id(item: &GroceryItem) -> i32 {
    item.id_number
}

fn get_quantity(item: &GroceryItem) -> f32 {
    item.quantity
}

fn main() {
    let item_1 = GroceryItem {
        id_number: 12345,
        quantity: 34.05,
    };
    println!(
        "{}",
        format!("{}, {}, ", get_quantity(&item_1), get_id(&item_1))
    );
}
