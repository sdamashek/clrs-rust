pub fn insertion_sort<T: PartialOrd>(v: &mut Vec<T>) {
    for j in 1..v.len() {
        let key = v.remove(j);

        let mut i = j;
        while i > 0 && v[i - 1] > key {
            i -= 1;
        }

        v.insert(i, key);
    }
}

pub fn insertion_sort_dec<T: PartialOrd>(v: &mut Vec<T>) {
    for j in 1..v.len() {
        let key = v.remove(j);

        let mut i = j;
        while i > 0 && v[i - 1] < key {
            i -= 1;
        }

        v.insert(i, key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut v1 = vec![5, 2, 10, 3];
        insertion_sort(&mut v1);
        assert_eq!(v1, [2, 3, 5, 10]);
        insertion_sort_dec(&mut v1);
        assert_eq!(v1, [10, 5, 3, 2]);

        let mut v2: Vec<u32> = vec![];
        insertion_sort(&mut v2);
        assert_eq!(v2, []);
        insertion_sort_dec(&mut v2);
        assert_eq!(v2, []);

        let mut v3 = vec![1, 1, 1, 300, 20, 600];
        insertion_sort(&mut v3);
        assert_eq!(v3, [1, 1, 1, 20, 300, 600]);
        insertion_sort_dec(&mut v3);
        assert_eq!(v3, [600, 300, 20, 1, 1, 1]);
    }
}
