trait MergeSort<T> {
    fn merge_sort(&mut self)
    where
        T: Ord;
}

impl<T> MergeSort<T> for Vec<T>
where
    T: Ord + Clone,
{
    // Worst case time complexity: Θ(nlogn)
    // Average case time complexity: Θ(nlogn)
    // Best case time complexity: Θ(nlogn)
    // Space complexity: Θ(nlogn)
    fn merge_sort(&mut self) {
        if self.len() <= 1 {
            return;
        }

        let mid = self.len() / 2;
        let mut left = self[..mid].to_vec();
        let mut right = self[mid..].to_vec();

        left.merge_sort();
        right.merge_sort();

        self.clear();
        merge(self, &left, &right);
    }
}

fn merge<T>(v: &mut Vec<T>, left: &[T], right: &[T])
where
    T: Ord + Clone,
{
    let mut l = 0;
    let mut r = 0;

    while l < left.len() && r < right.len() {
        if left[l] <= right[r] {
            v.push(left[l].clone());
            l += 1;
        } else {
            v.push(right[r].clone());
            r += 1;
        }
    }

    while l < left.len() {
        v.push(left[l].clone());
        l += 1;
    }

    while r < right.len() {
        v.push(right[r].clone());
        r += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let mut tests = vec![
            (vec![], vec![]),
            (vec![1], vec![1]),
            (vec![1, 2], vec![1, 2]),
            (vec![2, 1], vec![1, 2]),
            (vec![3, 1, -0, 5, -1, 44, 0], vec![-1, -0, 0, 1, 3, 5, 44]),
            (vec![3, 0, 0, 5, 0, 0, 1], vec![0, 0, 0, 0, 1, 3, 5]),
            (vec![1000; 1000], vec![1000; 1000]),
            (vec![5, 5, 5, 5, 5], vec![5, 5, 5, 5, 5]),
            (vec![-3, -1, -2], vec![-3, -2, -1]),
            (vec![5, 4, 3, 2, 1], vec![1, 2, 3, 4, 5]),
        ];

        for (got, want) in tests.iter_mut() {
            got.merge_sort();
            assert_eq!(got, want);
        }
    }
}
