use std::convert::TryInto;

fn binary_search(item: Vec<i32>, key: i32) {
    let mut low = 0;
    let mut high: usize = item.len().try_into().unwrap();
    while low != high {
        let mut mid = (low + high) / 2;
        if item[mid] == key {
            println!("Found {} at {} position", key, mid + 1);
            break;
        } else if item[mid] > key {
            high = mid - 1;
        } else if item[mid] < key {
            low = mid + 1;
        } else {
            println!("Element Not Found!!");
        }
    }
}

fn main() {
    let items = vec![0,1,2,3,4,5,6,7,8,9,10];
    binary_search(items, 1997);
}
