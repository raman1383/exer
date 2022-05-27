mod arrays;
#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_valid_anagram() {
        let mut s = "ram".to_string();
        let mut t = "mar".to_string();
        assert_eq!(
            true,
            arrays::valid_anagram::valid_anagram::_valid_anagram(&mut s, &mut t)
        );
    }
}
