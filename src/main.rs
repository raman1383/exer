#![allow(unused)]
fn main() {
    let arr = [1, 2, 3, 4, 3, 2, 1];
    for i in 0..7 {
        for j in 0..arr[i] {
            print!("#")
        }
        print!("\n")
    }

    println!("-------------");

    for mut i in 0..8 {
        if i <= 4 {
            for j in 0..i {
                print!("#")
            }
        } else {
            for j in 0..(8 - i) {
                print!("#")
            }
        }
        print!("\n")
    }
}
