pub mod fraction {
    use super::custom_math_traits::{MultiplicativeIdentity, MultiplicativeInverse};
    use std::convert::TryInto;
    use std::fmt::{Debug, Display, Formatter};
    use std::marker::PhantomData;
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

    // TODO: Replace this once https://github.com/rust-lang/rust/issues/41517 is solved
    pub trait FractionOperationType:
        Copy
        + Ord
        + Default
        + MultiplicativeIdentity
        + AddAssign
        + Add<Output = Self>
        + SubAssign
        + Sub<Output = Self>
        + MulAssign
        + Mul<Output = Self>
        + DivAssign
        + Div<Output = Self>
        + Display
        + Debug
        + Rem<Output = Self>
    {
    }

    impl<
        T: Copy
            + Ord
            + Default
            + MultiplicativeIdentity
            + AddAssign
            + Add<Output = Self>
            + SubAssign
            + Sub<Output = Self>
            + MulAssign
            + Mul<Output = Self>
            + DivAssign
            + Div<Output = Self>
            + Display
            + Debug
            + Rem<Output = Self>,
    > FractionOperationType for T
    {
    }

    /// OpT is the operation type, mainly for addition and multiplication
    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
    pub struct Fraction<T: FractionOperationType, OpT: FractionOperationType + From<T> + TryInto<T>> {
        pub num: T,
        pub den: T,
        phantom: PhantomData<OpT>,
    }

    impl<T: FractionOperationType, OpT: FractionOperationType + From<T> + TryInto<T>> Fraction<T, OpT> {
        pub fn new<U: Into<T>>(num: U, den: U) -> Fraction<T, OpT> {
            let num = num.into();
            let den = den.into();
            let g = super::gcd(num, den);
            assert!(den != T::default(), "Denominator cannot be zero!");
            Self {
                num: num / g,
                den: den / g,
                phantom: PhantomData,
            }
        }
    }

    impl<T: FractionOperationType, OpT: FractionOperationType + From<T> + TryInto<T>> AddAssign
        for Fraction<T, OpT>
    {
        fn add_assign(&mut self, rhs: Self) {
            let lcm = super::lcm(OpT::from(self.den), OpT::from(rhs.den));
            let num = lcm / OpT::from(self.den) * OpT::from(self.num)
                + lcm / OpT::from(rhs.den) * OpT::from(rhs.num);
            let new: Fraction<OpT, OpT> = Fraction::new(num, lcm);
            let num = new.num.try_into();
            let den = new.den.try_into();
            assert!(num.is_ok() && den.is_ok(), "Type too small to fit the sum!");
            unsafe {
                self.num = num.unwrap_unchecked();
                self.den = den.unwrap_unchecked();
            }
        }
    }

    impl<T: FractionOperationType, OpT: FractionOperationType + From<T> + TryInto<T>> Add
        for Fraction<T, OpT>
    {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            let mut ans = self;
            ans += rhs;
            ans
        }
    }

    impl<T: FractionOperationType, OpT: FractionOperationType + From<T> + TryInto<T>> Default
        for Fraction<T, OpT>
    {
        fn default() -> Self {
            Self::new(T::default(), T::one())
        }
    }

    impl<
        T: FractionOperationType + Neg<Output = T>,
        OpT: FractionOperationType + From<T> + TryInto<T>,
    > Neg for Fraction<T, OpT>
    {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Self {
                num: -self.num,
                den: self.den,
                phantom: PhantomData,
            }
        }
    }

    impl<
        T: FractionOperationType + Neg<Output = T>,
        OpT: FractionOperationType + From<T> + TryInto<T>,
    > SubAssign for Fraction<T, OpT>
    {
        fn sub_assign(&mut self, rhs: Self) {
            *self += -rhs;
        }
    }

    impl<
        T: FractionOperationType + Neg<Output = T>,
        OpT: FractionOperationType + From<T> + TryInto<T>,
    > Sub for Fraction<T, OpT>
    {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            let mut ans = self;
            ans -= rhs;
            ans
        }
    }

    impl<T: FractionOperationType, OpT: FractionOperationType + From<T> + TryInto<T>> MulAssign
        for Fraction<T, OpT>
    {
        fn mul_assign(&mut self, rhs: Self) {
            let new: Fraction<OpT, OpT> = Fraction::new(
                OpT::from(self.num) * OpT::from(rhs.num),
                OpT::from(self.den) * OpT::from(rhs.den),
            );
            let num = new.num.try_into();
            let den = new.den.try_into();
            assert!(
                num.is_ok() && den.is_ok(),
                "Type too small to fit the product!"
            );
            unsafe {
                self.num = num.unwrap_unchecked();
                self.den = den.unwrap_unchecked();
            }
        }
    }

    impl<T: FractionOperationType, OpT: FractionOperationType + From<T> + TryInto<T>> Mul
        for Fraction<T, OpT>
    {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            let mut ans = self;
            ans *= rhs;
            ans
        }
    }

    impl<T: FractionOperationType, OpT: FractionOperationType + From<T> + TryInto<T>>
        MultiplicativeIdentity for Fraction<T, OpT>
    {
        fn one() -> Self {
            Self {
                num: T::one(),
                den: T::one(),
                phantom: PhantomData,
            }
        }
    }

    impl<T: FractionOperationType, OpT: FractionOperationType + From<T> + TryInto<T>>
        MultiplicativeInverse for Fraction<T, OpT>
    {
        type Output = Self;

        fn mul_inv(&self) -> Self::Output {
            assert!(self.num != T::default(), "Inverse of 0 doesn't exist!");
            Self {
                num: self.den,
                den: self.num,
                phantom: PhantomData,
            }
        }
    }

    impl<T: FractionOperationType, OpT: FractionOperationType + From<T> + TryInto<T>> DivAssign
        for Fraction<T, OpT>
    {
        fn div_assign(&mut self, rhs: Self) {
            *self *= rhs.mul_inv();
        }
    }

    impl<T: FractionOperationType, OpT: FractionOperationType + From<T> + TryInto<T>> Div
        for Fraction<T, OpT>
    {
        type Output = Self;

        fn div(self, rhs: Self) -> Self::Output {
            let mut ans = self;
            ans /= rhs;
            ans
        }
    }

    impl<T: FractionOperationType, OpT: FractionOperationType + From<T> + TryInto<T>> Debug
        for Fraction<T, OpT>
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}/{}", self.num, self.den)
        }
    }

    impl<T: FractionOperationType, OpT: FractionOperationType + From<T> + TryInto<T>> Display
        for Fraction<T, OpT>
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}
use fraction::*;
