fn main() {
    println!("start");
    let mut x = 0;
    loop {
        x += 1;
        println!("hello world");
        if x == 10 {
            println!("End");
            break;
        }
    }
}
