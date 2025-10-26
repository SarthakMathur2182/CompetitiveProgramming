/// # Binary Search ([source](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/BinarySearch/bs.rs))
///
/// You'll need the traits in module [math.rs](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/CombinedRustModules/math.rs) to use the same.
#[allow(private_bounds)]
pub mod bs {
    // TODO: Replace this once https://github.com/rust-lang/rust/issues/41517 is solved
    trait BinarySearchRangeType:
        Copy
        + std::ops::Add<Output = Self>
        + super::MultiplicativeIdentity
        + PartialOrd
        + super::Midpoint
        + std::ops::Sub<Output = Self>
    {
    }

    impl<
        T: Copy
            + std::ops::Add<Output = Self>
            + super::MultiplicativeIdentity
            + PartialOrd
            + super::Midpoint
            + std::ops::Sub<Output = Self>,
    > BinarySearchRangeType for T
    {
    }

    pub fn first_true<T, F>(mut l: T, mut r: T, mut predicate: F) -> T
    where
        T: BinarySearchRangeType,
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
        T: BinarySearchRangeType,
        F: FnMut(T) -> bool,
    {
        first_true(l, r, predicate) - T::one()
    }

    pub fn first_false<T, F>(l: T, r: T, mut predicate: F) -> T
    where
        T: BinarySearchRangeType,
        F: FnMut(T) -> bool,
    {
        first_true(l, r, |t| !predicate(t))
    }

    pub fn last_true<T, F>(l: T, r: T, mut predicate: F) -> T
    where
        T: BinarySearchRangeType,
        F: FnMut(T) -> bool,
    {
        last_false(l, r, |t| !predicate(t))
    }

    pub fn first_true_floating_with_epsilon<F>(
        mut l: f64,
        mut r: f64,
        epsilon: f64,
        mut predicate: F,
    ) -> f64
    where
        F: FnMut(f64) -> bool,
    {
        while r - l > epsilon {
            let m = l.midpoint(r);
            if predicate(m) {
                r = m;
            } else {
                l = m;
            }
        }
        r
    }

    pub fn first_false_floating_with_epsilon<F>(
        l: f64,
        r: f64,
        epsilon: f64,
        mut predicate: F,
    ) -> f64
    where
        F: FnMut(f64) -> bool,
    {
        first_true_floating_with_epsilon(l, r, epsilon, |t| !predicate(t))
    }

    pub fn first_true_floating_with_iterations<F>(
        mut l: f64,
        mut r: f64,
        iterations: u32,
        mut predicate: F,
    ) -> f64
    where
        F: FnMut(f64) -> bool,
    {
        for _ in 0..iterations {
            let m = l.midpoint(r);
            if predicate(m) {
                r = m;
            } else {
                l = m;
            }
        }
        r
    }

    pub fn first_false_floating_with_iterations<F>(
        l: f64,
        r: f64,
        iterations: u32,
        mut predicate: F,
    ) -> f64
    where
        F: FnMut(f64) -> bool,
    {
        first_true_floating_with_iterations(l, r, iterations, |t| !predicate(t))
    }
}
