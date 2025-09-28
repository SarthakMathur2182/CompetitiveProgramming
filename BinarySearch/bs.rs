/// # Binary Search ([source](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/BinarySearch/bs.rs))
///
/// You'll need the traits in module [math.rs](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/CombinedRustModules/math.rs) to use the same.
pub mod bs {
    use std::ops::{Add, Sub};

    pub fn first_true<T, F>(mut l: T, mut r: T, mut predicate: F) -> T
    where
        T: Copy
            + Add<Output = T>
            + super::math::custom_math_traits::MultiplicativeIdentity
            + PartialOrd
            + super::math::custom_math_traits::Midpoint,
        F: FnMut(T) -> bool,
    {
        r = r + T::one();
        while l < r {
            let m = T::midpoint(l, r);
            if predicate(m) {
                r = m;
            } else {
                l = m + T::one();
            }
        }

        r
    }

    pub fn last_false<T, F>(l: T, r: T, predicate: F) -> T
    where
        T: Copy
            + Add<Output = T>
            + super::math::custom_math_traits::MultiplicativeIdentity
            + PartialOrd
            + super::math::custom_math_traits::Midpoint
            + Sub<Output = T>,
        F: FnMut(T) -> bool,
    {
        first_true(l, r, predicate) - T::one()
    }

    pub fn first_false<T, F>(l: T, r: T, mut predicate: F) -> T
    where
        T: Copy
            + Add<Output = T>
            + super::math::custom_math_traits::MultiplicativeIdentity
            + PartialOrd
            + super::math::custom_math_traits::Midpoint,
        F: FnMut(T) -> bool,
    {
        first_true(l, r, |t| !predicate(t))
    }

    pub fn last_true<T, F>(l: T, r: T, mut predicate: F) -> T
    where
        T: Copy
            + Add<Output = T>
            + super::math::custom_math_traits::MultiplicativeIdentity
            + PartialOrd
            + super::math::custom_math_traits::Midpoint
            + Sub<Output = T>,
        F: FnMut(T) -> bool,
    {
        last_false(l, r, |t| !predicate(t))
    }
}
