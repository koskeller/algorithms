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
    // Space complexity: Θ(n)
    fn merge_sort(&mut self) {
        let len = self.len();
        if len <= 1 {
            return;
        }
        let mut temp_arr: Vec<T> = Vec::with_capacity(len);
        merge(self, &mut temp_arr, 0, len);
    }
}

fn merge<T: Ord + Clone>(arr: &mut [T], temp_vec: &mut Vec<T>, from: usize, to: usize) {
    if to - from <= 1 {
        return;
    }

    let mid = (from + to) / 2;
    merge(arr, temp_vec, from, mid);
    merge(arr, temp_vec, mid, to);

    let mut i = from;
    let mut j = mid;
    while i < mid && j < to {
        if arr[i] <= arr[j] {
            temp_vec.push(arr[i].clone());
            i += 1;
        } else {
            temp_vec.push(arr[j].clone());
            j += 1;
        }
    }

    while i < mid {
        temp_vec.push(arr[i].clone());
        i += 1;
    }

    while j < to {
        temp_vec.push(arr[j].clone());
        j += 1;
    }

    for i in 0..temp_vec.len() {
        std::mem::swap(&mut arr[from + i], &mut temp_vec[i]);
    }

    temp_vec.clear();
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
