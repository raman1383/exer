pub mod quick_sort {
    pub fn quick_sort<T: Ord>(arr: &mut [T]) -> &mut [T] {
        let len = arr.len();
        _quick_sort(arr, 0, (len - 1) as isize)
    }

    fn partition<T: PartialOrd>(arr: &mut [T], lo: isize, hi: isize) -> isize {
        let pivot = hi as usize;
        let mut i = lo - 1;
        let mut j = hi;
        loop {
            i += 1;
            while arr[i as usize] < arr[pivot as usize] {
                i += 1
            }
            j -= 1;
            while j >= 0 && arr[j as usize] > arr[pivot as usize] {
                j -= 1
            }
            if i >= j {
                break;
            } else {
                arr.swap(i as usize, j as usize);
            }
        }
        arr.swap(i as usize, pivot as usize);
        i
    }

    fn _quick_sort<T: Ord>(arr: &mut [T], lo: isize, hi: isize) -> &mut [T] {
        if lo < hi {
            let pivot_point = partition(arr, lo, hi);
            _quick_sort(arr, lo, pivot_point - 1);
            _quick_sort(arr, pivot_point + 1, hi);
        }
        arr
    }
}
