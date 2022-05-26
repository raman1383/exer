pub mod two_sum {

    // One-pass Hash Table
    use std::{collections::HashMap, vec};
    pub fn _two_sum_one_pass(nums: &Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_map = HashMap::<i32, i32>::new();
        for (i, num) in nums.iter().enumerate() {
            match hash_map.get(num) {
                Some(&index) => return vec![index, i as i32],
                None => hash_map.insert(target - num, i as i32),
            };
        }
        vec![]
    }
}
