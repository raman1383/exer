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
