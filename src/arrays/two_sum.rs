//Given an array of integers nums and an integer target, return indices of the two numbers
//such that they add up to target.

//You may assume that each input would have exactly one solution, and you may not use the
//same element twice.

//You can return the answer in any order.

pub mod two_sum {
    pub fn _two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm = std::collections::HashMap::<i32, i32>::new();
        for (i, num) in nums.iter().enumerate() {
            match hm.get(&num) {
                Some(&index) => return vec![index, i as i32],
                None => hm.insert(target - num, i as i32),
            };
        }

        vec![]
    }
}
// [ 0:1, 1:3, 2:5] t:8

// hash_map[7:0, 5:1, 3:2]
