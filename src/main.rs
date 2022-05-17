// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

fn main() {
    // println!("guess a number...");
    // let secret_number = rand::thread_rng().gen_range(1..101);
    // print!("Now, Enter youe guess: \n");
    // loop {
    //     let mut guess = String::new();
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Unable to read the input !");
    //     let guess: i32 = match guess.trim().parse() {
    //         Ok(guess) => guess,
    //         Err(_) => continue,
    //     };
    //     match guess.cmp(&secret_number) {
    //         Ordering::Greater => println!("come lower..."),
    //         Ordering::Less => println!("go higher..."),
    //         Ordering::Equal => {
    //             println!("✌️ You Win ✌️");
    //             break;
    //         }
    //     }
    // }

    let words = String::from("Hello world");
    let fist_word_in_words = first_word(&words);
    println!("{}", fist_word_in_words);
    
    let s = "hello "; // stacked
    let ss = s;
    println!("{}{}",s,ss);

    let st = String::from("hello "); // heaped
    let _stt = st;
    // println!("{}{}", st,stt);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
