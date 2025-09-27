pub mod utils {
    use std::ops::{Bound, RangeBounds};

    /// I'm just too comfortable with inclusive bounds.
    pub fn get_inclusive_usize_bounds(
        range: &impl RangeBounds<usize>,
        end_exclusive: usize,
    ) -> (usize, usize) {
        let l = match range.start_bound() {
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x + 1,
            Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x - 1,
            Bound::Unbounded => end_exclusive - 1,
        };
        (l, r)
    }
}

pub mod sparse {
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
                for i in 1..table[b - 1].len() {
                    // Replace it with push_within_capacity() once its stable
                    // let val = func(&table[b - 1][i - 1], &table[b - 1][i]);
                    table[b].push(func(&table[b - 1][i - 1], &table[b - 1][i]));
                }
            }

            Self {
                n,
                func,
                sparse_table: table,
            }
        }

        pub fn query<R: RangeBounds<usize>>(&self, range: R) -> T {
            let (l, r) = super::utils::get_inclusive_usize_bounds(range, self.n);
            let len = r - l + 1;
            // let log = (usize::BITS - len.leading_zeros() - 1) as usize;
            let log = len.ilog2() as usize;
            (self.func)(
                &self.sparse_table[log][l],
                &self.sparse_table[log][r - (1 << log) + 1],
            )
        }
    }

    pub struct RMQ<T: PartialOrd + Copy + Clone, const FindingMaximum: bool> {
        a: Vec<T>,
        rmq_index: Vec<Vec<usize>>,
    }

    impl<T: PartialOrd + Copy + Clone, const FindingMaximum: bool> RMQ<T, FindingMaximum> {
        pub fn new(a: &[T]) -> Self {
            let better_index = |i: usize, j: usize| -> usize {
                if FindingMaximum {
                    if a[j].ge(&a[i]) { j } else { i }
                } else {
                    if a[j].le(&a[i]) { j } else { i }
                }
            };

            Self {
                a: a.to_vec(),
                indices_sparse_table: SparseTable::new(
                    &(0..a.len()).collect::<Vec<usize>>(),
                    Self::better_index, // |i, j| Self::better_index(i, j, a),
                ),
            }
        }

        pub fn query_index<R: RangeBounds<usize>>(&self, range: R) -> usize {
            self.indices_sparse_table.query(range)
        }

        pub fn query_value<R: RangeBounds<usize>>(&self, range: R) -> T {
            self.a[self.query_index(range)]
        }
    }
}
