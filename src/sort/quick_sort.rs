// #[allow(dead_code)]

pub fn _quick_sort<T: Ord>(arr: &mut [T]) -> &mut [T] {
    let len = arr.len();
    __quick_sort(arr, 0, (len - 1) as isize)
}

fn __quick_sort<T: Ord>(arr: &mut [T], low: isize, high: isize) -> &mut [T] {
    if low < high {
        let p = _partition(arr, low, high);
        __quick_sort(arr, low, p - 1);
        __quick_sort(arr, p + 1, high);
    }
    arr
}

fn _partition<T: Ord>(arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut i = low - 1;
    let mut j = high;
    loop {
        i += 1;
        while arr[i as usize] < arr[pivot] {
            i += 1;
        }
        j -= 1;
        while j >= 0 && arr[j as usize] > arr[pivot] {
            j -= 1;
        }
        if i >= j {
            break;
        } else {
            arr.swap(i as usize, j as usize)
        }
    }
    arr.swap(i as usize, pivot as usize);
    i
}
