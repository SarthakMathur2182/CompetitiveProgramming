/// # Modular Arithmetic ([source](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/Math/ModInt/mod_int.rs))
///
/// The way of using this is given right after the end of the module (choose the types and modulo accordingly).
///
/// You'll need the module [math.rs](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/CombinedRustModules/math.rs) to use the same.
pub mod mod_int {
    use super::math::custom_math_traits::*;
    use std::convert::{TryFrom, TryInto};
    use std::fmt::{Debug, Display, Error, Formatter};
    use std::ops::{
        Add, AddAssign, BitAnd, Div, DivAssign, Mul, MulAssign, Neg, Rem, ShrAssign, Sub, SubAssign,
    };

    #[repr(u64)]
    #[derive(Copy, Clone)]
    #[allow(non_camel_case_types)]
    pub enum MODS {
        Mod_1e9_7 = 1_000_000_007,
        Mod_1e9_9 = 1_000_000_009,
        Mod_998 = 998_244_353,
    }

    /// OpT denotes the operation type (mainly for multiplication)
    pub trait Modulo {
        type T: Copy
            + Clone
            + Ord // Keeping Ord for T in case we ever need it (but not for ModInt)
            + PartialOrd // Also this is required in modular inverse (it is the exponent)
            + BitAnd<Output = Self::T>
            + ShrAssign
            + Eq
            + PartialEq
            + Add<Output = Self::T>
            + Sub<Output = Self::T>
            + Default
            + MultiplicativeIdentity
            + From<u8>
            + Display
            + Debug;

        type OpT: Copy
            + Clone
            + Eq
            + PartialEq
            + Rem<Output = Self::OpT>
            + Mul<Output = Self::OpT>
            + From<Self::T>;

        const MOD: Self::T;

        /// Not using [TryInto] to avoid overhead.
        fn opt_to_t(val: Self::OpT) -> Self::T;
    }

    /// This struct does not type cast when adding, so make sure `2 * (MOD - 1) <= T::MAX`
    #[derive(Eq, PartialEq, Copy, Clone)]
    pub struct ModInt<M: Modulo>(M::T);

    impl<M: Modulo> ModInt<M> {
        pub fn new<U: Into<M::OpT>>(val: U) -> Self {
            Self(Self::normalize(val))
        }

        #[inline]
        pub fn normalize<U: Into<M::OpT>>(val: U) -> M::T {
            let rem = M::opt_to_t(val.into() % M::OpT::from(M::MOD));
            if rem < M::T::default() {
                rem + M::MOD
            } else {
                rem
            }
        }

        #[inline]
        /// Ideally this should be unsafe, but keeping it simple for now.
        pub fn sub_mod_if_big(val: M::T) -> M::T {
            if val >= M::MOD { val - M::MOD } else { val }
        }
    }

    impl<M: Modulo, U: Into<M::OpT>> From<U> for ModInt<M> {
        fn from(val: U) -> Self {
            Self::new(val)
        }
    }

    impl<M: Modulo, U: TryInto<M::OpT>> TryFrom<U> for ModInt<M> {
        type Error = TryInto<M::OpT>::Error;

        fn try_from(val: U) -> Result<Self, Self::Error> {
            Self::new(val.try_into())
        }
    }

    impl<M: Modulo> AddAssign for ModInt<M> {
        fn add_assign(&mut self, rhs: Self) {
            self.0 = ModInt::<M>::sub_mod_if_big(self.0 + rhs.0);
        }
    }

    impl<M: Modulo> Add for ModInt<M> {
        type Output = ModInt<M>;

        fn add(self, rhs: Self) -> Self::Output {
            let mut ans = self;
            ans += rhs;
            ans
        }
    }

    impl<M: Modulo> Default for ModInt<M> {
        fn default() -> Self {
            ModInt(M::T::default())
        }
    }

    impl<M: Modulo> Neg for ModInt<M> {
        type Output = ModInt<M>;

        fn neg(self) -> Self::Output {
            if self.0 == M::T::default() {
                self
            } else {
                ModInt(M::MOD - self.0)
            }
        }
    }

    impl<M: Modulo> SubAssign for ModInt<M> {
        fn sub_assign(&mut self, rhs: Self) {
            *self += -rhs;
        }
    }

    impl<M: Modulo> Sub for ModInt<M> {
        type Output = ModInt<M>;

        fn sub(self, rhs: Self) -> Self::Output {
            let mut ans = self;
            ans -= rhs;
            ans
        }
    }

    impl<M: Modulo> MulAssign for ModInt<M> {
        fn mul_assign(&mut self, rhs: Self) {
            self.0 = ModInt::<M>::normalize(M::OpT::from(self.0) * M::OpT::from(rhs.0));
        }
    }

    impl<M: Modulo> Mul for ModInt<M> {
        type Output = ModInt<M>;

        fn mul(self, rhs: Self) -> Self::Output {
            let mut ans = self;
            ans *= rhs;
            ans
        }
    }

    impl<M: Modulo> MultiplicativeIdentity for ModInt<M> {
        fn one() -> Self {
            ModInt(M::T::one())
        }
    }

    impl<M: Modulo> MultiplicativeInverse for ModInt<M> {
        type Output = ModInt<M>;

