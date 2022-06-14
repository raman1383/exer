mod arrays;
mod dyn_pro;
mod sort;
mod search;
mod tree;
mod graph;
#[cfg(test)]
mod test {
    use crate::*;
    use test;
    #[test]
    fn test_fib() {
        assert_eq!(
            2971215073,
            dyn_pro::fibonacci::_dynamic_programming_recursive_fibonacci(46)
        )
    }
    #[test]
    fn test_quick_sort() {
        assert_eq!(
            sort::quick_sort::_quick_sort(&mut [2, 1, 3, 5, 4, 6, 8, 7]),
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

    #[test]
    fn test_valid_sudoku() {
        assert_eq!(
            arrays::valid_sudoku::_valid_sudoku(vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ]),
            false
        );
        assert_eq!(
            arrays::valid_sudoku::_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            true
        );
    }
}
