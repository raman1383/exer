use std::cmp::max;
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
