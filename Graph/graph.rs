pub mod graph {
    #[derive(Debug)]
    enum NodeVisitStatus {
        NotVisited,
        Visiting,
        Visited,
    }

    pub struct Graph {
        pub n: usize,
        pub vis: Vec<NodeVisitStatus>,
        pub adj: Vec<Vec<u32>>,
    }

    impl Graph {
        pub fn new(n: usize) -> Self {
            Self {
                n,
                vis: vec![NodeVisitStatus::NotVisited; n],
                adj: vec![vec![]; n],
            }
        }

        pub fn add_edge(&mut self, u: usize, v: usize) {
            self.adj[u].push(v as u32);
            // self.adj[v].push(u as u32);
        }

        pub fn dfs(&mut self, u: usize) {
            if self.vis[u] == NodeVisitStatus::Visited {
                return;
            }
            assert!(self.vis[u] != NodeVisitStatus::Visiting);
            self.vis[u] = NodeVisitStatus::Visiting;
            // Pre order process u
            for i in 0..self.adj[u].len() {
                let v = self.adj[u][i] as usize;
                match self.vis[v] {
                    NodeVisitStatus::NotVisited => {
                        // Process tree edge before traversal
                        self.dfs(v);
                        // Process tree edge after traversal
                    }
                    NodeVisitStatus::Visiting => {
                        // Process back edge
                    }
                    NodeVisitStatus::Visited => {
                        // Process cross edge
                    }
                }
            }
            // Post order process u
        }
    }

    pub struct Tree {
        pub n: usize,
        pub adj: Vec<Vec<u32>>,
    }

    impl Tree {
        pub fn new(n: usize) -> Self {
            Self {
                n,
                adj: vec![vec![]; n],
            }
        }

        pub fn add_edge(&mut self, u: usize, v: usize) {
            self.adj[u].push(v as u32);
            // self.adj[v].push(u as u32);
        }

        pub fn dfs(&mut self, u: usize, par: Option<usize>) {
            for i in 0..self.adj[u].len() {
                let v = self.adj[u][i] as usize;
                if Some(v) == par {
                    continue;
                }
                self.dfs(v, Some(u));
            }
        }
    }
}
use graph::*;
