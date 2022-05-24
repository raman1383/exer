pub mod quick_sort {
    pub fn quick_sort<T: Ord>(arr: &mut [T]) -> &mut [T] {
        let len = arr.len();
        _quick_sort(arr, 0, (len - 1) as isize)
    }
    fn partition<T: PartialOrd>(arr: &mut [T], lo: isize, hi: isize) -> isize {
        let pivot = hi as usize;
        let mut i = lo - 1;
        let mut j = hi;
        loop {
            i += 1;
            while arr[i as usize] < arr[pivot as usize] {
                i += 1
            }
            j -= 1;
            while j >= 0 && arr[j as usize] > arr[pivot as usize] {
                j -= 1
            }
            if i >= j {
                break;
            } else {
                arr.swap(i as usize, j as usize);
            }
        }
        arr.swap(i as usize, pivot as usize);
        i
    }

    fn _quick_sort<T: Ord>(arr: &mut [T], lo: isize, hi: isize) -> &mut [T] {
        if lo < hi {
            let pivot_point = partition(arr, lo, hi);
            _quick_sort(arr, lo, pivot_point - 1);
            _quick_sort(arr, pivot_point + 1, hi);
        }
        arr
    }
}

// Binary search
pub mod binary_search {
    pub fn binary_search(arr: &[i32], length: usize, target: &i32) -> Option<usize> {
        let mut low = 0;
        let mut high = length - 1;
        while low < high {
            let mid = ((high + low) / 2) + low;
            let mid_index = mid;
            let value = &arr[mid_index];

            if value == target {
                return Some(mid_index);
            }

            if value < target {
                low = mid + 1;
            }

            if value > target {
                high = mid - 1;
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn q_sort() {
        let mut arr = [2, 3, 5, 4, 1];
        assert_eq!(quick_sort::quick_sort(&mut arr), [1, 2, 3, 4, 5])
    }

    #[test]
    fn bin_search() {
        assert_eq!(
            Some(3),
            binary_search::binary_search(&mut [12, 3, 2, 46, 7, 8], 7, &46)
        );
    }
}

// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;
// fn main() {
//     println!("guess a number...");
//     let secret_number = rand::thread_rng().gen_range(1..101);
//     print!("Now, Enter you guess: \n");
//     loop {
//         let mut guess = String::new();
//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Unable to read the input !");
//         let guess: i32 = match guess.trim().parse() {
//             Ok(guess) => guess,
//             Err(_) => continue,
//         };
//         match guess.cmp(&secret_number) {
//             Ordering::Greater => println!("come lower..."),
//             Ordering::Less => println!("go higher..."),
//             Ordering::Equal => {
//                 println!("😍😍😍😍😍😍😍😍");
//                 println!("    You Win      ");
//                 println!("😍😍😍😍😍😍😍😍");
//                 break;
//             }
//         }
//     }
// }

// Fibonacci with dynamic programming
pub mod fibonacci {
    use std::collections::HashMap;

    pub fn memoized_fibonacci(n: u32) -> u128 {
        let mut cache: HashMap<u32, u128> = HashMap::new();
        _memoized_fibonacci(n, &mut cache)
    }

    fn _memoized_fibonacci(n: u32, cache: &mut HashMap<u32, u128>) -> u128 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        let f = match cache.get(&n) {
            Some(f) => f,
            None => {
                let f1 = _memoized_fibonacci(n - 1, cache);
                let f2 = _memoized_fibonacci(n - 2, cache);
                cache.insert(n, f1 + f2);
                cache.get(&n).unwrap()
            }
        };
        *f
    }
}
