#![allow(dead_code)]
use std::cmp::max;

// `rod_cut(p)` returns the maximum possible profit if a rod of length `n` = `p.len()`
// is cut into up to `n` pieces, where the profit gained from each piece of length
// `l` is determined by `p[l - 1]` and the total profit is the sum of the profit
// gained from each piece.
//
// # Arguments
//    - `p` - profit for rods of length 1 to n inclusive
//
// # Complexity
//    - time complexity: O(n^2),
//    - space complexity: O(n^2),
//
// where n is the length of `p`.

pub fn rod_cutting(p: &[usize]) -> usize {
    let n = p.len();
    let mut f = vec![0; n];

    for i in 0..n {
        let mut max_price = p[i];
        for j in 1..=i {
            max_price = max(max_price, p[j - 1] + f[i - j]);
        }
        f[i] = max_price;
    }
    if n != 0 {
        f[n - 1]
    } else {
        0
    }
}
