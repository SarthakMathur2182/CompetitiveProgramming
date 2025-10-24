/// # Sparse Table and RMQ ([source](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/SparseTable/sparse.rs))
///
/// You'll need the module [my_utils.rs](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/CombinedRustModules/my_utils.rs) to use the same.
pub mod sparse {
    use std::convert::{TryFrom, TryInto};
    use std::ops::{Bound, RangeBounds};

    pub struct SparseTable<T: Copy + Clone, F: Fn(&T, &T) -> T> {
        n: usize,
        func: F,
        sparse_table: Vec<Vec<T>>,
    }

    impl<T: Copy + Clone, F: Fn(&T, &T) -> T> SparseTable<T, F> {
        pub fn new(a: &[T], func: F) -> Self {
            let n = a.len();
            let log = n.ilog2() as usize;
            let mut table = vec![vec![]; log + 1];
            table[0] = a.to_vec();
            for b in 1..=log {
                table[b].reserve_exact(n - (1 << b) + 1);
                for i in 0..table[b].capacity() {
                    // Replace it with push_within_capacity() once its stable
                    let val = func(&table[b - 1][i], &table[b - 1][i + (1 << (b - 1))]);
                    table[b].push(val);
                }
            }

            Self {
                n,
                func,
                sparse_table: table,
            }
        }

        pub fn query<R: RangeBounds<usize>>(&self, range: R) -> T {
            let (l, r) = super::get_inclusive_usize_bounds(&range, self.n);
            let len = r - l + 1;
            let log = len.ilog2() as usize;
            (self.func)(
                &self.sparse_table[log][l],
                &self.sparse_table[log][r + 1 - (1 << log)],
            )
        }
    }

    /// I is the index type
    pub struct RMQ<
        T: PartialOrd + Copy + Clone,
        const FINDING_MAXIMUM: bool,
        I: TryInto<usize> + TryFrom<usize> + Copy = usize,
    > {
        n: usize,
        a: Vec<T>,
        rmq_index: Vec<Vec<I>>,
    }

    impl<
        T: PartialOrd + Copy + Clone,
        const FINDING_MAXIMUM: bool,
        I: TryInto<usize> + TryFrom<usize> + Copy,
    > RMQ<T, FINDING_MAXIMUM, I>
    {
        pub fn new(a: &[T]) -> Self {
            let n = a.len();
            let log = n.ilog2() as usize;
            let table = vec![vec![]; log + 1];
            let mut obj = Self {
                n,
                a: a.to_vec(),
                rmq_index: table,
            };
            obj.build();
            obj
        }

        pub fn build(&mut self) {
            let log = self.n.ilog2() as usize;
            unsafe {
                self.rmq_index[0] = (0..self.n)
                    .map(|x| I::try_from(x).unwrap_unchecked())
                    .collect();
            }
            for b in 1..=log {
                self.rmq_index[b].reserve_exact(self.n - (1 << b) + 1);
                for i in 0..self.rmq_index[b].capacity() {
                    // Replace it with push_within_capacity() once its stable
                    let val = self.better_index(
                        self.rmq_index[b - 1][i],
                        self.rmq_index[b - 1][i + (1 << (b - 1))],
                    );
                    self.rmq_index[b].push(val);
                }
            }
        }

        pub fn better_index(&self, i: I, j: I) -> I {
            unsafe {
                let i_usize = i.try_into().unwrap_unchecked();
                let j_usize = j.try_into().unwrap_unchecked();
                if FINDING_MAXIMUM {
                    if self.a[j_usize].ge(&self.a[i_usize]) {
                        j
                    } else {
                        i
                    }
                } else {
                    if self.a[j_usize].le(&self.a[i_usize]) {
                        j
                    } else {
                        i
                    }
                }
            }
        }

        pub fn query_index<R: RangeBounds<usize>>(&self, range: R) -> usize {
            let (l, r) = super::get_inclusive_usize_bounds(&range, self.n);
            let len = r - l + 1;
            let log = len.ilog2() as usize;
            unsafe {
                self.better_index(
                    self.rmq_index[log][l],
                    self.rmq_index[log][r + 1 - (1 << log)],
                )
                .try_into()
                .unwrap_unchecked()
            }
        }

        pub fn query_value<R: RangeBounds<usize>>(&self, range: R) -> T {
            self.a[self.query_index(range)]
        }
    }
}
use sparse::*;
