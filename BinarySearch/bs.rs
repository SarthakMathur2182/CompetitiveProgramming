pub mod math {
    pub mod custom_math_traits {
        pub trait MultiplicativeIdentity {
            fn mul_one() -> Self;
        }
        macro_rules! impl_multiplicative_identity_for_primitives {
            ($($t:ty),*) => {
                $(impl MultiplicativeIdentity for $t {
                    fn mul_one() -> Self {
                        1
                    }
                })*
            };
        }
        impl_multiplicative_identity_for_primitives!(
            u8, u16, u32, u64, usize, i8, i16, i32, i64, isize
        );

        // Currently using Default for Zero / AdditiveIdentity

        pub trait Midpoint {
            fn midpoint(a: Self, b: Self) -> Self;
        }
        macro_rules! impl_midpoint_trait_for_primitives {
            ($($t:ty),*) => {
                $(impl Midpoint for $t {fn midpoint(a: Self, b: Self) -> Self {
                    a.midpoint(b)
                }
                })*
            };
        }
        impl_midpoint_trait_for_primitives!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);
    }
}

/// Binary Search
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
        r = r + T::mul_one();
        while l < r {
            let m = T::midpoint(l, r);
            if predicate(m) {
                r = m;
            } else {
                l = m + T::mul_one();
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
        first_true(l, r, predicate) - T::mul_one()
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
