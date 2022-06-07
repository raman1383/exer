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
        for j in 0..(4 - (4_i32 - i).abs()) {
            print!("#")
        }

        print!("\n")
    }
}
