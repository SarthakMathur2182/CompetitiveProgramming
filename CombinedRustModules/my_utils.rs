/// # Utils ([source](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/CombinedRustModules/my_utils.rs))
///
/// Some common utils, might add more in the future.
pub mod my_utils {
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

    pub fn coordinate_compression<T: Clone + Ord>(a: &[T]) -> (Vec<u32>, Vec<T>) {
        let mut comp_to_val = a.to_vec();
        comp_to_val.sort_unstable();
        comp_to_val.dedup();

        let n = a.len();
        let mut ans = vec![0; n];
        for i in 0..n {
            ans[i] = comp_to_val.binary_search(&a[i]).unwrap() as u32;
        }

        (ans, comp_to_val)
    }
}
use my_utils::*;
