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

    // {
    let person_1 = Person {
        name: String::from("lana"),
        age: 27,
        alive: true,
    };
    println!("{}--{}--{}", person_1.name, person_1.age, person_1.alive);
    // }

    let person_2 = Person {
        name: "mia".to_string(),
        ..person_1
    };

    println!("{}--{}--{}", person_2.name, person_2.age, person_2.alive);
    println!("{:#?}", person_2)
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

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    alive: bool,
}
