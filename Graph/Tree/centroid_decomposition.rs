/// Centroid Decomposition
///
/// You'll need the module [tree.rs](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/Graph/Tree/tree.rs)) to use the same.
///
/// Call the Centroid Decomposition on a tree with the adjacency list initialized.
pub mod centroid_decomposition {
    use super::Tree;

    pub struct CentroidTree {
        pub n: usize,
        pub root: usize,
        // pub children: Vec<Vec<u32>>,
        // /// parent\[[root]] = root
        // pub parent: Vec<u32>,
        // /// depth\[[root]] = 0
        // pub depth: Vec<u32>,
        // pub size: Vec<u32>,
    }

    impl CentroidTree {
        pub fn from_tree(tree: &Tree) -> Self {
            let mut centroid_tree = Self {
                n: tree.n,
                root: 0, // Any value
                         // children: vec![vec![]; tree.n],
                         // parent: vec![0; tree.n],
                         // depth: vec![0; tree.n],
                         // size: vec![0; tree.n],
            };

            let mut centroid_taken = vec![false; tree.n];
            centroid_tree.root = centroid_tree.build(0, &mut centroid_taken, tree /*, 0*/);
            // centroid_tree.parent[centroid_tree.root] = centroid_tree.root as u32;
            centroid_tree
        }

        fn build(
            &mut self,
            u: usize,
            centroid_taken: &mut Vec<bool>,
            tree: &Tree,
            // root_depth: u32,
        ) -> usize {
            let root = self.find_centroid(u, centroid_taken, tree);
            centroid_taken[root] = true;
            // self.depth[root] = root_depth;
            // self.children.reserve(tree.adj[root].len());
            for v in &tree.adj[root] {
                let v = *v as usize;
                if centroid_taken[v] {
                    continue;
                }

                let v_root = self.build(v, centroid_taken, tree /*, root_depth + 1*/);
                // self.parent[v_root] = root as u32;
                // self.children[root].push(v_root as u32);
            }
            root
        }

        fn find_centroid(
            &mut self,
            u: usize,
            centroid_taken: &mut Vec<bool>,
            tree: &Tree,
        ) -> usize {
            fn dfs1(
                u: usize,
                par: Option<usize>,
                centroid_taken: &Vec<bool>,
                size: &mut Vec<u32>,
                tree: &Tree,
            ) {
                size[u] = 1;
                for v in &tree.adj[u] {
                    let v = *v as usize;
                    if Some(v) == par || centroid_taken[v] {
                        continue;
                    }

                    dfs1(v, Some(u), centroid_taken, size, tree);
                    size[u] += size[v];
                }
            }
            dfs1(u, None, &*centroid_taken, &mut self.size, tree);

            fn dfs2(
                u: usize,
                par: Option<usize>,
                centroid_taken: &Vec<bool>,
                size: &Vec<u32>,
                tree: &Tree,
                root: usize,
            ) -> usize {
                for v in &tree.adj[u] {
                    let v = *v as usize;
                    if Some(v) == par || centroid_taken[v] {
                        continue;
                    }

                    if size[v] > size[root] >> 1 {
                        return dfs2(v, Some(u), centroid_taken, size, tree, root);
                    }
                }
                u
            }
            dfs2(u, None, &*centroid_taken, &self.size, tree, u)
        }
    }
}
use centroid_decomposition::*;
