/// # Li Chao Tree ([source](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/DataStructures/li_chao_tree.rs))
///
/// You'll need the module [math.rs](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/CombinedRustModules/math.rs) to use the same.
#[allow(private_interfaces)]
pub mod li_chao_tree {
    use super::{MultiplicativeIdentity, RoundedMidpoint};
    use std::fmt::{Debug, Display, Formatter};
    use std::ops::{Add, Mul};

    /// TODO: Create and move it to a new geometry module (first study a little bit of geometry and create some more template items).
    #[derive(Clone)]
    pub struct Line<T: Clone> {
        m: T,
        c: T,
    }

    impl<T: Clone> Line<T> {
        #[inline]
        pub fn new(m: T, c: T) -> Self {
            Self { m, c }
        }

        #[inline]
        pub fn eval(&self, x: T) -> T
        where
            T: Mul<Output = T> + Add<Output = T> + Copy,
        {
            self.m * x + self.c
        }
    }

    impl<T: Display + Clone> Debug for Line<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "y = {} * x + {}", self.m, self.c)
        }
    }

    struct LiChaoTreeNode<T: Clone> {
        line: Line<T>,
        left_child: u32,
        right_child: u32,
    }

    pub struct LiChaoTree<T: Copy + Clone, const FINDING_MAX: bool> {
        min_x: T,
        max_x: T,
        /// If `FINDING_MAX` is true, `border_y` should be -INF and vice versa.
        border_y: T,
        pub tree: Vec<LiChaoTreeNode<T>>,
    }

    impl<
        T: Default
        + Copy
        + Clone
        + Ord
        + Mul<Output = T>
        + Add<Output = T>
        + RoundedMidpoint
        + MultiplicativeIdentity
        + PartialOrd,
        const FINDING_MAX: bool,
    > LiChaoTree<T, FINDING_MAX>
    {
        /// If `FINDING_MAX` is true, `border_y` should be -INF and vice versa.
        pub fn new(min_x: T, max_x: T, border_y: T, estimated_number_of_nodes: usize) -> Self {
            let mut obj = Self {
                min_x,
                max_x,
                border_y,
                tree: Vec::with_capacity(estimated_number_of_nodes),
            };
            obj.tree.push(obj.default_node());
            obj
        }

        fn default_node(&self) -> LiChaoTreeNode<T>
        where
            T: Default,
        {
            LiChaoTreeNode {
                line: Line {
                    m: T::default(),
                    c: self.border_y,
                },
                left_child: 0,
                right_child: 0,
            }
        }

        pub fn add_line(&mut self, line: Line<T>, seg_min_x: T, seg_max_x: T) {
            self._add_line(0, line, self.min_x, self.max_x, seg_min_x, seg_max_x);
        }

        // First attempt, will require some refactoring
        fn _add_line(
            &mut self,
            i: usize,
            mut line: Line<T>,
            node_left: T,
            node_right: T,
            line_left: T,
            line_right: T,
        ) {
            // Tried Range::intersect, but it is not stable yet.
            if line_right < node_left || line_left > node_right {
                return;
            }

            let node_mid = T::midpoint_floor(node_left, node_right);
            // if line_range contains node_range
            if line_left <= node_left && node_right <= line_right {
                // The entire range is considered

                let left_new_better =
                    (line.eval(node_left) > self.tree[i].line.eval(node_left)) == FINDING_MAX;
                let mid_new_better =
                    (line.eval(node_mid) > self.tree[i].line.eval(node_mid)) == FINDING_MAX;
                let right_new_better =
                    (line.eval(node_right) > self.tree[i].line.eval(node_right)) == FINDING_MAX;
                let leaf_node = node_left == node_right;

                if mid_new_better {
                    std::mem::swap(&mut self.tree[i].line, &mut line);
                }

                // New line is either completely better or worse, so no propagating to the child nodes.
                // Better case handled in the swap above
                if left_new_better == right_new_better || leaf_node {
                    return;
                }

                if left_new_better == mid_new_better {
                    // We'll only check right
                    if self.tree[i].right_child == 0 {
                        self.tree[i].right_child = self.tree.len() as u32;
                        self.tree.push(self.default_node());
                    }

                    self._add_line(
                        self.tree[i].right_child as usize,
                        line,
                        node_mid + T::one(),
                        node_right,
                        line_left,
                        line_right,
                    );
                } else {
                    // We'll check left
                    if self.tree[i].left_child == 0 {
                        self.tree[i].left_child = self.tree.len() as u32;
                        self.tree.push(self.default_node());
                    }

                    self._add_line(
                        self.tree[i].left_child as usize,
                        line.clone(),
                        node_left,
                        node_mid,
                        line_left,
                        line_right,
                    );
                }
            } else {
                // The entire range of the node is not considered, so splitting into 2.

                if line_left <= node_mid {
                    if self.tree[i].left_child == 0 {
                        self.tree[i].left_child = self.tree.len() as u32;
                        self.tree.push(self.default_node());
                    }

                    self._add_line(
                        self.tree[i].left_child as usize,
                        line.clone(),
                        node_left,
                        node_mid,
                        line_left,
                        line_right,
                    );
                }

                if line_right > node_mid {
                    if self.tree[i].right_child == 0 {
                        self.tree[i].right_child = self.tree.len() as u32;
                        self.tree.push(self.default_node());
                    }

                    self._add_line(
                        self.tree[i].right_child as usize,
                        line,
                        node_mid + T::one(),
                        node_right,
                        line_left,
                        line_right,
                    );
                }
            }
        }

        pub fn query(&self, x: T) -> T {
            let mut i = 0;
            let mut l = self.min_x;
            let mut r = self.max_x;
            let mut ans = self.border_y;
            loop {
                // assert!((l..=r).contains(&x));
                let new_val = self.tree[i].line.eval(x);
                ans = if (ans > new_val) == FINDING_MAX {
                    ans
                } else {
                    new_val
                };
                let m = T::midpoint_floor(l, r);
                (i, l, r) = if x <= m {
                    (self.tree[i].left_child as usize, l, m)
                } else {
                    (self.tree[i].right_child as usize, m + T::one(), r)
                };

                if i == 0 {
                    return ans;
                }
            }
        }
    }
}
use li_chao_tree::*;