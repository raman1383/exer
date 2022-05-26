pub mod coin_change {
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
}
