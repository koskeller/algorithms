trait SelectionSort<T> {
    fn selection_sort(&mut self)
    where
        T: Ord + std::fmt::Debug;
}

impl<T> SelectionSort<T> for Vec<T>
where
    T: Ord + std::fmt::Debug,
{
    // Worst case time complexity: Θ(n^2)
    // Average case time complexity: Θ(n^2)
    // Best case time complexity: Θ(n^2)
    // Space complexity: Θ(1)
    fn selection_sort(&mut self) {
        if self.len() <= 1 {
            return;
        }

        for i in 0..self.len() - 1 {
            let mut smallest = i;
            for j in i + 1..self.len() {
                if self[j] < self[smallest] {
                    smallest = j;
                }
            }
            self.swap(i, smallest);
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
            (vec![3, 1, -0, 5, -1, 44, 0], vec![-1, -0, 0, 1, 3, 5, 44]),
            (vec![3, 0, 0, 5, 0, 0, 1], vec![0, 0, 0, 0, 1, 3, 5]),
        ];

        for (got, want) in tests.iter_mut() {
            got.selection_sort();
            assert_eq!(got, want);
        }
    }
}
