pub mod product_of_array_except_self {
    pub fn _product_of_array_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut top_right = 1;
        let mut bottom_left = 1;

        let n = nums.len();
        let mut ret = vec![1; n];
        for i in (0..n).rev() {
            ret[i] *= top_right;
            top_right *= nums[i];
        }
        for i in 0..n {
            ret[i] *= bottom_left;
            bottom_left *= nums[i];
        }
        ret
    }
}
