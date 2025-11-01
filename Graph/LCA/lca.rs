pub mod lca {
    use std::convert::TryInto;

    pub struct BinaryLifting {
        pub n: usize,
        /// `up[lg]` exists
        pub lg: usize,
        pub up: Vec<Vec<u32>>,
        /// TODO: Learn properly about lifetime and see if you can use a reference
        pub depth: Vec<u32>,
    }

    impl BinaryLifting {
        /// Assuming `par[root] = root`.
        ///
        /// The `depth` is required for lca.
        pub fn from_parent_and_depth<T: TryInto<u32> + Copy>(par: &[T], depth: &[T]) -> Self {
            let n = par.len();
            let lg = n.ilog2() as usize;
            let mut up = vec![vec![0; n]; lg + 1];
            for u in 0..n {
                unsafe {
                    up[0][u] = par[u].try_into().unwrap_unchecked();
                }
            }
            for l in 1..=lg {
                for u in 0..n {
                    up[l][u] = up[l - 1][up[l - 1][u] as usize];
                }
            }

            unsafe {
                Self {
                    n,
                    lg,
                    up,
                    depth: depth
                        .iter()
                        .map(|&x| x.try_into().unwrap_unchecked())
                        .collect(),
                }
            }
        }

        pub fn lift(&self, mut u: usize, k: u32) -> usize {
            for l in 0..=self.lg {
                if k >> l & 1 == 1 {
                    u = self.up[l][u] as usize;
                }
            }
            u
        }

        pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
            let d1 = self.depth[a];
            let d2 = self.depth[b];
            if d1 > d2 {
                a = self.lift(a, d1 - d2);
            } else if d1 < d2 {
                b = self.lift(b, d2 - d1);
            }

            if a == b {
                return a;
            }
            for l in (0..=self.lg).rev() {
                if self.up[l][a] != self.up[l][b] {
                    a = self.up[l][a] as usize;
                    b = self.up[l][b] as usize;
                }
            }
            self.up[0][a] as usize
        }

        pub fn dist(&self, a: usize, b: usize) -> u32 {
            let lca = self.lca(a, b);
            (self.depth[a] - self.depth[lca]) + (self.depth[b] - self.depth[lca])
        }
    }
}
use lca::*;
