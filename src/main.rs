mod arrays;

fn main() {
    let vec = vec![12, 44, 45, 27, 843, 74, 2];
    println!(
        "{:?}",
        arrays::two_sum::two_sum::_two_sum_two_pass(&vec, 76)
    );
    println!(
        "{:?}",
        arrays::two_sum::two_sum::_two_sum_brute_force(&vec, 46)
    );
    println!(
        "{:?}",
        arrays::two_sum::two_sum::_two_sum_one_pass(&vec, 56)
    );
}
