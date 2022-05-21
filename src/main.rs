use std::io::Read;

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
    println!("{sum}");

    let xx: i8 = 5;
    let yy: Result<i8, &str> = Ok(12);
    let sum = xx + yy.unwrap();
    println!("{sum}");

    let coin_1 = Coin::Quarter(UsState::Alabama);
    println!("{:?}", coin_1);

    let mut arr: [i32; 6] = [23, 4, 312, 67, 243, 876];
    let y = exer::binary_search::binary_search(&arr, 6, &312);
    println!("{:?}", y);
    let x = exer::quick_sort::quick_sort(&mut arr);
    println!("{:?}", x);

    let mut vec = Vec::<i32>::new();
    vec.push(33);
    vec.push(22);
    vec.pop();

    let mut hash_map = std::collections::HashMap::<&str, i32>::new();
    hash_map.insert("Leakers", 122);
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

#[allow(dead_code)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[allow(dead_code)]
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn _read_username_from_file() -> Result<String, std::io::Error> {
    let mut s = String::new();
    std::fs::File::open("hello.txt")?.read_to_string(&mut s)?; // WOW !
    Ok(s)
}
