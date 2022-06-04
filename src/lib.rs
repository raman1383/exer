mod arrays;
mod sort;
#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn test_quick_sort() {
        assert_eq!(
            sort::quick_sort::quick_sort(&mut [2, 1, 3, 5, 4, 6, 8, 7]),
            [1, 2, 3, 4, 5, 6, 7, 8]
        );
    }
    #[test]
    fn test_valid_anagram() {
        let mut s = "ram".to_string();
        let mut t = "mar".to_string();
        assert_eq!(true, arrays::valid_anagram::_valid_anagram(&mut s, &mut t));
    }

    #[test]
    fn test_two_sum() {
        let nums = vec![12, 10, 10];
        assert_eq!(vec![1, 2], arrays::two_sum::_two_sum(nums, 20))
    }

    // #[test]
    // fn test_group_anagram() {
    //     assert_eq!(
    //         arrays::group_anagram::group_anagram::_group_anagrams(vec![
    //             "eat".to_string(),
    //             "tea".to_string(),
    //             "tan".to_string(),
    //             "ate".to_string(),
    //             "nat".to_string(),
    //             "bat".to_string()
    //         ]),
    //         vec![
    //             vec!["bat".to_string()],
    //             vec!["eat".to_string(), "tea".to_string(), "ate".to_string(),],
    //             vec!["tan".to_string(), "nat".to_string(),],
    //         ]
    //     )
    // }

    #[test]
    fn test_top_freq() {
        assert_eq!(
            arrays::top_k_frequent_elements::_top_k_frequent_elements(vec![1, 1, 1, 2, 2, 3], 2),
            vec![1, 2]
        )
    }

    #[test]
    fn test_product_of_arr_exp_self() {
        assert_eq!(
            arrays::product_of_array_except_self::_product_of_array_except_self(vec![1, 2, 3, 4],),
            vec![24, 12, 8, 6]
        )
    }
}
