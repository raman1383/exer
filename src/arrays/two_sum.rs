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

    //brute force
    pub fn _two_sum_brute_force(nums: &Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[j] == target - nums[i] {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }

    //two pass hash table
    pub fn _two_sum_two_pass(nums: &Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map = HashMap::<i32, i32>::new();
        for (i, num) in nums.iter().enumerate() {
            num_map.insert(*num, i as i32);
        }
        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&index) = num_map.get(&complement) {
                if index != i as i32 {
                    return vec![i as i32, index];
                }
            }
        }
        vec![]
    }
}
