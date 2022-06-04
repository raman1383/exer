//Given an integer array nums, return true if any value appears at
//least twice in the array, and return false if every element is distinct.

pub fn _contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hm = std::collections::HashMap::<i32, i32>::new();
    for i in nums {
        match hm.contains_key(&i) {
            true => return true,
            false => hm.insert(i, i),
        };
    }
    false
}
