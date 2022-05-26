mod arrays;
#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_contains_duplicate() {
        assert_eq!(
            arrays::not_contains_duplicate::not_contains_duplicate::_not_contains_duplicate(&vec![
                22, 34, 55, 55
            ]),
            true
        )
    }
}
