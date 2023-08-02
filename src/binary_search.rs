#![allow(unused)]

// Worst case time complexity: Θ(logn)
// Average case time complexity: Θ(logn)
// Best case time complexity: Θ(1)
// Space complexity: Θ(logn)
fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }
    binary_search_recursive(arr, target, 0, arr.len())
}

fn binary_search_recursive<T: Ord>(
    arr: &[T],
    target: &T,
    start: usize,
    end: usize,
) -> Option<usize> {
    if start >= end {
        return None;
    }
    let mid = (end - start) / 2 + start;
    if arr[mid] == *target {
        return Some(mid);
    } else if arr[mid] > *target {
        return binary_search_recursive(arr, target, start, mid);
    } else {
        return binary_search_recursive(arr, target, mid + 1, end);
    }
}

// Worst case time complexity: Θ(logn)
// Average case time complexity: Θ(logn)
// Best case time complexity: Θ(1)
// Space complexity: Θ(1)
fn binary_search_iterative<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut start = 0;
    let mut end = arr.len();

    while start < end {
        let mid = start + (end - start) / 2;
        if arr[mid] == *target {
            return Some(mid);
        } else if arr[mid] > *target {
            end = mid;
        } else {
            start = mid + 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let arr: [i32; 0] = [];
        assert_eq!(binary_search(&arr, &5), None);
    }

    #[test]
    fn test_single_element_array() {
        let arr = [5];
        assert_eq!(binary_search(&arr, &5), Some(0));
        assert_eq!(binary_search(&arr, &4), None);
    }

    #[test]
    fn test_multiple_elements_array() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&arr, &1), Some(0));
        assert_eq!(binary_search(&arr, &3), Some(2));
        assert_eq!(binary_search(&arr, &5), Some(4));
        assert_eq!(binary_search(&arr, &6), None);
    }
}
