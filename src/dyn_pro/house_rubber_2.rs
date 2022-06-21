use std::cmp::max;

pub fn _rob(nums: Vec<i32>) -> i32 {
    max(
        max(nums[0], _helper(nums[1..].to_vec())),
        _helper(nums[..nums.len() - 1].to_vec()),
    )
}

fn _helper(nums: Vec<i32>) -> i32 {
    let mut rob_1: i32 = 0;
    let mut rob_2: i32 = 0;
    for n in nums {
        let new_rob = max(rob_1 + n, rob_2);
        rob_1 = rob_2;
        rob_2 = new_rob;
    }
    rob_2
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_house_rub_2() {
        assert_eq!(_rob(vec![2, 3, 2]), 3);
        assert_eq!(_rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(_rob(vec![1, 2, 3]), 3);
    }
}
