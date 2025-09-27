/// # Segment Tree ([source](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/SegmentTree/seg_tree.rs))
///
/// You'll need the module [my_utils.rs](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/CombinedRustModules/my_utils.rs) to use the same.
///
/// Will add lazy propagation later.
pub mod seg_tree {
    use std::collections::Bound;
    use std::ops::RangeBounds;

    pub trait SegmentTreeNode: Copy + Clone {
        const DATA_IDENTITY: Self;

        fn merge(a: &Self, b: &Self) -> Self;
    }

    pub struct SegmentTree<Node: SegmentTreeNode> {
        /// The number of elements passed during construction.
        pub n: usize,

        /// The number of leaf nodes we're considering in the construction.
        ///
        /// Equal to `n`'s ceiling power of 2.
        capacity: usize,

        /// All the nodes.
        ///
        /// Its length is twice of `capacity`, and the root is index `1`.
        nodes: Vec<Node>,
    }

    impl<Node: SegmentTreeNode> SegmentTree<Node> {
        /// Initialize the nodes equal to `nodes`
        pub fn with_nodes(nodes: Vec<Node>) -> Self {
            Self::with_func(nodes.len(), |i| nodes[i])
        }

        /// Initialize the nodes with `Node::DATA_IDENTITY`
        pub fn with_defaults(n: usize) -> Self {
            let capacity = n.next_power_of_two();
            let nodes = vec![Node::DATA_IDENTITY; capacity << 1];
            Self { n, capacity, nodes }
        }

        /// Initialize the nodes according to the function/lambda `f`, taking the index and returning the nodes.
        pub fn with_func<F>(n: usize, f: F) -> Self
        where
            F: Fn(usize) -> Node,
        {
            let mut seg_tree = Self::with_defaults(n);
            for i in 0..n {
                seg_tree.nodes[seg_tree.capacity + i] = f(i);
            }
            for i in (1..seg_tree.capacity).rev() {
                seg_tree.nodes[i] =
                    Node::merge(&seg_tree.nodes[i << 1], &seg_tree.nodes[i << 1 | 1]);
            }

            seg_tree
        }

        pub fn query<R>(&self, range: R) -> Node
        where
            R: RangeBounds<usize>,
        {
            let (l, r) = super::get_inclusive_usize_bounds(&range, self.capacity);

            self._query(1, 0, self.capacity - 1, l, r)
        }

        fn _query(&self, v: usize, l: usize, r: usize, ql: usize, qr: usize) -> Node {
            if ql > r || qr < l {
                return Node::DATA_IDENTITY;
            }
            if l <= ql && qr <= r {
                return self.nodes[v];
            }

            let m = ((l ^ r) >> 1) + (l & r);
            Node::merge(
                &self._query(v << 1, l, m, ql, qr),
                &self._query(v << 1 | 1, m + 1, r, ql, qr),
            )
        }

        /// Point-update in the Segment Tree. The function `updateTo` allows multiple types of updates.
        ///
        /// If we want to update the value to `x`, `updateTo = c -> x`
        ///
        /// If we want to increase the current value by `x`, `updateTo = c -> c + x`
        pub fn update<F>(&mut self, pos: usize, update_to: F)
        where
            F: Fn(Node) -> Node,
        {
            self._update(1, 0, self.capacity - 1, pos, &update_to);
        }

        fn _update<F>(&mut self, v: usize, l: usize, r: usize, p: usize, update_to: &F)
        where
            F: Fn(Node) -> Node,
        {
            if p < l || p > r {
                return;
            }
            if l == r {
                self.nodes[v] = update_to(self.nodes[v]);
                return;
            }

            let m = ((l ^ r) >> 1) + (l & r);
            self._update(v << 1, l, m, p, update_to);
            self._update(v << 1 | 1, m + 1, r, p, update_to);
            self.nodes[v] = Node::merge(&self.nodes[v << 1], &self.nodes[v << 1 | 1]);
        }
    }
}
use seg_tree::*;
