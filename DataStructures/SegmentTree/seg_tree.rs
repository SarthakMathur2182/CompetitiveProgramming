/// # Segment Tree ([source](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/SegmentTree/seg_tree.rs))
///
/// Includes lazy propagation
///
/// You'll need the module [my_utils.rs](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/CombinedRustModules/my_utils.rs) to use the same.
pub mod seg_tree {
    use std::fmt::{Debug, Formatter};
    use std::ops::RangeBounds;

    pub trait SegmentTreeOperations {
        type Data: Clone;

        fn data_identity() -> Self::Data;

        /// The binary operator which we'll apply on the tree.
        fn merge(a: &Self::Data, b: &Self::Data) -> Self::Data;
    }

    pub struct SegmentTree<Ops: SegmentTreeOperations> {
        /// The number of elements passed during construction.
        pub n: usize,

        /// The number of leaf nodes we're considering in the construction.
        ///
        /// Equal to `n`'s ceiling power of 2.
        capacity: usize,

        /// All the data nodes.
        ///
        /// Its length is twice of `capacity`, and the root is index `1`.
        nodes: Vec<Ops::Data>,
    }

    impl<Ops: SegmentTreeOperations> SegmentTree<Ops> {
        /// Initialize the nodes equal to `nodes`
        pub fn with_nodes(nodes: &Vec<Ops::Data>) -> Self {
            Self::with_func(nodes.len(), |i| nodes[i].clone())
        }

        /// Initialize the nodes with `Ops::data_identity()`
        pub fn with_defaults(n: usize) -> Self {
            let capacity = n.next_power_of_two();
            let nodes = vec![Ops::data_identity(); capacity << 1];
            Self { n, capacity, nodes }
        }

        /// Initialize the nodes according to the function/lambda `f`, taking the index and returning the data node.
        pub fn with_func<F>(n: usize, f: F) -> Self
        where
            F: Fn(usize) -> Ops::Data,
        {
            let mut seg_tree = Self::with_defaults(n);
            for i in 0..n {
                seg_tree.nodes[seg_tree.capacity + i] = f(i);
            }
            for i in (1..seg_tree.capacity).rev() {
                seg_tree.nodes[i] =
                    Ops::merge(&seg_tree.nodes[i << 1], &seg_tree.nodes[i << 1 | 1]);
            }

            seg_tree
        }

        /// Range query over `range`.
        pub fn query<R>(&self, range: R) -> Ops::Data
        where
            R: RangeBounds<usize>,
        {
            let (mut l, mut r) = super::get_inclusive_usize_bounds(&range, self.capacity);
            l += self.capacity;
            r += self.capacity;
            let mut ans = Ops::data_identity();
            while l <= r {
                if l & 1 == 1 {
                    ans = Ops::merge(&self.nodes[l], &ans);
                    l += 1;
                }
                if r & 1 == 0 {
                    ans = Ops::merge(&ans, &self.nodes[r]);
                    r -= 1;
                }
                l >>= 1;
                r >>= 1;
            }
            ans
        }

        /// Query without computing/merging nodes, and allows to use the nodes in the `access_node` function.
        ///
        /// Probably need a better name.
        ///
        /// TODO: Try to create an iterator over nodes accessing l..=r
        pub fn query2<R, F>(&self, range: R, mut access_node: F)
        where
            R: RangeBounds<usize>,
            F: FnMut(&Ops::Data),
        {
            let (l, r) = super::get_inclusive_usize_bounds(&range, self.capacity);
            self._query2(1, 0, self.capacity - 1, l, r, &mut access_node);
        }

        fn _query2<F>(
            &self,
            v: usize,
            l: usize,
            r: usize,
            ql: usize,
            qr: usize,
            access_node: &mut F,
        ) where
            F: FnMut(&Ops::Data),
        {
            if ql > r || qr < l {
                return;
            }
            if ql <= l && r <= qr {
                access_node(&self.nodes[v]);
                return;
            }

            let m = (l + r) >> 1;
            self._query2(v << 1, l, m, ql, qr, access_node);
            self._query2(v << 1 | 1, m + 1, r, ql, qr, access_node);
        }

