pub fn _rub(nums: Vec<i32>) -> i32 {
    let (mut rob_1, mut rob_2) = (0, 0);

    for n in nums {
        let temp = std::cmp::max(n + rob_1, rob_2);
        rob_1 = rob_2;
        rob_2 = temp;
    }

    rob_2
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_rob() {
        assert_eq!(_rub(vec![2, 7, 9, 3, 1]), 12);
        assert_eq!(_rub(vec![1, 2, 3, 1]), 4);
    }
}
