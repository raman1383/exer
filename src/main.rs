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

    let person1 = NameLength::new("mia khalifa");
    println!("{}  < - | - >  {}", person1.length, person1.name);
    person1.print();
}

struct NameLength {
    name: String,
    length: usize,
}

impl NameLength {
    fn new(name: &str) -> Self {
        NameLength {
            name: name.to_string(),
            length: name.len(),
        }
    }
    fn print(&self) {
        println!("{} is {} characters long !!!", self.name, self.length)
    }
}
