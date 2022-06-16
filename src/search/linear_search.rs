pub fn linear_search<T: PartialEq>(arr: &[T], item: &T) -> Option<usize> {
    for (i, data) in arr.iter().enumerate() {
        if item == data {
            return Some(i);
        }
    }

    None
}
