trait QuickSort<T> {
    fn quick_sort(&mut self)
    where
        T: Ord;
}

impl<T> QuickSort<T> for Vec<T>
where
    T: Ord + Clone,
{
    // Worst case time complexity (sorted array): Θ(n^2)
    // Average case time complexity: Θ(nlogn)
    // Best case time complexity: Θ(nlogn)
    // Space complexity: Θ(1)
    fn quick_sort(&mut self) {
        quick_sort(self)
    }
}

fn quick_sort<T: Ord + Clone>(arr: &mut [T]) {
    match arr.len() {
        0 | 1 => return,
        2 => {
            if arr[0] > arr[1] {
                arr.swap(0, 1);
            }
        }
        _ => {}
    }

    let pivot = arr[0].clone();
    let mut left = 0;
    let mut right = arr.len() - 1;

    // Partition
    while left <= right {
        if arr[left] <= pivot {
            left += 1;
        } else if arr[right] > pivot {
            if right == 0 {
                break;
            }
            right -= 1;
        } else {
            arr.swap(left, right);
            left += 1;
            if right == 0 {
                break;
            }
            right -= 1;
        }
    }

    // Move pivot to it's place.
    arr.swap(0, left - 1);

    // Exclude pivot
    quick_sort(&mut arr[..left - 1]);
    quick_sort(&mut arr[left..]);
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
            got.quick_sort();
            assert_eq!(got, want);
        }
    }
}
