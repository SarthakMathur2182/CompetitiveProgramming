/// # Sieve of Eratosthenes ([source](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/Math/Sieve/sieve.rs))
pub mod sieve {
    use std::convert::TryInto;
    use std::fmt::Debug;

    pub struct Sieve<const N: usize> {
        is_prime: Vec<bool>,
        primes: Vec<u32>,
        spf: Vec<u32>,
    }

    impl<const N: usize> Sieve<N> {
        pub fn new() -> Self {
            let mut is_prime = vec![true; N + 1];
            let mut primes = Vec::with_capacity(N + 1);
            let mut spf: Vec<u32> = (0..N as u32 + 1).collect();
            is_prime[0] = false;
            is_prime[1] = false;
            for i in 2..=N {
                if !is_prime[i] {
                    continue;
                }
                primes.push(i as u32);
                for j in (i * i..=N).step_by(i) {
                    if is_prime[j] {
                        is_prime[j] = false;
                        spf[j] = i as u32;
                    }
                }
            }
            primes.shrink_to_fit();

            Self {
                is_prime,
                primes,
                spf,
            }
        }

        pub fn is_prime<T>(&self, n: T) -> bool
        where
            T: TryInto<usize>,
            <T as TryInto<usize>>::Error: Debug,
        {
            let n = n.try_into().unwrap();
            assert!(
                n <= N,
                "Number {} is too big for the precomputation of size {}. Use is_prime_big instead!",
                n,
                N
            );
            self.is_prime[n]
        }

        pub fn is_prime_big<T>(&self, n: T) -> bool
        where
            T: TryInto<u64>,
            <T as TryInto<u64>>::Error: Debug,
        {
            let n = n.try_into().unwrap();
            assert!(
                (N * N) as u64 >= n,
                "Number {} is too big to check! Maximum number allowed: {}",
                n,
                N * N
            );
            for &p in &self.primes {
                if n % p as u64 == 0 {
                    return false;
                }
            }
            true
        }

        pub fn smallest_prime_factor<T>(&self, n: T) -> u32
        where
            T: TryInto<usize>,
            <T as TryInto<usize>>::Error: Debug,
        {
            let n = n.try_into().unwrap();
            assert!(
                n <= N,
                "Number {} is too big for precomputation of size {}. Use smallest_prime_factor_big instead!",
                n,
                N
            );
            self.spf[n]
        }

        pub fn smallest_prime_factor_big<T>(&self, n: T) -> u64
        where
            T: TryInto<u64>,
            <T as TryInto<u64>>::Error: Debug,
        {
            let n = n.try_into().unwrap();
            assert!(
                (N * N) as u64 >= n,
                "Number {} is too big to check! Maximum number allowed: {}",
                n,
                N * N
            );
            for &p in &self.primes {
                let p = p as u64;
                if n % p == 0 {
                    return p;
                }
            }
            n
        }

        pub fn primes(&self) -> &Vec<u32> {
            &self.primes
        }
    }
}
use sieve::*;
