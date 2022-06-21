mod arrays;
mod dyn_pro;
pub mod search;
mod sort;
fn main() {
    let (cpp, rust) = ("cpp sucks", "rust is AWESOME");

    for i in 0..100 {
        println!(" {i} -> {cpp}, {rust}")
    }
}
