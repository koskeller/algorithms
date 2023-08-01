#[allow(unused)]

pub fn union(s1: Vec<usize>, s2: Vec<usize>) -> Vec<usize> {
    if s1.len() < 1 || s2.len() < 1 {
        todo!();
    }
    let mut i1 = 0;
    let mut i2 = 0;

    let max = std::cmp::max(s1.len(), s2.len());
    let mut result = Vec::with_capacity(max);

    for i in 0..max {
        if s1.len() >= i1 {
            result.push(s2[i2]);
            i2 += 1;
            continue;
        }
        if s2.len() >= i2 {
            result.push(s1[i1]);
            i1 += 1;
            continue;
        }
        if s1[i1] < s2[i2] {
            result.push(s1[i1]);
            i1 += 1;
        } else if s2[i2] < s1[i1] {
            result.push(s2[i2]);
            i2 += 1;
        } else {
            result.push(s2[i2]);
            i2 += 1;
            i1 += 1;
        }
    }
    result
}

#[test]
fn ok() {
    // assert_eq!(union(vec![], vec![]), vec![]);
    assert_eq!(union(vec![1, 2, 3], vec![2, 3, 4]), vec![1, 2, 3]);
    assert_eq!(union(vec![1], vec![2]), vec![1]);
    assert_eq!(union(vec![1, 2], vec![3, 4]), vec![1, 2]);
}
