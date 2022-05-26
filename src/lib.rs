mod arrays;
#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test() {
        assert_eq!(
            arrays::maximum_subarray::maximum_subarray::_max_sub_array(vec![
                -2,1,-3,4,-1,2,1,-5,4
            ]),
            6
        )
    }
}
