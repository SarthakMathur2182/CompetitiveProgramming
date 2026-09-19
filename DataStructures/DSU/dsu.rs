/// # DSU ([source](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/DataStructures/DSU/dsu.rs))
pub mod dsu {
    use std::collections::BTreeMap;

    // TODO: Weighted

    pub struct DSU {
        number_of_components: u32,
        nodes: Vec<i32>,
    }

    impl DSU {
        pub fn new(n: usize) -> Self {
            Self {
                number_of_components: n as u32,
                nodes: vec![-1; n],
            }
        }

        pub fn parent(&mut self, a: usize) -> usize {
            if self.nodes[a] < 0 {
                return a;
            }
            self.nodes[a] = self.parent(self.nodes[a] as usize) as i32;
            self.nodes[a] as usize
        }

        pub fn size(&mut self, a: usize) -> usize {
            let par = self.parent(a);
            -self.nodes[par] as usize
        }

        pub fn merge(&mut self, mut a: usize, mut b: usize) -> bool {
            a = self.parent(a);
            b = self.parent(b);
            if a == b {
                return false;
            }
            if self.size(a) < self.size(b) {
                std::mem::swap(&mut a, &mut b);
            }

            self.number_of_components -= 1;
            self.nodes[a] += self.nodes[b];
            self.nodes[b] = a as i32;
            true
        }

        pub fn number_of_components(&self) -> usize {
            self.number_of_components as usize
        }

        pub fn get_components(&mut self) -> BTreeMap<usize, Vec<usize>> {
            let mut map: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
            for i in 0..self.nodes.len() {
                map.entry(self.parent(i)).or_default().push(i);
            }

            map
        }
    }

    /// As a general pattern, the functions returning true/false tells if the operation was done.
    /// Merge will return false if the nodes were already connected.
    /// Rollback will return false if there is nothing to rollback. So you can add assertions accordingly.
    pub struct DSURollback {
        nodes: Vec<i32>,
        history: Vec<(u32, u32, i32)>,
        checkpoints: Vec<u32>,
    }

    impl DSURollback {
        pub fn new(n: usize) -> Self {
            Self {
                nodes: vec![-1; n],
                history: Vec::new(),
                checkpoints: Vec::new(),
            }
        }

        #[inline]
        pub fn number_of_components(&self) -> usize {
            self.nodes.len() - self.history.len()
        }

        pub fn get_components(&mut self) -> BTreeMap<usize, Vec<usize>> {
            let mut map: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
            for i in 0..self.nodes.len() {
                map.entry(self.parent(i)).or_default().push(i);
            }

            map
        }

        pub fn parent(&self, mut a: usize) -> usize {
            while self.nodes[a] >= 0 {
                a = self.nodes[a] as usize;
            }
            a
        }

        pub fn size(&self, a: usize) -> usize {
            let par = self.parent(a);
            -self.nodes[par] as usize
        }

        pub fn merge(&mut self, mut a: usize, mut b: usize) -> bool {
            a = self.parent(a);
            b = self.parent(b);
            if a == b {
                return false;
            }
            if self.size(a) < self.size(b) {
                std::mem::swap(&mut a, &mut b);
            }

            self.history.push((a as u32, b as u32, self.nodes[b]));
            self.nodes[a] += self.nodes[b];
            self.nodes[b] = a as i32;
            true
        }

        fn rollback_once(&mut self) -> bool {
            if let Some((parent, child, child_size)) = self.history.pop() {
                self.nodes[parent as usize] -= child_size;
                self.nodes[child as usize] = child_size;
                true
            } else {
                false
            }
        }

        pub fn rollback(&mut self, count: usize) -> bool {
            for _ in 0..count {
                if !self.rollback_once() {
                    return false;
                }
            }
            true
        }

        pub fn create_checkpoint(&mut self) {
            self.checkpoints.push(self.history.len() as u32);
        }

        /// Don't use `rollback` along with this.
        pub fn rollback_to_last_checkpoint(&mut self) -> bool {
            if let Some(checkpoint) = self.checkpoints.pop() {
                let checkpoint = checkpoint as usize;
                assert!(
                    checkpoint <= self.history.len(),
                    "The last checkpoint timestamp is more than the number of changes we've stored!\
                    This mean either the code is wrong, or both `rollback` and `rollback_to_last_checkpoint`\
                    are used together (which is not safe in my opinion)!"
                );

                while self.history.len() > checkpoint {
                    assert!(self.rollback_once());
                }
                return true;
            } else {
                false
            }
        }
    }
}
use dsu::*;
