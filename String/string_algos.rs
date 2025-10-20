pub mod string_algos {
    pub fn lps<T: PartialEq>(a: &[T]) -> Vec<usize> {
        let n = a.len();
        let mut lps = vec![0; n + 1];
        let mut i = 1;
        let mut j = 0;
        while i < n {
            if a[i] == a[j] {
                i += 1;
                j += 1;
                lps[i] = j;
            } else if j > 0 {
                j = lps[j];
            } else {
                i += 1;
            }
        }
        lps
    }
}
use string_algos::*;