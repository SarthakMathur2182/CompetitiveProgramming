/// # Graph ([source](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/Graph/graph.rs))
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
}
use graph::*;
