fn main() {
    let words = String::from("Hello world");
    let fist_word_in_words = first_word(&words);
    println!("{}", fist_word_in_words);

    let s = "hello "; // stacked
    let ss = s;
    println!("{}{}", s, ss);

    let st = String::from("hello "); // heaped
    let _stt = st;
    // println!("{}{}", st,stt);
    let _tup = ("w", 22u8, String::from("_"), false);
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
