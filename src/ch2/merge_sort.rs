fn merge<T: Ord + Clone>(A: &mut Vec<T>, p: usize, q: usize, r: usize, sentinel: &T) {
    // p, r inclusive
    let n1 = q - p + 1;
    let n2 = r - q;

    let mut L: Vec<T> = Vec::with_capacity(n1 + 1);
    let mut R: Vec<T> = Vec::with_capacity(n2 + 1);

    for i in 0..n1 {
        L.push(A[p + i].clone());
    }

    for j in 0..n2 {
        R.push(A[q + j + 1].clone());
    }

    L.push(sentinel.clone());
    R.push(sentinel.clone());

    let mut i = 0;
    let mut j = 0;
    for k in p..(r + 1) {
        if L[i] <= R[j] {
            A[k] = L[i].clone();
            i += 1;
        }
        else {
            A[k] = R[j].clone();
            j += 1;
        }
    }
}

fn merge_sort_sentinel<T: Ord + Clone>(A: &mut Vec<T>, p: usize, r: usize, sentinel: &T) {
    // p, r inclusive
    if p < r {
        let q = p + (r - p) / 2;
        merge_sort_sentinel(A, p, q, sentinel);
        merge_sort_sentinel(A, q + 1, r, sentinel);
        merge(A, p, q, r, sentinel);
    }
}

fn get_sentinel<T: Ord + Clone>(A: &Vec<T>) -> Option<T> {
    let sentinel = A.iter().max();
    if let Some(max) = sentinel {
        return Some(max.clone());
    } else {
        return None;
    }
}

pub fn merge_sort<T: Ord + Clone>(A: &mut Vec<T>, p: usize, r: usize) {
    let sentinel = get_sentinel(A);

    if let Some(max) = sentinel {
        merge_sort_sentinel(A, p, r, &max);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_merge_sort() {
        let mut v1 = vec![5, 2, 10, 3];
        merge_sort(&mut v1, 0, 3);
        assert_eq!(v1, [2, 3, 5, 10]);

        let mut v2 = vec![5, 1, 6, 3, 7, 4, 55, 9, 0];
        merge_sort(&mut v2, 0, 8);
        assert_eq!(v2, [0, 1, 3, 4, 5, 6, 7, 9, 55]);

        let mut v3 = vec![1, 1, 1, 300, 20, 600];
        merge_sort(&mut v3, 0, 5);
        assert_eq!(v3, [1, 1, 1, 20, 300, 600]);

        let mut rng = thread_rng();
        let mut ints: Vec<i32> = (0..(1_000 + 1)).collect();
        rng.shuffle(&mut ints);
        let mut ints2: Vec<i32> = ints.clone();

        merge_sort(&mut ints, 0, 1_000);
        ints2.sort();
        assert_eq!(ints, ints2);

    }
}
