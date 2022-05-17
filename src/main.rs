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
    println!("{:#?}", person_1);
    // }

    let person_2 = Person {
        name: "mia".to_string(),
        ..person_1
    };

    println!("{:#?}", person_2);

    let person_3 = Person::new("alexis".to_string(), 33, true);
    println!("{:#?}", &person_3);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y.unwrap();
    println!("{sum}")
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
#[allow(dead_code)]
struct Person {
    name: String,
    age: u8,
    alive: bool,
}
impl Person {
    fn new(name: String, age: u8, alive: bool) -> Person {
        Person { name, age, alive }
    }
}
