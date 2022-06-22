pub fn _coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let target_value = amount as usize;
    let mut dp = vec![i32::from(i16::MAX); target_value + 1];
    dp[0] = 0;

    for coin in coins {
        let coin = coin as usize;
        for value in coin..=target_value {
            dp[value] = dp[value].min(dp[value - coin] + 1);
        }
    }

    if dp[target_value] == i32::from(i16::MAX) {
        return -1;
    }

    dp[target_value]
}

#[cfg(test)]
mod test_super {
    use std::vec;

    use super::*;

    #[test]
    fn test_coin_change() {
        assert_eq!(_coin_change(vec![1, 2, 5], 11), 3);
        assert_eq!(_coin_change(vec![2], 3), -1);
        assert_eq!(_coin_change(vec![1], 0), 0);
    }
}
