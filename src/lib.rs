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

pub fn coin_change(coins: &[usize], amount: usize) -> Option<usize> {
    let mut dp = vec![std::usize::MAX; amount + 1];
    dp[0] = 0;

    // Assume dp[i] is the fewest number of coins making up amount i,
    // then for every coin in coins, dp[i] = min(dp[i - coin] + 1).
    for i in 0..=amount {
        for j in 0..coins.len() {
            if i >= coins[j] && dp[i - coins[j]] != std::usize::MAX {
                dp[i] = dp[i].min(dp[i - coins[j]] + 1);
            }
        }
    }
    match dp[amount] {
        std::usize::MAX => None,
        _ => Some(dp[amount]),
    }
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash_map = HashMap::<i32, i32>::new();
    for (i, num) in nums.iter().enumerate() {
        match hash_map.get(num) {
            Some(&index) => return vec![index, i as i32],
            None => hash_map.insert(target - num, i as i32),
        };
    }
    vec![]
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn bin_search() {
        assert_eq!(Some(3), binary_search(&mut [12, 3, 2, 46, 7, 8], 7, &46));
    }

    #[test]
    fn q_sort() {
        let mut arr = [2, 3, 5, 4, 1];
        assert_eq!(quick_sort(&mut arr), [1, 2, 3, 4, 5])
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(memoized_fibonacci(10), 55);
        assert_eq!(memoized_fibonacci(20), 6765);
    }

    #[test]
    fn test_coin_change() {
        let coins = vec![1, 2, 5];
        assert_eq!(Some(3), coin_change(&coins, 11));

        // 119 = 11 * 10 + 7 * 1 + 2 * 1
        let coins = vec![2, 3, 5, 7, 11];
        assert_eq!(Some(12), coin_change(&coins, 119));
    }

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![12, 12, 444, 3, 1], 24), vec![0, 1]);
    }
}
