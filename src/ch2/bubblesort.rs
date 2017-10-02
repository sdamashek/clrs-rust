pub fn bubblesort<T: PartialOrd>(A: &mut Vec<T>) {
    for i in 0..A.len() {
        for j in ((i+1)..A.len()).rev() {
            if A[j] < A[j - 1] {
                A.swap(j, j - 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut v1 = vec![5, 2, 10, 3];
        bubblesort(&mut v1);
        assert_eq!(v1, [2, 3, 5, 10]);

        let mut v2 = vec![5, 1, 6, 3, 7, 4, 55, 9, 0];
        bubblesort(&mut v2);
        assert_eq!(v2, [0, 1, 3, 4, 5, 6, 7, 9, 55]);

        let mut v3 = vec![1, 1, 1, 300, 20, 600];
        bubblesort(&mut v3);
        assert_eq!(v3, [1, 1, 1, 20, 300, 600]);
    }
}