        fn mul_inv(&self) -> Self::Output {
            super::math::pow(*self, M::MOD - M::T::from(2u8))
        }
    }

    impl<M: Modulo> DivAssign for ModInt<M> {
        fn div_assign(&mut self, rhs: Self) {
            *self *= rhs.mul_inv();
        }
    }

    impl<M: Modulo> Div for ModInt<M> {
        type Output = ModInt<M>;

        fn div(self, rhs: Self) -> Self::Output {
            let mut ans = self;
            ans /= rhs;
            ans
        }
    }

    impl<M: Modulo> Debug for ModInt<M> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            write!(f, "Mint({})", self.0)
        }
    }

    impl<M: Modulo> Display for ModInt<M> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            write!(f, "{}", self.0)
        }
    }

    /// The factorials are present till `N-1` only, so initialize N accordingly.
    pub struct Combinatorics<M: Modulo, const N: usize> {
        factorial: Vec<ModInt<M>>,
        inverse_factorial: Vec<ModInt<M>>,
    }

    impl<M: Modulo, const N: usize> Combinatorics<M, N> {
        pub fn new() -> Self {
            let mut factorial = vec![ModInt::<M>::one(); N];
            for i in 1..N {
                factorial[i] = factorial[i - 1] * ModInt::try_from(i);
            }
            let mut inverse_factorial = vec![ModInt::<M>::one(); N];
            inverse_factorial[N - 1] = factorial[N - 1].mul_inv();
            for i in (0..N - 1).rev() {
                inverse_factorial[i] = inverse_factorial[i + 1] * ModInt::try_from(i + 1);
            }

            Self {
                factorial,
                inverse_factorial,
            }
        }

        pub fn fact<I>(&self, n: I) -> ModInt<M>
        where
            I: TryInto<usize> + Copy,
            <I as TryInto<usize>>::Error: Debug,
        {
            self.factorial[n.try_into().unwrap()]
        }

        pub fn inv_fact<I>(&self, n: I) -> ModInt<M>
        where
            I: TryInto<usize> + Copy,
            <I as TryInto<usize>>::Error: Debug,
        {
            self.inverse_factorial[n.try_into().unwrap()]
        }

        /// c(n, r) = n! / (r! * (n-r)!)
        pub fn c<I>(&self, n: I, r: I) -> ModInt<M>
        where
            I: TryInto<usize> + Copy,
            <I as TryInto<usize>>::Error: Debug,
        {
            let n = match n.try_into() {
                Ok(n) => n,
                Err(_) => {
                    return ModInt::default();
                }
            };
            let r = match r.try_into() {
                Ok(r) => r,
                Err(_) => {
                    return ModInt::default();
                }
            };
            if r > n {
                ModInt::default()
            } else {
                self.factorial[n] * self.inverse_factorial[r] * self.inverse_factorial[n - r]
            }
        }

        /// p(n, r) = n! / (n-r)!
        pub fn p<I>(&self, n: I, r: I) -> ModInt<M>
        where
            I: TryInto<usize> + Copy,
            <I as TryInto<usize>>::Error: Debug,
        {
            let n = match n.try_into() {
                Ok(n) => n,
                Err(_) => {
                    return ModInt::default();
                }
            };
            let r = match r.try_into() {
                Ok(r) => r,
                Err(_) => {
                    return ModInt::default();
                }
            };
            if r > n {
                ModInt::default()
            } else {
                self.factorial[n] * self.inverse_factorial[n - r]
            }
        }

        /// Using the precomputed factorials and inverse-factorials to calculate multiplicative inverse of a number
        /// Inverse of n = (n-1)! / n!
        pub fn inv<I>(&self, n: I) -> ModInt<M>
        where
            I: TryInto<usize> + Copy,
            <I as TryInto<usize>>::Error: Debug,
        {
            let n = n.try_into().unwrap();
            assert!(n > 0, "Inverse of 0 does not exist!");
            self.inverse_factorial[n] * self.factorial[n - 1]
        }

        pub fn c<I>(&self, n: I, r: I) -> ModInt<M>
        where
            I: TryInto<u32> + Copy,
            <I as TryInto<u32>>::Error: Debug,
        {
            let n = match n.try_into() {
                Ok(n) => n,
                Err(_) => {
                    return ModInt::default();
                }
            };
            let mut r = match r.try_into() {
                Ok(r) => r,
                Err(_) => {
                    return ModInt::default();
                }
            };

            if r > n {
                return ModInt::default();
            }

            if r > n - r {
                r = n - r;
            }

            let mut ans = ModInt::default();
            for i in 0..r {
                ans *= ModInt::try_from(n - i);
                ans /= self.inv(i + 1);
            }
            ans
        }
    }
}
use mod_int::*;
struct Mod1e9_7;
impl Modulo for Mod1e9_7 {
    type T = u32;
    type OpT = u64;
    const MOD: Self::T = MODS::Mod_1e9_7 as u32;

    fn opt_to_t(val: Self::OpT) -> Self::T {
        val as u32
    }
}
// type Mint = ModInt<Mod1e9_7>;

// This cannot be initialized globally (at least not directly)
// let comb: Combinatorics<Mod1e9_7, N> = Combinatorics::new();
