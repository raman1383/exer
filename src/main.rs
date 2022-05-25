use std::collections::HashMap;

use exer::coin_change;
use exer::memoized_fibonacci;

fn main() {
    println!("{}", memoized_fibonacci(6));
    println!("{}", coin_change(&[12,2,1,1,3], 2).unwrap());

    let mut hash_map = HashMap::new();
    hash_map.insert(0, "zero");
    hash_map.insert(1, "ONE");
    hash_map.insert(2, "TWO");
    hash_map.insert(3, "THREE");

    println!("{:?}", hash_map.get(&2));
}
