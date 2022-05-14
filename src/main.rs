use std::io;

fn main() {
    println!("guess a number...");
    print!("Now, Enter youe guess: ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Unable to read the input !");
    println!("your guess is ->  {guess}");
}
