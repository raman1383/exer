pub mod binary_search {
    pub fn binary_search(arr: &[i32], length: usize, target: &i32) -> Option<usize> {
        let mut low = 0;
        let mut high = length - 1;
        while low < high {
            let mid = ((high + low) / 2) + low;
            let mid_index = mid;
            let value = &arr[mid_index];

            if value == target {
                return Some(mid_index);
            }

            if value < target {
                low = mid + 1;
            }

            if value > target {
                high = mid - 1;
            }
        }
        None
    }
}
