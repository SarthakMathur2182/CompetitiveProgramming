/// # Tree ([source](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/Graph/tree.rs))
pub mod tree {
    pub struct Tree {
        pub n: usize,
        pub adj: Vec<Vec<u32>>,
        // /// Take care of the parent of the root.
        // pub parent: Vec<u32>,
        // /// Depth of root is considered zero.
        // pub depth: Vec<u32>,
        // pub size: Vec<u32>,
        // pub in_time: Vec<usize>,
        // pub out_time: Vec<usize>,
        // pub tour: Vec<u32>,
    }

    impl Tree {
        pub fn new(n: usize) -> Self {
            Self {
                n,
                adj: vec![vec![]; n],
                // parent: vec![0; n],
                // depth: vec![0; n],
                // size: vec![0; n],
                // in_time: vec![0; n],
                // out_time: vec![0; n],
                // tour: Vec::with_capacity(n << 1),
            }
        }

        pub fn add_edge(&mut self, u: usize, v: usize) {
            self.adj[u].push(v as u32);
            // self.adj[v].push(u as u32);
        }

        pub fn dfs(&mut self, u: usize, par: Option<usize>) {
            // self.in_time[u] = self.tour.len();
            // self.tour.push(u as u32);
            // self.size[u] = 1;

            // Removing the parent from the adjacency list
            // if let Some(p) = par {
            //     if let Some(i) = self.adj[u].iter().position(|x| *x == p as u32) {
            //         self.adj[u].swap_remove(i);
            //     }
            // }
            for i in 0..self.adj[u].len() {
                let v = self.adj[u][i] as usize;
                if Some(v) == par {
                    continue;
                }

                // self.parent[v] = u as u32;
                // self.depth[v] = self.depth[u] + 1;
                self.dfs(v, Some(u));
                // self.size[u] += self.size[v];
                // self.tour.push(u as u32);
            }
            // self.out_time[u] = self.tour.len();
            // self.tour.push(u as u32);
        }
    }
}
use tree::*;
