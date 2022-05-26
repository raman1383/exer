// Given an integer array nums, find the contiguous subarray (containing at least one number)
// which has the largest sum and return its sum.

pub mod maximum_subarray {
    pub fn _max_sub_array(nums: Vec<i32>) -> i32 {
        let mut current_max = nums[0];
        let mut max_so_far = nums[0];
        for i in 0..nums.len() {
            current_max = nums[i].max(current_max + nums[i]);
            max_so_far = max_so_far.max(current_max);
        }
        max_so_far
    }
}

// [ -2,1,-3,4,-1,2,1,-5,4]
// out: 6