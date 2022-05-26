//Given an integer array nums, return true if any value appears at 
//least twice in the array, and return false if every element is distinct.


pub mod contains_duplicate {
    use std::collections::HashMap;

    pub fn _contains_duplicate(vec: &Vec<usize>) -> bool {
        let mut map = HashMap::<usize, usize>::new();
        for (i, num) in vec.iter().enumerate() {
            match map.contains_key(num) {
                true => return true,
                false => map.insert(*num, i),
            };
        }
        false
    }
}
