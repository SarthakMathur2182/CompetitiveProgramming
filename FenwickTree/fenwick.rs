pub mod fenwick {
    use std::ops::AddAssign;

    pub struct FenwickTree<T: AddAssign + Default + Copy + Clone> {
        tree: Vec<T>,
    }

    impl<T: AddAssign + Default + Copy + Clone> FenwickTree<T> {
        pub fn new(n: usize) -> Self {
            Self {
                tree: vec![T::default(); n + 1],
            }
        }

        pub fn add(&mut self, mut i: usize, x: T) {
            i += 1;
            while i < self.tree.len() {
                self.tree[i] += x;
                i += (i as isize & -(i as isize)) as usize;
            }
        }

        pub fn get(&self, mut i: usize) -> T {
            let mut ans = T::default();
            i += 1;
            while i > 0 {
                ans += self.tree[i];
                i -= (i as isize & -(i as isize)) as usize;
            }
            ans
        }
    }
}
use fenwick::*;
