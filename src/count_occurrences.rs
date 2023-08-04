// We want to count the number of times a given key k (say “Skiena”) occurs in a given sorted array.
// Proposal: Use binary search and when target if found use binary search again to the right until we find
//    right most boundary. And then do binary search to the left until we find left most boundary.
//     ([1,2,3,3,3,3,6,7], 3) -> 4
//     ([3,3,3,3,3,3,3,3], 3) -> 8
//     Worst time complexity: O(logn+logn+logn) -> O(logn)

#![allow(unused)]
pub fn count<T: Ord>(arr: &[T], target: &T) -> usize {
    let index = search(arr, target, 0, arr.len());
    if let Some(index) = index {
        let right = search_right(arr, target, index, arr.len());
        let left = search_left(arr, target, 0, index);
        right - left
    } else {
        0
    }
}

pub fn search_right<T: Ord>(arr: &[T], target: &T, start: usize, end: usize) -> usize {
    if start >= end {
        return end;
    }
    let mid = (start + end) / 2;
    if arr[mid] > *target {
        return search_right(arr, target, start, mid);
    } else {
        return search_right(arr, target, mid + 1, end);
    }
}

pub fn search_left<T: Ord>(arr: &[T], target: &T, start: usize, end: usize) -> usize {
    if start >= end {
        return start;
    }
    let mid = (start + end) / 2;
    if arr[mid] >= *target {
        return search_left(arr, target, start, mid);
    } else {
        return search_left(arr, target, mid + 1, end);
    }
}

fn search<T: Ord>(arr: &[T], target: &T, start: usize, end: usize) -> Option<usize> {
    if start >= end {
        return Some(end);
    }

    let mid = (start + end) / 2;
    if arr[mid] > *target {
        return search(arr, target, start, mid);
    } else {
        return search(arr, target, mid + 1, end);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let arr = [1, 2, 3, 3, 3, 4, 5];
        assert_eq!(count(&arr, &0), 0);
        assert_eq!(count(&arr, &1), 1);
        assert_eq!(count(&arr, &3), 3);
        assert_eq!(count(&arr, &5), 1);

        let arr = [3, 3, 3];
        assert_eq!(count(&arr, &3), 3);
    }
}
