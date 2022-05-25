use exer::coin_change;
use exer::memoized_fibonacci;

fn main() {
    println!("{}", memoized_fibonacci(6));
    println!("{}", coin_change(&[], 2).unwrap());
}
