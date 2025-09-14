pub mod ds {
    pub struct DSU {
        nodes: Vec<i32>,
    }
    impl DSU {
        pub fn new(n: usize) -> Self {
            Self { nodes: vec![-1; n] }
        }

        pub fn parent(&mut self, a: usize) -> usize {
            if self.nodes[a] < 0 {
                return a;
            }
            self.nodes[a] = self.parent(self.nodes[a] as usize) as i32;
            self.nodes[a] as usize
        }

        pub fn size(&mut self, a: usize) -> i32 {
            let par = self.parent(a);
            -self.nodes[par]
        }

        pub fn merge(&mut self, a: usize, b: usize) -> bool {
            let mut a = self.parent(a);
            let mut b = self.parent(b);
            if a == b {
                return false;
            }
            if self.size(a) < self.size(b) {
                std::mem::swap(&mut a, &mut b);
            }

            self.nodes[a] += self.nodes[b];
            self.nodes[b] = a as i32;
            true
        }
    }
}
use ds::DSU;
