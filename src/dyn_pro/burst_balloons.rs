pub fn _max_coins(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut coins = vec![0; nums.len() + 2];
    let mut len = 0_usize;
    // filter out zeros
    for &num in nums.iter() {
        if num != 0 {
            len += 1;
            coins[len] = num;
        }
    }
    coins[0] = 1;
    coins[len + 1] = 1;

    let mut memo = vec![vec![0; len + 1]; len + 1];
    _max_subrange(&coins, 1, len, &mut memo)
}

fn _max_subrange(coins: &Vec<i32>, start: usize, end: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
    if memo[start][end] != 0 {
        return memo[start][end];
    }
    if start == end {
        memo[start][end] = coins[start - 1] * coins[start] * coins[start + 1];
        return memo[start][end];
    }
    let mut max = 0;
    for i in start..end + 1 {
        let left_max = if i > start {
            _max_subrange(coins, start, i - 1, memo)
        } else {
            0
        };
        let right_max = if i < end {
            _max_subrange(coins, i + 1, end, memo)
        } else {
            0
        };
        max = i32::max(
            max,
            left_max + right_max + coins[i] * coins[start - 1] * coins[end + 1],
        );
    }
    memo[start][end] = max;
    return memo[start][end];
}
