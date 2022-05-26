use std::collections::HashMap;
fn main() {
    let mut hm = HashMap::new();
    hm.insert(11, "eleven");
    hm.insert(22, "twelve");

    let vec = vec![12,44,54,87,5];

    for (i, num) in hm.iter().enumerate() {
        println!("-> {:?},{:?} <--", i, num)
    }
    for (i, num) in vec.iter().enumerate() {
        println!("-> {:?},{:?} <-", i, num)
    }
}
