/// # Matrix ([source](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/Matrix/matrix.rs))
///
/// You'll need the traits in module [math.rs](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/CombinedRustModules/math.rs) to use the same.
pub mod matrix {
    use std::fmt::{Debug, Error, Formatter};
    use std::ops::{Add, AddAssign, BitAnd, Mul, MulAssign, Neg, ShrAssign, Sub, SubAssign};

    #[derive(Clone)]
    pub struct Matrix<T: Clone> {
        pub n: usize,
        pub m: usize,
        pub mat: Vec<Vec<T>>,
    }

    impl<T: Clone + Default> Matrix<T> {
        pub fn zero(n: usize, m: usize) -> Self {
            Self {
                n,
                m,
                mat: vec![vec![T::default(); m]; n],
            }
        }

        pub fn identity(n: usize) -> Self
        where
            T: super::MultiplicativeIdentity,
        {
            let mut mat = Self::zero(n, n);
            for i in 0..n {
                mat.mat[i][i] = T::one();
            }
            mat
        }

        pub fn pow<P>(&self, mut exp: P) -> Matrix<T>
        where
            T: Mul<Output = T> + super::MultiplicativeIdentity + AddAssign,
            P: Copy + PartialOrd + Default + BitAnd<Output = P> + ShrAssign + From<u8> + PartialEq,
        {
            assert!(
                self.n == self.m,
                "Only square matrix can be multiplied to itself!"
            );
            assert!(exp >= P::default(), "Negative power is not supported!");
            let mut base = self.clone();
            let mut res = Self::identity(self.n);
            while exp > P::default() {
                if (exp & P::from(1u8)) == P::from(1u8) {
                    res *= &base;
                }
                base = &base * &base;
                exp >>= P::from(1u8);
            }
            res
        }
    }

    impl<T: Clone + AddAssign> AddAssign<&Matrix<T>> for Matrix<T> {
        fn add_assign(&mut self, rhs: &Matrix<T>) {
            assert!(
                self.n == rhs.n && self.m == rhs.m,
                "Addition requires same dimensions!"
            );
            for i in 0..self.n {
                for j in 0..self.m {
                    self.mat[i][j] += rhs.mat[i][j].clone();
                }
            }
        }
    }

    impl<T: Clone + AddAssign> Add<&Matrix<T>> for &Matrix<T> {
        type Output = Matrix<T>;

        fn add(self, rhs: &Matrix<T>) -> Self::Output {
            let mut ans = self.clone();
            ans += &rhs;
            ans
        }
    }

    impl<T: Clone + Neg<Output = T> + Default> Neg for &Matrix<T> {
        type Output = Matrix<T>;

        fn neg(self) -> Self::Output {
            let mut ans = Matrix::<T>::zero(self.n, self.m);
            for i in 0..self.n {
                for j in 0..self.m {
                    ans.mat[i][j] = -self.mat[i][j].clone();
                }
            }
            ans
        }
    }

    impl<T: Clone + SubAssign> SubAssign<&Matrix<T>> for Matrix<T> {
        fn sub_assign(&mut self, rhs: &Matrix<T>) {
            assert!(
                self.n == rhs.n && self.m == rhs.m,
                "Subtraction requires same dimensions!"
            );
            for i in 0..self.n {
                for j in 0..self.m {
                    self.mat[i][j] -= rhs.mat[i][j].clone();
                }
            }
        }
    }

    impl<T: Clone + SubAssign> Sub<&Matrix<T>> for &Matrix<T> {
        type Output = Matrix<T>;

        fn sub(self, rhs: &Matrix<T>) -> Self::Output {
            let mut ans = self.clone();
            ans -= &rhs;
            ans
        }
    }

    impl<T: Clone + AddAssign + Mul<Output = T> + Default> Mul<&Matrix<T>> for &Matrix<T> {
        type Output = Matrix<T>;

        fn mul(self, rhs: &Matrix<T>) -> Self::Output {
            let n = self.mat.len();
            let m = self.mat[0].len();
            assert!(m == rhs.mat.len(), "Matrix multiplication not possible");
            let l = rhs.mat[0].len();

            let mut ans = Matrix::zero(n, l);
            for i in 0..n {
                for j in 0..l {
                    for k in 0..m {
                        ans.mat[i][j] += self.mat[i][k].clone() * rhs.mat[k][j].clone();
                    }
                }
            }
            ans
        }
    }

    // Use it carefully, I'm allowing the updated matrix to be of differnt dimensions.
    impl<T: Clone + AddAssign + Mul<Output = T> + Default> MulAssign<&Matrix<T>> for Matrix<T> {
        fn mul_assign(&mut self, rhs: &Matrix<T>) {
            let clone = (&*self) * rhs;
            *self = clone;
        }
    }

    // TODO: Inverse

    impl<T: Debug + Clone> Debug for Matrix<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            for i in 0..self.n {
                writeln!(f)?;
                for j in 0..self.m {
                    write!(f, "{:?} ", self.mat[i][j])?;
                }
            }
            Ok(())
        }
    }
}
use matrix::*;
