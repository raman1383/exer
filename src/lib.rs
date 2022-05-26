mod arrays;
#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_contains_duplicate() {
        assert_eq!(
            arrays::contains_duplicate::contains_duplicate::_contains_duplicate(&vec![
                22, 34, 55, 55
            ]),
            true
        )
    }
}
