#![allow(unused)]

fn main() {
    for i in (0..6).rev() {
        for j in 0..i {
            print!("#")
        }
        print!("\n")
    }
}
