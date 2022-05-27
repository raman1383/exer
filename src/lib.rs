mod arrays;
#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test() {
        assert_eq!(
            true,
            arrays::contains_duplicate::contains_duplicate::_contains_duplicate(vec![12, 22, 22])
        );
    }
}
