/// Polynomial Hash ([source](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/String/polynomial_hash.rs))
///
/// You'll need the modules [math.rs](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/CombinedRustModules/math.rs) and [mod_int.rs](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/Math/ModInt/mod_int.rs) to use the same.
pub mod polynomial_hash {
    use super::*;
    use std::convert::{TryFrom, TryInto};
    use std::fmt::Debug;

    // TODO: Consider if creating a trait is a better idea.

    /// For a slice `a` of length `n` and a base `b`, it stores, in a prefix sum, the hash:
    ///
    /// `a[0] * b + a[1] * b^2 + ... + a[n - 2] * b^(n-1) + a[n - 1] * b^n` (mod [M::MOD])
    pub struct PolynomialHash<'a, M: Modulo> {
        pub b: ModInt<M>,
        pub hash_pre: Vec<ModInt<M>>,
        inv_pow_b: &'a Vec<ModInt<M>>,
    }

    impl<'a, M: Modulo> PolynomialHash<'a, M> {
        pub fn new<T: Copy>(a: &[T], b: ModInt<M>, inv_pow_b: &'a Vec<ModInt<M>>) -> Self
        where
            M::OpT: TryFrom<T>,
            <M::OpT as TryFrom<T>>::Error: Debug,
        {
            let n = a.len();
            let mut hash_pre = vec![ModInt::<M>::default(); n + 1];
            let mut pow_b1 = b;
            for i in 1..=n {
                let h = pow_b1 * ModInt::<M>::new(M::OpT::try_from(a[i - 1]).unwrap());
                hash_pre[i] = hash_pre[i - 1] + h;
                pow_b1 *= b;
            }

            Self {
                b,
                hash_pre,
                inv_pow_b,
            }
        }

        pub fn get_hash(&self, l: usize, r: usize) -> ModInt<M> {
            let h = (self.hash_pre[r + 1] - self.hash_pre[l]) * self.inv_pow_b[l];
            h
        }
    }

    /// Internally uses 2 instances of [PolynomialHash].
    pub struct DoubleHash<'a, M1: Modulo, M2: Modulo> {
        hash1: PolynomialHash<'a, M1>,
        hash2: PolynomialHash<'a, M2>,
    }

    impl<'a, M1: Modulo, M2: Modulo> DoubleHash<'a, M1, M2> {
        pub fn new<T: Copy>(
            a: &[T],
            b1: ModInt<M1>,
            inv_pow_b1: &'a Vec<ModInt<M1>>,
            b2: ModInt<M2>,
            inv_pow_b2: &'a Vec<ModInt<M2>>,
        ) -> Self
        where
            M1::OpT: TryFrom<T>,
            <M1::OpT as TryFrom<T>>::Error: Debug,
            M2::OpT: TryFrom<T>,
            <M2::OpT as TryFrom<T>>::Error: Debug,
        {
            Self {
                hash1: PolynomialHash::new(a, b1, inv_pow_b1),
                hash2: PolynomialHash::new(a, b2, inv_pow_b2),
            }
        }

        pub fn get_hash(&self, l: usize, r: usize) -> (ModInt<M1>, ModInt<M2>) {
            (self.hash1.get_hash(l, r), self.hash2.get_hash(l, r))
        }
    }
}
use polynomial_hash::*;
