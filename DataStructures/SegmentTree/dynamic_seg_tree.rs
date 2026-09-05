/// # Dynamic Segment Tree ([source](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/DataStructures/SegmentTree/dynamic_seg_tree.rs))
///
/// You'll need the modules [math.rs](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/CombinedRustModules/math.rs), [my_utils.rs](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/CombinedRustModules/my_utils.rs) and [seg_tree.rs](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/DataStructures/SegmentTree/seg_tree.rs) to use the same.
///
/// TODO: See if we can avoid 3 dependencies.
#[allow(private_bounds)]
pub mod dynamic_seg_tree {
    use super::{MultiplicativeIdentity, RoundedMidpoint, SegmentTreeOperations};
    use std::collections::Bound;
    use std::fmt::{Debug, Display, Formatter, write};
    use std::ops::{Add, RangeBounds, Sub};

    // TODO: Replace this once https://github.com/rust-lang/rust/issues/41517 is solved
    trait DynamicSegmentTreeBoundType:
        Copy
        + RoundedMidpoint
        + Ord
        + Display
        + Debug
        + Add<Output = Self>
        + Sub<Output = Self>
        + MultiplicativeIdentity
    {
    }

    impl<
        T: Copy
            + RoundedMidpoint
            + Ord
            + Display
            + Debug
            + Add<Output = Self>
            + Sub<Output = Self>
            + MultiplicativeIdentity,
    > DynamicSegmentTreeBoundType for T
    {
    }

    // TODO: Check if we should store the range handled by this node.
    // TODO: Change the type from usize to u32 to save space (already done for RMQ)
    struct DynamicSegmentTreeNode<Ops: SegmentTreeOperations> {
        data: Ops::Data,
        left_child: usize,
        right_child: usize,
    }

    impl<Ops: SegmentTreeOperations> Default for DynamicSegmentTreeNode<Ops> {
        fn default() -> Self {
            // The root of the tree is present at the start of the nodes vector.
            // So it is safe to set the children indices at 0 (the root will never be a child of anyone).
            Self {
                data: Ops::data_identity(),
                left_child: 0,
                right_child: 0,
            }
        }
    }

    impl<Ops: SegmentTreeOperations> Debug for DynamicSegmentTreeNode<Ops>
    where
        Ops::Data: Debug,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{:?} -> {}, {}",
                self.data, self.left_child, self.right_child,
            )
        }
    }

    pub struct DynamicSegmentTree<B: DynamicSegmentTreeBoundType, Ops: SegmentTreeOperations> {
        pub left_boundary: B,
        pub right_boundary: B,

        nodes: Vec<DynamicSegmentTreeNode<Ops>>,
    }

    impl<B: DynamicSegmentTreeBoundType, Ops: SegmentTreeOperations> DynamicSegmentTree<B, Ops> {
        /// The estimated node size is just to avoid too much vector reallocation, need not be perfect.
        pub fn with_bounds_and_estimated_node_count(
            left_boundary: B,
            right_boundary: B,
            estimated_node_count: usize,
        ) -> Self {
            let mut nodes = Vec::with_capacity(estimated_node_count);
            nodes.push(DynamicSegmentTreeNode::default());
            Self {
                left_boundary,
                right_boundary,
                nodes,
            }
        }

        pub fn update<F>(&mut self, pos: B, update_to: F)
        where
            F: Fn(Ops::Data) -> Ops::Data,
        {
            assert!(
                self.left_boundary <= pos && pos <= self.right_boundary,
                "Tried to update position {} out of bounds [{}, {}]!",
                pos,
                self.left_boundary,
                self.right_boundary
            );
            self._update(0, self.left_boundary, self.right_boundary, pos, update_to);
        }

        fn _update<F>(&mut self, v: usize, l: B, r: B, pos: B, update_to: F)
        where
            F: Fn(Ops::Data) -> Ops::Data,
        {
            if l == r {
                assert_eq!(l, pos);
                let curr_node = std::mem::replace(&mut self.nodes[v].data, Ops::data_identity());
                self.nodes[v].data = update_to(curr_node);
                return;
            }

            if self.nodes[v].left_child == 0 {
                self.nodes[v].left_child = self.nodes.len();
                self.nodes.push(DynamicSegmentTreeNode::default());
            }
            if self.nodes[v].right_child == 0 {
                self.nodes[v].right_child = self.nodes.len();
                self.nodes.push(DynamicSegmentTreeNode::default());
            }

            let m = B::midpoint_floor(l, r);
            if pos <= m {
                self._update(self.nodes[v].left_child, l, m, pos, update_to);
            } else {
                self._update(self.nodes[v].right_child, m + B::one(), r, pos, update_to);
            }
            self.nodes[v].data = Ops::merge(
                &self.nodes[self.nodes[v].left_child].data,
                &self.nodes[self.nodes[v].right_child].data,
            );
        }

        pub fn query<R>(&self, range: R) -> Ops::Data
        where
            R: RangeBounds<B>,
        {
            let mut ans = Ops::data_identity();
            self.query2(range, |node| {
                ans = Ops::merge(&ans, node);
            });
            ans
        }

        pub fn query2<R, F>(&self, range: R, mut access_node: F)
        where
            R: RangeBounds<B>,
            F: FnMut(&Ops::Data),
        {
            let l = match range.start_bound() {
                Bound::Included(&l) => l,
                Bound::Excluded(&l) => l + B::one(),
                Bound::Unbounded => self.left_boundary,
            };
            let r = match range.end_bound() {
                Bound::Included(&r) => r,
                Bound::Excluded(&r) => r - B::one(),
                Bound::Unbounded => self.right_boundary,
            };
            self._query2(
                0,
                self.left_boundary,
                self.right_boundary,
                l,
                r,
                &mut access_node,
            )
        }

        fn _query2<F>(&self, v: usize, l: B, r: B, ql: B, qr: B, access_node: &mut F)
        where
            F: FnMut(&Ops::Data),
        {
            if qr < l || ql > r {
                return;
            }
            if ql <= l && r <= qr {
                access_node(&self.nodes[v].data);
                return;
            }

            let m = B::midpoint_floor(l, r);
            if self.nodes[v].left_child != 0 {
                self._query2(self.nodes[v].left_child, l, m, ql, qr, access_node);
            }
            if self.nodes[v].right_child != 0 {
                self._query2(
                    self.nodes[v].right_child,
                    m + B::one(),
                    r,
                    ql,
                    qr,
                    access_node,
                );
            }
        }
    }

    impl<B: DynamicSegmentTreeBoundType, Ops: SegmentTreeOperations> Debug
        for DynamicSegmentTree<B, Ops>
    where
        Ops::Data: Debug,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            for (i, node) in self.nodes.iter().enumerate() {
                write!(f, "\n{}: {:?}", i, node)?;
            }
            Ok(())
        }
    }
}
use dynamic_seg_tree::*;
