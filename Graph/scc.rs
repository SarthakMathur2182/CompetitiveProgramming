/// # Strongly Connected Components ([source](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/Graph/scc.rs))
///
/// You'll need the module [graph.rs](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/Graph/graph.rs) to use the same.
pub mod scc {
    use super::graph::*;

    pub struct SCC<T: Clone + Eq + PartialEq + Default> {
        pub n: usize,
        pub components: Vec<Vec<u32>>,
        /// The index of the component the node is part of
        pub group: Vec<u32>,
        // /// Might contain parallel edges
        // pub condensed_graph: Graph<T, true>,
    }

    impl<T: Clone + Eq + PartialEq + Default> SCC<T> {
        pub fn from_graph(g: &Graph<T, true>) -> Self {
            let mut adj = vec![vec![]; g.n];
            let mut rev_adj = vec![vec![]; g.n];
            for u in 0..g.n {
                for i in &g.adj[u] {
                    let i = *i as usize;
                    let v = (g.edges[i].u ^ g.edges[i].v ^ u as u32) as usize;
                    adj[u].push(v);
                    rev_adj[v].push(u);
                }
            }

            let mut scc = Self {
                n: g.n,
                components: Vec::new(),
                group: vec![0; g.n],
                // condensed_graph: Graph::new(g.n, g.edges.len()),
            };
            scc.init(adj, rev_adj);

            // for e in &g.edges {
            //     let (u, v) = (scc.group[e.u as usize], scc.group[e.v as usize]);
            //     if u != v {
            //         scc.condensed_graph
            //             .add_edge(u as usize, v as usize, e.w.clone());
            //     }
            // }
            scc
        }

        pub fn init(&mut self, adj: Vec<Vec<usize>>, rev_adj: Vec<Vec<usize>>) {
            // The nodes in order of out time
            let mut nodes = Vec::with_capacity(self.n);
            let mut vis = vec![false; self.n];
            for u in 0..self.n {
                if !vis[u] {
                    Self::dfs(u, &mut vis, &mut nodes, &adj);
                }
            }

            vis.fill(false);
            for u in nodes.into_iter().rev() {
                if !vis[u] {
                    self.components.push(Vec::new());
                    self.dfs2(u, self.components.len() as u32 - 1, &mut vis, &rev_adj);
                }
            }
        }

        fn dfs(u: usize, vis: &mut Vec<bool>, nodes: &mut Vec<usize>, adj: &Vec<Vec<usize>>) {
            vis[u] = true;
            for &v in &adj[u] {
                if !vis[v] {
                    Self::dfs(v, vis, nodes, adj);
                }
            }
            nodes.push(u);
        }

        fn dfs2(&mut self, u: usize, group_idx: u32, vis: &mut Vec<bool>, adj: &Vec<Vec<usize>>) {
            vis[u] = true;
            self.group[u] = group_idx;
            self.components.last_mut().unwrap().push(u as u32);
            for &v in &adj[u] {
                if !vis[v] {
                    self.dfs2(v, group_idx, vis, adj);
                }
            }
        }
    }
}
use scc::*;
