pub mod graph {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub enum NodeVisitStatus {
        NotVisited,
        Visiting,
        Visited,
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub enum EdgeType {
        Unknown,
        TreeEdge,
        BackEdge,
        ForwardEdge,
        CrossEdge,
    }

    #[derive(Clone, Eq, PartialEq)]
    pub struct Edge<T: Clone + Eq + PartialEq + Default> {
        pub u: u32,
        pub v: u32,
        pub w: T,
    }

    impl<T: Clone + Eq + PartialEq + Default> Edge<T> {
        pub fn new(u: u32, v: u32, w: T) -> Self {
            Self { u, v, w }
        }
    }

    pub struct Graph<T: Clone + Eq + PartialEq + Default, const DIRECTED: bool> {
        pub n: usize,
        pub adj: Vec<Vec<u32>>,
        pub edges: Vec<Edge<T>>,
        pub node_visit_status: Vec<NodeVisitStatus>,
        pub edge_type: Vec<EdgeType>,
        // /// Take care of the parent of the root.
        // pub parent: Vec<u32>,
    }

    impl<T: Clone + Eq + PartialEq + Default, const DIRECTED: bool> Graph<T, DIRECTED> {
        pub fn new(n: usize, m: usize) -> Self {
            Self {
                n,
                adj: vec![vec![]; n],
                edges: Vec::with_capacity(m),
                node_visit_status: vec![NodeVisitStatus::NotVisited; n],
                edge_type: Vec::with_capacity(m),
                // parent: vec![0; n],
            }
        }

        pub fn add_edge(&mut self, u: usize, v: usize, w: T) {
            self.adj[u].push(self.edges.len() as u32);
            if !DIRECTED {
                self.adj[v].push(self.edges.len() as u32);
            }
            self.edges.push(Edge::new(u as u32, v as u32, w));
            self.edge_type.push(EdgeType::Unknown);
        }

        pub fn dfs(&mut self, u: usize, par: Option<usize>) {
            assert!(self.node_visit_status[u] == NodeVisitStatus::NotVisited);
            self.node_visit_status[u] = NodeVisitStatus::Visiting;
            for i in 0..self.adj[u].len() {
                let i = self.adj[u][i] as usize;
                if !DIRECTED && self.edge_type[i] != EdgeType::Unknown {
                    continue;
                }
                assert!(self.edge_type[i] == EdgeType::Unknown);

                let v = (self.edges[i].u ^ self.edges[i].v) as usize ^ u;
                match self.node_visit_status[v] {
                    NodeVisitStatus::NotVisited => {
                        // Process tree edge before traversal

                        self.edge_type[i] = EdgeType::TreeEdge;
                        // self.parent[v] = u as u32;
                        self.dfs(v, Some(u));

                        // Process bottom up operations (related to backing from v to u)
                    }
                    NodeVisitStatus::Visiting => {
                        self.edge_type[i] = EdgeType::BackEdge;
                        // Process back edge
                    }
                    NodeVisitStatus::Visited => {
                        // TODO: Process Forward edge and Cross edge for directed graph.
                        // assert!(false); // In case of undirected graph
                    }
                }
            }
            // Post-order process u
            self.node_visit_status[u] = NodeVisitStatus::Visited;
        }
    }

    pub struct Tree {
        pub n: usize,
        pub adj: Vec<Vec<u32>>,
        // /// Take care of the parent of the root.
        // pub parent: Vec<u32>,
        // /// Depth of root is considered zero.
        // pub depth: Vec<u32>,
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
            for i in 0..self.adj[u].len() {
                let v = self.adj[u][i] as usize;
                if Some(v) == par {
                    continue;
                }

                // self.parent[v] = u as u32;
                // self.depth[v] = self.depth[u] + 1;
                self.dfs(v, Some(u));
                // self.tour.push(u as u32);
            }
            // self.out_time[u] = self.tour.len();
            // self.tour.push(u as u32);
        }
    }
}
use graph::*;
