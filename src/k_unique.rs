#![allow(unused)]

use std::{collections::HashSet, hash::Hash};

// Time complexity: O(n)
// Space complexity: O(k)
pub fn is_k_unique<T: Hash + Eq>(k: usize, arr: Vec<T>) -> bool {
    if k == 0 || arr.len() == 0 {
        return true;
    }
    let mut map = HashSet::with_capacity(k);
    for (i, v) in arr.iter().enumerate() {
        if i > k {
            map.remove(&arr[i - k - 1]);
        }
        if map.contains(&v) {
            return false;
        }
        map.insert(v);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iw_works() {
        assert!(is_k_unique(1, vec![0]));
        assert!(is_k_unique(1, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]));
        assert!(is_k_unique(1, vec![1, 2, 1, 2]));
        assert!(!is_k_unique(2, vec![1, 2, 1, 2]));
        assert!(!is_k_unique(4, vec![1, 2, 3, 4, 1, 2, 3, 4]));
        assert!(!is_k_unique(2, vec![2, 1, 2]));
    }
}
