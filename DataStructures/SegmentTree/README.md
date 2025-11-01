# Segment Tree

You can read more about Segment Tree [here](https://cp-algorithms.com/data_structures/segment_tree.html).

---

The java implementation contains abstract classes, so you have to override some methods.

---

The rust implementation is similar to that of java's (not the abstract classes obviously).
You need to create a struct to implement the traits and pass it as generics.

- Simple point-update Segment tree will require the trait `SegmentTreeOperations`.
- Segment tree with Lazy Propagation will require both `SegmentTreeOperations` and `LazySegmentTreeOperations`.