        /// Point-update in the Segment Tree. The function `updateTo` allows multiple types of updates.
        ///
        /// If we want to update the value to `x`, `updateTo = |c| x`
        ///
        /// If we want to increase the current value by `x`, `updateTo = |c| c + x`
        pub fn update<F>(&mut self, mut pos: usize, update_to: F)
        where
            F: Fn(Ops::Data) -> Ops::Data,
        {
            pos += self.capacity;
            self.nodes[pos] = update_to(self.nodes[pos].clone());
            while pos > 1 {
                self.nodes[pos >> 1] = Ops::merge(&self.nodes[pos], &self.nodes[pos ^ 1]);
                pos >>= 1;
            }
        }
    }

    impl<Ops: SegmentTreeOperations> Debug for SegmentTree<Ops>
    where
        Ops::Data: Debug,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            for i in 1..self.capacity + self.n {
                writeln!(f)?;
                write!(f, "{}: {:?}", i, self.nodes[i])?;
            }
            Ok(())
        }
    }

    pub trait LazySegmentTreeOperations: SegmentTreeOperations {
        type Lazy: Clone;

        fn lazy_identity() -> Self::Lazy;

        /// Apply the lazy update on the data.
        ///
        /// I pass the range `l` and `r` as they are often used in the updates.
        fn apply(data: &Self::Data, update: &Self::Lazy, l: usize, r: usize) -> Self::Data;

        /// Update the lazy operation.
        fn compose(prev: &Self::Lazy, next: &Self::Lazy) -> Self::Lazy;
    }

    /// Make sure that `Ops` implement both `SegmentTreeOperations` and `LazySegmentTreeOperations`
    pub struct LazySegmentTree<Ops: LazySegmentTreeOperations> {
        /// The number of elements passed during construction.
        pub n: usize,

        /// The number of leaf nodes we're considering in the construction.
        ///
        /// Equal to `n`'s ceiling power of 2.
        capacity: usize,

        /// All the data nodes.
        ///
        /// Its length is twice of `capacity`, and the root is index `1`.
        data_nodes: Vec<Ops::Data>,

        /// All the lazy nodes.
        ///
        /// Its length is twice of `capacity`, and the root is index `1`.
        lazy_nodes: Vec<Ops::Lazy>,
    }

    impl<Ops: LazySegmentTreeOperations> LazySegmentTree<Ops> {
        /// Initialize the data nodes equal to `nodes`
        pub fn with_nodes(nodes: Vec<Ops::Data>) -> Self {
            Self::with_func(nodes.len(), |i| nodes[i].clone())
        }

        /// Initialize the nodes with `Ops::data_identity()` and `Ops::lazy_identity()`
        pub fn with_defaults(n: usize) -> Self {
            let capacity = n.next_power_of_two();
            let data_nodes = vec![Ops::data_identity(); capacity << 1];
            let lazy_nodes = vec![Ops::lazy_identity(); capacity << 1];
            Self {
                n,
                capacity,
                data_nodes,
                lazy_nodes,
            }
        }

        /// Initialize the data nodes according to the function/lambda `f`, taking the index and returning the data node.
        pub fn with_func<F>(n: usize, f: F) -> Self
        where
            F: Fn(usize) -> Ops::Data,
        {
            let mut seg_tree = Self::with_defaults(n);
            for i in 0..n {
                seg_tree.data_nodes[seg_tree.capacity + i] = f(i);
            }
            for i in (1..seg_tree.capacity).rev() {
                seg_tree.data_nodes[i] = Ops::merge(
                    &seg_tree.data_nodes[i << 1],
                    &seg_tree.data_nodes[i << 1 | 1],
                );
            }

            seg_tree
        }

        fn apply_at(&mut self, v: usize, l: usize, r: usize, update_to: &Ops::Lazy) {
            self.data_nodes[v] = Ops::apply(&self.data_nodes[v], update_to, l, r);
            self.lazy_nodes[v] = Ops::compose(&self.lazy_nodes[v], update_to);
        }

        fn push(&mut self, v: usize, l: usize, r: usize) {
            let m = (l + r) >> 1;
            let update_to = self.lazy_nodes[v].clone();
            self.apply_at(v << 1, l, m, &update_to);
            self.apply_at(v << 1 | 1, m + 1, r, &update_to);
            self.lazy_nodes[v] = Ops::lazy_identity();
        }

        /// Range query over `range`
        pub fn query<R>(&mut self, range: R) -> Ops::Data
        where
            R: RangeBounds<usize>,
        {
            let (l, r) = super::get_inclusive_usize_bounds(&range, self.capacity);
            self._query(1, 0, self.capacity - 1, l, r)
        }

        fn _query(&mut self, v: usize, l: usize, r: usize, ql: usize, qr: usize) -> Ops::Data {
            if ql > r || qr < l {
                return Ops::data_identity();
            }
            if ql <= l && r <= qr {
                return self.data_nodes[v].clone();
            }

            self.push(v, l, r);
            let m = (l + r) >> 1;
            Ops::merge(
                &self._query(v << 1, l, m, ql, qr),
                &self._query(v << 1 | 1, m + 1, r, ql, qr),
            )
        }

        /// Query without computing/merging nodes, and allows to use the nodes in the `access_node` function.
        ///
        /// Probably need a better name.
        ///
        /// TODO: Try to create an iterator over nodes accessing l..=r
        pub fn query2<R, F>(&mut self, range: R, mut access_node: F)
        where
            R: RangeBounds<usize>,
            F: FnMut(&Ops::Data),
        {
            let (l, r) = super::get_inclusive_usize_bounds(&range, self.capacity);
            self._query2(1, 0, self.capacity - 1, l, r, &mut access_node);
        }

        fn _query2<F>(
            &mut self,
            v: usize,
            l: usize,
            r: usize,
            ql: usize,
            qr: usize,
            access_node: &mut F,
        ) where
            F: FnMut(&Ops::Data),
        {
            if ql > r || qr < l {
                return;
            }
            if ql <= l && r <= qr {
                access_node(&self.data_nodes[v]);
                return;
            }

            self.push(v, l, r);
            let m = (l + r) >> 1;
            self._query2(v << 1, l, m, ql, qr, access_node);
            self._query2(v << 1 | 1, m + 1, r, ql, qr, access_node);
        }

        /// Range update according to the lazy value `update_to`.
        pub fn update<R>(&mut self, range: R, update_to: Ops::Lazy)
        where
            R: RangeBounds<usize>,
        {
            let (l, r) = super::get_inclusive_usize_bounds(&range, self.capacity);
            self._update(1, 0, self.capacity - 1, l, r, &update_to);
        }

        fn _update(
            &mut self,
            v: usize,
            l: usize,
            r: usize,
            ql: usize,
            qr: usize,
            update_to: &Ops::Lazy,
        ) {
            if qr < l || ql > r {
                return;
            }
            if ql <= l && r <= qr {
                self.apply_at(v, l, r, update_to);
                return;
            }

            self.push(v, l, r);
            let m = (l + r) >> 1;
            self._update(v << 1, l, m, ql, qr, update_to);
            self._update(v << 1 | 1, m + 1, r, ql, qr, update_to);
            self.data_nodes[v] = Ops::merge(&self.data_nodes[v << 1], &self.data_nodes[v << 1 | 1]);
        }
    }

    impl<Ops: LazySegmentTreeOperations> Debug for LazySegmentTree<Ops>
    where
        Ops::Data: Debug,
        Ops::Lazy: Debug,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            for i in 1..self.capacity + self.n {
                writeln!(f)?;
                write!(
                    f,
                    "{}: Data = {:?}, Lazy = {:?}",
                    i, self.data_nodes[i], self.lazy_nodes[i]
                )?;
            }
            Ok(())
        }
    }
}
use seg_tree::*;
