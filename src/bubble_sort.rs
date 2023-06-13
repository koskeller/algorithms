trait BubbleSort<T> {
    fn bubble_sort(&mut self)
    where
        T: Ord;
}

impl<T> BubbleSort<T> for Vec<T>
where
    T: Ord + Clone,
{
    // Worst case time complexity: Θ(^2)
    // Average case time complexity: Θ(n^2)
    // Best case time complexity: Θ(n)
    // Space complexity: Θ(1)
    fn bubble_sort(&mut self) {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for n in 1..self.len() {
                if self[n] < self[n - 1] {
                    self.swap(n, n - 1);
                    swapped = true;
                }
            }
        }
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
            (vec![1, 1], vec![1, 1]),
            (vec![3, 1, -0, 5, -1, 44, 0], vec![-1, -0, 0, 1, 3, 5, 44]),
            (vec![3, 0, 0, 5, 0, 0, 1], vec![0, 0, 0, 0, 1, 3, 5]),
            (vec![1000; 1000], vec![1000; 1000]),
            (vec![5, 5, 5, 5, 5], vec![5, 5, 5, 5, 5]),
            (vec![-3, -1, -2], vec![-3, -2, -1]),
            (vec![5, 4, 3, 2, 1], vec![1, 2, 3, 4, 5]),
        ];

        for (got, want) in tests.iter_mut() {
            got.bubble_sort();
            assert_eq!(got, want);
        }
    }
}
