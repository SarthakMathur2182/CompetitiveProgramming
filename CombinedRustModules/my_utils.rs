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
}
use my_utils::*;
