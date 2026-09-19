/// # BIT / Fenwick Tree ([source](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/DataStructures/FenwickTree/fenwick.rs))
///
/// The tree is 1-indexed, but the utils have to be called on basis of 0-based indexing.
///
/// You'll need the module [my_utils.rs](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/CombinedRustModules/my_utils.rs) to use the same.
pub mod fenwick {
    use std::ops::{AddAssign, RangeBounds, SubAssign};

    pub struct FenwickTree<T: AddAssign + SubAssign + Default + Copy + Clone> {
        tree: Vec<T>,
    }

    impl<T: AddAssign + SubAssign + Default + Copy + Clone> FenwickTree<T> {
        pub fn new(n: usize) -> Self {
            Self {
                tree: vec![T::default(); n + 1],
            }
        }

        pub fn add(&mut self, mut i: usize, x: T) {
            i += 1;
            while i < self.tree.len() {
                self.tree[i] += x;
                i += i & (!i + 1);
            }
        }

        pub fn sum<R: RangeBounds<usize>>(&self, range: R) -> T {
            let (l, r) = super::get_inclusive_usize_bounds(&range, self.tree.len() - 1);
            let mut ans = self._sum(r);
            if l > 0 {
                ans -= self._sum(l - 1);
            }
            ans
        }

        fn _sum(&self, mut i: usize) -> T {
            let mut ans = T::default();
            i += 1;
            while i > 0 {
                ans += self.tree[i];
                i -= i & (!i + 1);
            }
            ans
        }

        pub fn update(&mut self, i: usize, mut x: T) {
            x -= self.sum(i..=i);
            self.add(i, x);
        }
    }
}
use fenwick::*;
