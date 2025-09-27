pub mod math {
    use std::ops::{BitAnd, Mul, ShrAssign};

    pub mod custom_math_traits {
        pub trait MultiplicativeIdentity {
            const ONE: Self;
        }
        macro_rules! impl_multiplicative_identity_for_integer_primitives {
            ($($t:ty),*) => {
                $(impl MultiplicativeIdentity for $t {
                    const ONE: Self = 1 as $t;
                })*
            };
        }
        impl_multiplicative_identity_for_integer_primitives!(
            u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
        );

        // Currently using Default for Zero / AdditiveIdentity

        pub trait MultiplicativeInverse {
            type Output;
            fn mul_inv(&self) -> Self::Output;
        }
        macro_rules! impl_multiplicative_inverse_for_primitives {
            ($($t:ty),*) => {
                $(impl MultiplicativeInverse for $t {
                    type Output = f64;
                    fn mul_inv(&self) -> Self::Output {
                        1f64 / (self.clone() as f64)
                    }
                })*
            };
        }
        impl_multiplicative_inverse_for_primitives!(
            u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f64
        );
        impl MultiplicativeInverse for f32 {
            type Output = f32;
            fn mul_inv(&self) -> Self::Output {
                1f32 / self
            }
        }

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
        impl_midpoint_trait_for_primitives!(
            u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
        );
    }

    pub fn pow<T, P>(mut base: T, mut exp: P) -> T
    where
        T: custom_math_traits::MultiplicativeIdentity + Mul<Output = T> + Copy,
        P: Copy + PartialOrd + Default + BitAnd<Output = P> + ShrAssign + From<u8> + PartialEq,
    {
        let mut res = T::ONE;
        while exp > P::default() {
            if (exp & P::from(1u8)) == P::from(1u8) {
                res = res * base;
            }
            base = base * base;
            exp >>= P::from(1u8);
        }
        res
    }

    // Credits: https://codeforces.com/blog/entry/91800
    pub fn gcd<T: Default + std::ops::Rem<Output = T> + PartialEq + Copy>(a: T, b: T) -> T {
        if b == T::default() {
            return a;
        }
        gcd(b, a % b)
    }
}
use custom_math_traits::*;
use math::*;
