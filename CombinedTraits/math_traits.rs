// Expected to be inside pub mod math
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

    pub trait MultiplicativeInv {
        type Output;
        fn mul_inv(&self) -> Self::Output;
    }
}


