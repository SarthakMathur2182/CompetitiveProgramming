pub mod math {
    use std::ops::{BitAnd, Div, Mul, Rem, ShrAssign};

    pub mod custom_math_traits {
        pub trait MultiplicativeIdentity {
            fn one() -> Self;
        }
        macro_rules! impl_multiplicative_identity_for_integer_primitives {
            ($($t:ty),*) => {
                $(impl MultiplicativeIdentity for $t {
                    fn one() -> Self {
                        1 as $t
                    }
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
        // a.midpoint(b) is released in a quite latest version of rust.
        // Currently copying this from the rust library code.
        macro_rules! impl_midpoint_trait_for_unsigned_primitives {
            ($($t:ty),*) => {
                $(impl Midpoint for $t {fn midpoint(a: Self, b: Self) -> Self {
                    ((a ^ b) >> 1) + (a & b)
                }
                })*
            };
        }
        impl_midpoint_trait_for_unsigned_primitives!(u8, u16, u32, u64, u128, usize);
        macro_rules! impl_midpoint_trait_for_signed_primitives {
            ($($t:ty),*) => {
                $(impl Midpoint for $t {fn midpoint(a: Self, b: Self) -> Self {
                    let ans = ((a ^ b) >> 1) + (a & b);
                    ans + (if ans < 0 { 1 } else { 0 } & (a ^ b))
                }
                })*
            };
        }
        impl_midpoint_trait_for_signed_primitives!(i8, i16, i32, i64, i128, isize);
        macro_rules! impl_midpoint_trait_for_floating_primitives {
            ($($t:ty),*) => {
                $(impl Midpoint for $t {fn midpoint(a: Self, b: Self) -> Self {
                    a / 2 as $t + b / 2 as $t
                }
                })*
            };
        }
        impl_midpoint_trait_for_floating_primitives!(f32, f64);
        // TODO: Use this once the online judges update their version
        // macro_rules! impl_midpoint_trait_for_primitives {
        //     ($($t:ty),*) => {
        //         $(impl Midpoint for $t {fn midpoint(a: Self, b: Self) -> Self {
        //             a.midpoint(b)
        //         }
        //         })*
        //     };
        // }
        // impl_midpoint_trait_for_primitives!(
        //     u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
        // );
    }

    pub fn pow_with_identity<T, P>(mut base: T, mut exp: P, identity: T) -> T
    where
        T: Mul<Output = T> + Copy,
        P: Copy + PartialOrd + Default + BitAnd<Output = P> + ShrAssign + From<u8> + PartialEq,
    {
        let mut res = identity;
        while exp > P::default() {
            if (exp & P::from(1u8)) == P::from(1u8) {
                res = res * base;
            }
            base = base * base;
            exp >>= P::from(1u8);
        }
        res
    }

    pub fn pow<T, P>(base: T, exp: P) -> T
    where
        T: custom_math_traits::MultiplicativeIdentity + Mul<Output = T> + Copy,
        P: Copy + PartialOrd + Default + BitAnd<Output = P> + ShrAssign + From<u8> + PartialEq,
    {
        pow_with_identity(base, exp, T::one())
    }

    // Credits: https://codeforces.com/blog/entry/91800
    pub fn gcd<T: Default + Rem<Output = T> + PartialEq + Copy>(a: T, b: T) -> T {
        if b == T::default() {
            return a;
        }
        gcd(b, a % b)
    }

    pub fn lcm<
        T: Default + Rem<Output = T> + PartialEq + Copy + Div<Output = T> + Mul<Output = T>,
    >(
        a: T,
        b: T,
    ) -> T {
        a / gcd(a, b) * b
    }
}
use custom_math_traits::*;
use math::*;
