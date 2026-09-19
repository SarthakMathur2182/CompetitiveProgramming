/// # Utils ([source](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/CombinedRustModules/my_utils.rs))
///
/// Some common utils, might add more in the future.
pub mod my_utils {
    use std::convert::TryFrom;
    use std::ops::{AddAssign, Bound, RangeBounds};

    /// I'm just comfortable with inclusive bounds.
    pub fn get_inclusive_usize_bounds(
        range: &impl RangeBounds<usize>,
        end_exclusive: usize,
    ) -> (usize, usize) {
        (
            match range.start_bound() {
                Bound::Included(&x) => x,
                Bound::Excluded(&x) => x + 1,
                Bound::Unbounded => 0,
            },
            match range.end_bound() {
                Bound::Included(&x) => x,
                Bound::Excluded(&x) => x - 1,
                Bound::Unbounded => end_exclusive - 1,
            },
        )
    }

    /// Returns a tuple of 2 vectors:
    /// 1. The result vector with the values replaced by the compressed values.
    /// 2. The mapping from the compressed value (index of the array) to the original value.
    pub fn coordinate_compression<T: Clone + Ord, Comp: TryFrom<usize>>(
        a: &[T],
    ) -> (Vec<Comp>, Vec<T>) {
        let mut comp_to_val = a.to_vec();
        comp_to_val.sort_unstable();
        comp_to_val.dedup();

        let ans: Vec<Comp> = a
            .iter()
            .map(
                |x| match Comp::try_from(comp_to_val.binary_search(&x).unwrap()) {
                    Ok(x) => x,
                    Err(_) => {
                        panic!("Conversion from usize in coordinate compression failed!")
                    }
                },
            )
            .collect();

        (ans, comp_to_val)
    }

    /// This util will modify the slice passed.
    pub fn mex<T: Ord + Default + AddAssign + TryFrom<u8>>(a: &mut [T]) -> T {
        a.sort_unstable();
        let mut mex = T::default();
        for x in &*a {
            if mex == *x {
                mex += match T::try_from(1u8) {
                    Ok(x) => x,
                    Err(_) => {
                        panic!("The mex type should implement TryFrom<u8>!")
                    }
                };
            }
        }
        mex
    }
}
use my_utils::*;
