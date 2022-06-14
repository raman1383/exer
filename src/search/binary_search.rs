pub fn _binary_search(array: &[i32], target: i32) -> i32 {
    let mut high = array.len() as i32 - 1;
    let mut mid: i32;
    let mut low = 0;

    while high >= low {
        mid = ((high - low) / 2) + low;
        let val = array[mid as usize];
        if val == target {
            return array.iter().position(|&value| value == val).unwrap() as i32;
        } else if val > target {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    return -1;
}
