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

pub fn quick_sort<T: Ord>(arr: &mut [T]) -> &mut [T] {
    let len = arr.len();
    _quick_sort(arr, 0, (len - 1) as isize)
}

// Binary search

pub fn binary_search(arr: &[i32], length: usize, target: &i32) -> Option<usize> {
    let mut low = 0;
    let mut high = length - 1;
    while low < high {
        let mid = ((high + low) / 2) + low;
        let mid_index = mid;
        let value = &arr[mid_index];

        if value == target {
            return Some(mid_index);
        }

        if value < target {
            low = mid + 1;
        }

        if value > target {
            high = mid - 1;
        }
    }
    None
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn q_sort() {
        let mut arr = [2, 3, 5, 4, 1];
        assert_eq!(quick_sort(&mut arr), [1, 2, 3, 4, 5])
    }

    #[test]
    fn bin_search() {
        assert_eq!(Some(3), binary_search(&mut [12, 3, 2, 46, 7, 8], 7, &46));
    }
}
