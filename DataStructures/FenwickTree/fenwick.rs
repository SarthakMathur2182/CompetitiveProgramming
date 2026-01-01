/// # BIT / Fenwick Tree ([source](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/DataStructures/FenwickTree/fenwick.rs))
///
/// The tree is 1-indexed, but the utils have to be called on basis of 0-based indexing.
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

        pub fn add(&mut self, i: usize, x: T) {
            let mut i = i as isize + 1;
            let len = self.tree.len() as isize;
            while i < len {
                self.tree[i as usize] += x;
                i += i & -i;
            }
        }

        pub fn get(&self, i: usize) -> T {
            let mut ans = T::default();
            let mut i = i as isize + 1;
            while i > 0 {
                ans += self.tree[i as usize];
                i -= i & -i;
            }
            ans
        }
    }
}
use fenwick::*;
