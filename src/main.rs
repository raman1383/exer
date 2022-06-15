mod search;

fn main() {
    let x = 10;
    println!("hello {x} times ");

    println!(
        "{:?}",
        search::binary_search::_binary_search(&[12, 33, 44, 56, 87, 237, 678], 87)
    );

    println!("The grand empire of software...")
}
