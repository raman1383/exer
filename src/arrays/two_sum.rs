//Given an array of integers nums and an integer target, return indices of the two numbers 
//such that they add up to target.

//You may assume that each input would have exactly one solution, and you may not use the 
//same element twice.

//You can return the answer in any order.

pub mod two_sum {
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
// [ 1,3,5]