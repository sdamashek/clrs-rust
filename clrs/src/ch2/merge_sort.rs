fn merge<T: Ord + Clone>(A: &mut Vec<T>, p: usize, q: usize, r: usize) {
    let sentinel = A.iter().max();
    let n1 = q - p + 1;
    let n2 = r - q;

    let mut L: Vec<T> = Vec::with_capacity(n1 + 1);
    let mut R: Vec<T> = Vec::with_capacity(n2 + 1);

    for i in 0..n1 {
        L[i] = A[p + i].clone();
    }

    for j in 0..n2 {
        R[j] = A[q + j].clone();
    }
}


