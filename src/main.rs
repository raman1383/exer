use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("guess a number...");
    let secret_number = rand::thread_rng().gen_range(1..101);
    print!("Now, Enter youe guess: \n");
    loop {
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Unable to read the input !");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("come lower..."),
            Ordering::Less => println!("go higher..."),
            Ordering::Equal => {
                println!("😍😍😍😍😍😍😍😍");
                println!("    You Win      ");
                println!("😍😍😍😍😍😍😍😍");
                break;
            }
        }
    }
}
