// We have an array A consisting of a run of 0’s, followed by an unbounded run of 1’s,
// and would like to identify the exact point of transition between them.
// Solution: Iterate over array using increasing intervals (1, 2, 4, 8) until we find transition.
// Then use binary search to find point of transition between start and end of interval.
// Average time complexity: O(lognp)

#![allow(unused)]
pub fn search(arr: &[usize]) -> Option<usize> {
    let mut interval = 1;
    let mut start = 0;
    let mut end = start + interval;
    while start < arr.len() {
        if let Some(value) = arr.get(end) {
            if value == &1 {
                break;
            } else {
                interval = interval * 2;
                start = end;
                end = start + interval;
            }
        } else {
            end = arr.len();
            break;
        }
    }

    while start < end {
        let mid = (start + end) / 2;
        if arr[mid] == 1 {
            return Some(mid);
        } else if arr[mid] > 1 {
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
    fn test_ok() {
        let arr = [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1];
        assert_eq!(search(&arr), Some(9));

        let arr = [0, 0];
        assert_eq!(search(&arr), None);

        let arr = [1];
        assert_eq!(search(&arr), Some(0));

        let arr = [0];
        assert_eq!(search(&arr), None);
    }
}
