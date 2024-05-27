# RMQ and Sparse Table

You can read more about this here: [Sparse Table](https://cp-algorithms.com/data_structures/sparse-table.html)

---

This implementation only supports Idempotent operations, which don't have any issue with overlapping.

Operations like addition, multiplication and bitwise XOR are not allowed.

---

The RMQ implementation is inspired from Neal Wu's C++ implementation.

The difference is obviously the language, and I prefer the query range [l, r] both inclusive.