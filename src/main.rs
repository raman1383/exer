fn main() {
    let x = 1;
    let x = x + 2;
    let x = x * 2;
    println!("Value of x: {}", x);

    println!("{}", format!("{:.*}", 4, 12.1234));

    let _arr2vec = [23, 3, 44, 8].to_vec();
}
