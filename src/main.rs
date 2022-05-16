use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess a number...");
    let secret_number = rand::thread_rng().gen_range(1..101);
    print!("Now, Enter youe guess: \n");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to read the input !");
        let guess: i32 = match guess.trim().parse() {
            Ok(guess) => guess,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("come lower..."),
            Ordering::Less => println!("go higher..."),
            Ordering::Equal => {
                println!("✌️ You Win ✌️");
                break;
            }
        }
    }

    let tuple = (12, 4, 54, 77);
    let array = [12, 33, 44, 5, 67];
    let vec = vec![12, 3, 44, 55, 77, 23];

    let tuple_2 = tuple;
    println!("tuple -> {:?}, tuple_2 -> {:?}", tuple, tuple_2);

    let array_2 = array;
    println!("array_2 -> {:?}, array -> {:?}", array_2, array);

    let vec_2 = vec;r
    println!("vec_2 -> {:?}, vec -> {:?}", vec_2, vec);
}
