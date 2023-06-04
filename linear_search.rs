fn linear_search(items: &Vec<i32>, key: &i32) {
    for index in 0..items.len() {
        if items[index] == *key {
            println!("Item: {} Found At Index Position {}! ", key, index + 1);
        }
    }
}

fn main() {
    let items = vec![0, 1, 2, 3];
    let key = 3;
    linear_search(&items, &key);
}
