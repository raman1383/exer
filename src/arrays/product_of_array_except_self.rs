pub fn _product_of_array_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut top_right = 1;
    let mut bottom_left = 1;

    let mut ret = vec![1; nums.len()];
    for i in (0..nums.len()).rev() {
        ret[i] = ret[i] * top_right;
        top_right = top_right * nums[i];
    }
    for i in 0..nums.len() {
        ret[i] = ret[i] * bottom_left;
        bottom_left = bottom_left * nums[i];
    }
    ret
}
