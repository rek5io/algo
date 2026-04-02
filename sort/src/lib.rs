use rand::prelude::*;

pub fn sort_test_n_avg<F>(
    n_times: usize,
    size: usize,
    percent_sorted: f64,
    rev: bool,
    sort_fn: F,
) -> f64
where
    F: Fn(&mut [i32]),
{
    let res: Vec<std::time::Duration> = (0..n_times)
        .map(|_| sort_test(size, percent_sorted, rev, &sort_fn))
        .collect();
    res.iter().map(|d| d.as_nanos() as f64).sum::<f64>() / (res.len() as f64 * 1000.0)
}

pub fn sort_test(
    size: usize,
    percent_sorted: f64,
    rev_sorted: bool,
    sort_fn: impl Fn(&mut [i32]),
) -> std::time::Duration {
    let mut rng = rand::rng();
    let mut set: Vec<i32> = (0..size).map(|_| rng.random()).collect();
    let mut set_org = set.clone();

    if percent_sorted > 0.0 {
        let len = set.len();
        (&mut set[..(len as f64 * percent_sorted) as usize]).sort();
    }

    if rev_sorted {
        set.sort();
        set.reverse();
    }

    let t_start = std::time::Instant::now();
    sort_fn(&mut set);
    let t_end = std::time::Instant::now();

    set_org.sort();
    assert!(set_org == set);

    t_end - t_start
}

mod util {
    #[inline(always)]
    pub fn choose_pivot<T: PartialOrd>(arr: &mut [T]) -> usize {
        let len = arr.len();
        if len < 100 {
            median_of_three(arr, 0, len / 2, len - 1)
        } else {
            tukeys_ninther(arr)
        }
    }

    #[inline(always)]
    fn median_of_three<T: PartialOrd>(arr: &mut [T], a: usize, b: usize, c: usize) -> usize {
        if arr[a] > arr[b] {
            arr.swap(a, b);
        }

        if arr[a] > arr[c] {
            arr.swap(a, c);
        }

        if arr[b] > arr[c] {
            arr.swap(b, c);
        }

        b
    }

    #[inline(always)]
    fn tukeys_ninther<T: PartialOrd>(arr: &mut [T]) -> usize {
        let len = arr.len();
        let eps = len / 8;
        let m1 = median_of_three(arr, 0, eps, 2 * eps);
        let m2 = median_of_three(arr, len / 2 - eps, len / 2, len / 2 + eps);
        let m3 = median_of_three(arr, len - 1 - 2 * eps, len - 1 - eps, len - 1);
        median_of_three(arr, m1, m2, m3)
    }

    #[inline(always)]
    pub fn partition<T: PartialOrd>(arr: &mut [T]) -> usize {
        let len = arr.len();
        let pivot_idx = len - 1;
        let mut i = 0;

        for j in 0..pivot_idx {
            if arr[j] < arr[pivot_idx] {
                arr.swap(i, j);
                i += 1;
            }
        }

        arr.swap(i, pivot_idx);
        i
    }
}

pub mod merge {
    #[inline(always)]
    pub fn merge_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
        if arr.len() < 2 {
            return;
        }

        let mut buffer = vec![arr[0]; arr.len()];
        merge_sort_inner(arr, &mut buffer);
    }

    fn merge_sort_inner<T: PartialOrd + Copy>(arr: &mut [T], buffer: &mut [T]) {
        let len = arr.len();
        if len < 2 {
            return;
        }

        let mid = len / 2;
        let (arr_left, arr_right) = arr.split_at_mut(mid);
        let (buf_left, buf_right) = buffer.split_at_mut(mid);

        merge_sort_inner(arr_left, buf_left);
        merge_sort_inner(arr_right, buf_right);

        merge(arr_left, arr_right, buffer);

        arr.copy_from_slice(&buffer[..len]);
    }

    #[inline(always)]
    fn merge<T: PartialOrd + Copy>(left: &[T], right: &[T], buffer: &mut [T]) {
        let mut l = 0;
        let mut r = 0;
        let mut offset = 0;

        while l < left.len() && r < right.len() {
            if left[l] <= right[r] {
                buffer[offset] = left[l];
                l += 1;
            } else {
                buffer[offset] = right[r];
                r += 1;
            }
            offset += 1;
        }

        if l < left.len() {
            buffer[offset..offset + left.len() - l].copy_from_slice(&left[l..]);
        }

        if r < right.len() {
            buffer[offset..offset + right.len() - r].copy_from_slice(&right[r..]);
        }
    }
}

pub mod quick {
    use super::*;

    #[inline(always)]
    pub fn quick_sort<T: PartialOrd>(arr: &mut [T]) {
        if arr.len() < 2 {
            return;
        }

        quick_sort_inner(arr);
    }

    fn quick_sort_inner<T: PartialOrd>(arr: &mut [T]) {
        let len = arr.len();
        if len < 2 {
            return;
        }

        let pivot = util::choose_pivot(arr);
        arr.swap(pivot, len - 1);

        let pivot = util::partition(arr);
        quick_sort_inner(&mut arr[..pivot]);
        quick_sort_inner(&mut arr[pivot + 1..]);
    }
}

pub mod intro {
    use super::*;

    #[inline(always)]
    pub fn intro_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
        let max_depth = ((arr.len() as f64).log2() as usize) * 2;
        intro_sort_inner(arr, max_depth);
    }

    fn intro_sort_inner<T: PartialOrd + Copy>(arr: &mut [T], max_depth: usize) {
        let len = arr.len();
        if len < 2 {
            return;
        }

        if len < 16 {
            insertion_sort(arr);
        } else if max_depth == 0 {
            heap_sort(arr);
        } else {
            let pivot = util::choose_pivot(arr);
            arr.swap(pivot, len - 1);

            let pivot = util::partition(arr);
            intro_sort_inner(&mut arr[..pivot], max_depth - 1);
            intro_sort_inner(&mut arr[pivot + 1..], max_depth - 1);
        }
    }

    #[inline(always)]
    fn insertion_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
        let len = arr.len();

        for i in 1..len {
            let key = arr[i];
            let mut j = i;

            while j > 0 && arr[j - 1] > key {
                arr[j] = arr[j - 1];
                j -= 1;
            }

            arr[j] = key;
        }
    }

    #[inline(always)]
    fn heap_sort<T: PartialOrd>(arr: &mut [T]) {
        let len = arr.len();

        for i in (0..len / 2).rev() {
            sift_down(arr, i, len);
        }

        for end in (1..len).rev() {
            arr.swap(0, end);
            sift_down(arr, 0, end);
        }
    }

    #[inline(always)]
    fn sift_down<T: PartialOrd>(arr: &mut [T], mut root: usize, heap_size: usize) {
        loop {
            let left = 2 * root + 1;
            let right = 2 * root + 2;
            let mut largest = root;

            if left < heap_size && arr[left] > arr[largest] {
                largest = left;
            }

            if right < heap_size && arr[right] > arr[largest] {
                largest = right;
            }

            if largest == root {
                break;
            }

            arr.swap(root, largest);
            root = largest;
        }
    }
}
