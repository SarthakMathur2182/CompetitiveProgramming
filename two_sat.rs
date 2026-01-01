pub mod two_sat {
    use super::graph::*;
    use super::scc::*;

    pub struct TwoSat {
        /// The number of variables
        pub n: usize,
        pub graph: Graph<(), true>,
        pub satisfiable: Option<bool>,
        pub true_value: Vec<bool>,
    }

    impl TwoSat {
        pub fn new(n: usize, m: usize) -> Self {
            Self {
                n,
                graph: Graph::new(n << 1, m << 1),
                satisfiable: None,
                true_value: vec![false; n],
            }
        }

        pub fn add_clause(&mut self, a: usize, a_negated: bool, b: usize, b_negated: bool) {
            let u = a << 1 | (a_negated as usize);
            let v = b << 1 | (b_negated as usize);
            self.graph.add_edge(u ^ 1, v, ());
            self.graph.add_edge(v ^ 1, u, ());
        }

        pub fn init(&mut self) {
            let scc_graph = SCC::from_graph(&self.graph);

            let mut satisfiable = true;
            for u in 0..self.n {
                if scc_graph.group[u << 1] == scc_graph.group[u << 1 | 1] {
                    satisfiable = false;
                    break;
                }
                self.true_value[u] = scc_graph.group[u << 1] > scc_graph.group[u << 1 | 1];
            }
            self.satisfiable = Some(satisfiable);
        }
    }
}
use two_sat::*;
