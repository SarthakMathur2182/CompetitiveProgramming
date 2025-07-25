pub mod math {
    use std::ops::{BitAnd, Mul, ShrAssign};

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

        pub trait MultiplicativeInv {
            type Output;
            fn mul_inv(&self) -> Self::Output;
        }
    }

    pub fn pow<T, P>(mut base: T, mut exp: P) -> T
    where
        T: custom_math_traits::MultiplicativeIdentity + Mul<Output = T> + Copy,
        P: Copy + PartialOrd + Default + BitAnd<Output = P> + ShrAssign + From<u8> + PartialEq,
    {
        let mut res = T::mul_one();
        while exp > P::default() {
            if (exp & P::from(1u8)) == P::from(1u8) {
                res = res * base;
            }
            base = base * base;
            exp >>= P::from(1u8);
        }
        res
    }

    pub mod mod_int {
        use super::custom_math_traits::MultiplicativeIdentity;
        use std::convert::TryInto;
        use std::fmt::{Debug, Display, Error, Formatter};
        use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

        pub mod modulos {
            use std::fmt::{Debug, Display};
            use std::ops::{Add, BitAnd, Mul, Rem, ShrAssign, Sub, SubAssign};

            #[repr(u64)]
            #[derive(Copy, Clone)]
            #[allow(non_camel_case_types)]
            pub enum MOD {
                Mod_1e9_7 = 1_000_000_007,
                Mod_1e9_9 = 1_000_000_009,
                Mod_998 = 998_244_353,
            }

            pub trait Modulo {
                type T: Copy
                    + Ord
                    + PartialOrd
                    + Eq
                    + PartialEq
                    + Add<Output = Self::T>
                    + Sub<Output = Self::T>
                    + SubAssign
                    + Default
                    + super::super::custom_math_traits::MultiplicativeIdentity
                    + From<u8>
                    + BitAnd<Output = Self::T>
                    + ShrAssign
                    + Display
                    + Debug;

                type OpT: Copy
                    + Ord
                    + PartialOrd
                    + Eq
                    + PartialEq
                    + From<Self::T>
                    + Add<Output = Self::OpT>
                    + Sub<Output = Self::OpT>
                    + SubAssign
                    + Rem<Output = Self::OpT>
                    + Mul<Output = Self::OpT>;

                const MOD: Self::T;

                fn opt_to_t(val: Self::OpT) -> Self::T;
            }
        }

        /// OpT denotes the operation type (mainly for multiplication)
        /// This struct does not type cast when adding, so make sure `2 * (MOD - 1) <= T::MAX`
        #[derive(Eq, PartialEq)]
        pub struct ModInt<M: modulos::Modulo> {
            val: M::T,
        }

        impl<M: modulos::Modulo> Clone for ModInt<M>
        where
            M::T: Copy,
        {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<M: modulos::Modulo> Copy for ModInt<M> where M::T: Copy {}

        impl<M: modulos::Modulo> ModInt<M> {
            pub fn from<U: Into<M::OpT>>(val: U) -> Self {
                Self {
                    val: Self::norm(val),
                }
            }

            #[inline]
            pub fn norm<U: Into<M::OpT>>(val: U) -> M::T {
                let rem = M::opt_to_t(val.into() % M::OpT::from(M::MOD));
                if rem < M::T::default() {
                    rem + M::MOD
                } else {
                    rem
                }
            }

            #[inline]
            pub fn sub_mod_if_big(val: M::T) -> M::T {
                if val >= M::MOD { val - M::MOD } else { val }
            }
        }

        impl<M: modulos::Modulo> AddAssign for ModInt<M> {
            fn add_assign(&mut self, rhs: Self) {
                self.val = ModInt::<M>::sub_mod_if_big(self.val + rhs.val);
            }
        }

        impl<M: modulos::Modulo> Add for ModInt<M> {
            type Output = ModInt<M>;

            fn add(self, rhs: Self) -> Self::Output {
                let mut ans = self;
                ans += rhs;
                ans
            }
        }

        impl<M: modulos::Modulo> Default for ModInt<M> {
            fn default() -> Self {
                ModInt::from(M::T::default())
            }
        }

        impl<M: modulos::Modulo> Neg for ModInt<M> {
            type Output = ModInt<M>;

            fn neg(self) -> Self::Output {
                if self.val == M::T::default() {
                    self
                } else {
                    ModInt::from(M::MOD - self.val)
                }
            }
        }

        impl<M: modulos::Modulo> SubAssign for ModInt<M> {
            fn sub_assign(&mut self, rhs: Self) {
                *self += -rhs;
            }
        }

        impl<M: modulos::Modulo> Sub for ModInt<M> {
            type Output = ModInt<M>;

            fn sub(self, rhs: Self) -> Self::Output {
                let mut ans = self;
                ans -= rhs;
                ans
            }
        }

        impl<M: modulos::Modulo> MulAssign for ModInt<M> {
            fn mul_assign(&mut self, rhs: Self) {
                self.val = ModInt::<M>::norm(M::OpT::from(self.val) * M::OpT::from(rhs.val));
            }
        }

        impl<M: modulos::Modulo> Mul for ModInt<M> {
            type Output = ModInt<M>;

            fn mul(self, rhs: Self) -> Self::Output {
                let mut ans = self;
                ans *= rhs;
                ans
            }
        }

        impl<M: modulos::Modulo> MultiplicativeIdentity for ModInt<M> {
            fn mul_one() -> Self {
                ModInt::from(M::T::mul_one())
            }
        }

        use super::custom_math_traits::MultiplicativeInv;
        impl<M: modulos::Modulo> MultiplicativeInv for ModInt<M> {
            type Output = ModInt<M>;

            fn mul_inv(&self) -> Self::Output {
                super::pow(*self, M::MOD - M::T::from(2u8))
            }
        }

        impl<M: modulos::Modulo> DivAssign for ModInt<M> {
            fn div_assign(&mut self, rhs: Self) {
                *self *= rhs.mul_inv();
            }
        }

        impl<M: modulos::Modulo> Div for ModInt<M> {
            type Output = ModInt<M>;

            fn div(self, rhs: Self) -> Self::Output {
                let mut ans = self;
                ans /= rhs;
                ans
            }
        }

        impl<M: modulos::Modulo> Debug for ModInt<M> {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
                write!(f, "Mint({})", self.val)
            }
        }

        impl<M: modulos::Modulo> Display for ModInt<M> {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
                write!(f, "{}", self.val)
            }
        }

        /// The factorials are present till `N-1` only, so initialize N accordingly.
        pub struct Combinatorics<M: modulos::Modulo, const N: usize> {
            factorial: Box<[ModInt<M>; N]>,
            inverse_factorial: Box<[ModInt<M>; N]>,
        }

        impl<M: modulos::Modulo, const N: usize> Combinatorics<M, N> {
            pub fn new() -> Self
            where
                M::OpT: From<u32>,
            {
                let mut factorial = Box::new([ModInt::<M>::mul_one(); N]);
                for i in 1..N {
                    factorial[i] = factorial[i - 1] * ModInt::from(i as u32);
                }
                let mut inverse_factorial = Box::new([ModInt::<M>::mul_one(); N]);
                inverse_factorial[N - 1] = factorial[N - 1].mul_inv();
                for i in (0..N - 1).rev() {
                    inverse_factorial[i] = inverse_factorial[i + 1] * ModInt::from(i as u32 + 1);
                }

                Self {
                    factorial,
                    inverse_factorial,
                }
            }

            pub fn fact<I>(&self, n: I) -> ModInt<M>
            where
                I: TryInto<usize>,
                <I as TryInto<usize>>::Error: Debug,
            {
                self.factorial[n.try_into().unwrap()]
            }

            pub fn inv_fact<I>(&self, n: I) -> ModInt<M>
            where
                I: TryInto<usize>,
                <I as TryInto<usize>>::Error: Debug,
            {
                self.inverse_factorial[n.try_into().unwrap()]
            }

            /// c(n, r) = n! / (r! * (n-r)!)
            pub fn c<I>(&self, n: I, r: I) -> ModInt<M>
            where
                I: TryInto<usize>,
                <I as TryInto<usize>>::Error: Debug,
            {
                let n = n.try_into().unwrap();
                let r = r.try_into().unwrap();
                if r < 0 || r > n {
                    ModInt::default()
                } else {
                    self.factorial[n] * self.inverse_factorial[r] * self.inverse_factorial[n - r]
                }
            }

            /// p(n, r) = n! / (n-r)!
            pub fn p<I>(&self, n: I, r: I) -> ModInt<M>
            where
                I: TryInto<usize>,
                <I as TryInto<usize>>::Error: Debug,
            {
                let n = n.try_into().unwrap();
                let r = r.try_into().unwrap();
                if r < 0 || r > n {
                    ModInt::default()
                } else {
                    self.factorial[n] * self.inverse_factorial[n - r]
                }
            }

            /// Using the precomputed factorials and inverse-factorials to calculate multiplicative inverse of a number
            /// Inverse of n = (n-1)! / n!
            pub fn inv<I>(&self, n: I) -> ModInt<M>
            where
                I: TryInto<usize>,
                <I as TryInto<usize>>::Error: Debug,
            {
                let n = n.try_into().unwrap();
                assert!(n > 0, "Inverse of 0 does not exist!");
                self.inverse_factorial[n] * self.factorial[n - 1]
            }
        }
    }
}
use math::mod_int::*;
struct Mod1e9_7;
impl modulos::Modulo for Mod1e9_7 {
    type T = u32;
    type OpT = u64;
    const MOD: Self::T = modulos::MOD::Mod_1e9_7 as u32;

    fn opt_to_t(val: Self::OpT) -> Self::T {
        val as u32
    }
}
type Mint = ModInt<Mod1e9_7>;
// This cannot be initialized globally (at least not directly)
// let comb: Combinatorics<Mod1e9_7, N> = Combinatorics::new();
