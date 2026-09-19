/// # Dynamic Connectivity ([source](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/DataStructures/DSU/dynamic_connectivity.rs))
///
/// You'll need the modules [dsu.rs](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/DataStructures/DSU/dsu.rs) and [my_utils.rs](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/CombinedRustModules/my_utils.rs) to use the same.
pub mod dynamic_connectivity {
    use super::{DSURollback, get_inclusive_usize_bounds};
    use std::ops::RangeBounds;

    pub struct DynamicConnectivity {
        nodes: usize,
        checkpoints: usize,
        capacity: usize,
        tree: Vec<Vec<(u32, u32)>>,
    }

    impl DynamicConnectivity {
        pub fn new(nodes: usize, checkpoints: usize) -> Self {
            let capacity = checkpoints.next_power_of_two();
            Self {
                nodes,
                checkpoints,
                capacity,
                tree: vec![vec![]; capacity + checkpoints],
            }
        }

        pub fn add_line<R>(&mut self, u: usize, v: usize, time: R)
        where
            R: RangeBounds<usize>,
        {
            let (l, r) = get_inclusive_usize_bounds(&time, self.checkpoints);
            self._add_line(1, 0, self.capacity - 1, l, r, &(u as u32, v as u32));
        }

        fn _add_line(
            &mut self,
            v: usize,
            l: usize,
            r: usize,
            ql: usize,
            qr: usize,
            edge: &(u32, u32),
        ) {
            if qr < l || ql > r {
                return;
            }
            if ql <= l && r <= qr {
                self.tree[v].push(edge.clone());
                return;
            }

            let m = (l + r) >> 1;
            self._add_line(v << 1, l, m, ql, qr, edge);
            self._add_line(v << 1 | 1, m + 1, r, ql, qr, edge);
        }

        // Will think of making this generic to various types of queries (if face problems of that type).
        pub fn process_and_return_component_counts(&self) -> Vec<usize> {
            let mut dsu_rollback = DSURollback::new(self.nodes);
            let mut ans = vec![0; self.checkpoints];
            self._process_for_component_counts(1, &mut dsu_rollback, &mut ans);
            ans
        }

        fn _process_for_component_counts(
            &self,
            u: usize,
            dsu_rollback: &mut DSURollback,
            ans: &mut [usize],
        ) {
            if u >= self.tree.len() {
                return;
            }

            dsu_rollback.create_checkpoint();
            for &(u, v) in &self.tree[u] {
                dsu_rollback.merge(u as usize, v as usize);
            }

            if u >= self.capacity {
                let i = u - self.capacity;
                ans[i] = dsu_rollback.number_of_components();
            } else {
                self._process_for_component_counts(u << 1, dsu_rollback, ans);
                self._process_for_component_counts(u << 1 | 1, dsu_rollback, ans);
            }

            assert!(dsu_rollback.rollback_to_last_checkpoint());
        }
    }
}
use dynamic_connectivity::DynamicConnectivity;
