/**
 * This is an abstract class, so you have to override the methods {@link #setIdentity()} and {@link #merge(Node, Node)}.
 * <p>The implementation could've also called them as constructor parameters, but this way seemed cleaner, especially when we might have multiple functions, for example in lazy propagation.
 */
abstract class SegmentTree<Node> {
    Node[] tree;
    private final int n;
    Node IDENTITY;

    /**
     * All the elements are initialized with the identity element.
     *
     * @param n The length of the segment
     */
    @SuppressWarnings("unchecked")
    public SegmentTree(int n) {
        this.n = ceilingPowerOf2(n);
        tree = (Node[]) new Object[this.n << 1];
        setIdentity();
        java.util.Arrays.fill(tree, IDENTITY);
    }

    /**
     * @param a The array we're considering as the segment
     */
    public SegmentTree(Node[] a) {
        this(i -> a[i], a.length);
    }

    /**
     * In case the array is not declared, we can use this to get the array value at the i-th index.
     *
     * @param init The function giving us the value corresponding to the {@code i-th} index.
     * @param n    The length of the segment
     */
    public SegmentTree(java.util.function.IntFunction<Node> init, int n) {
        this(n);
        for (int i = 0; i < n; i++)
            tree[this.n + i] = init.apply(i);
        for (int i = this.n - 1; i > 0; i--)
            tree[i] = merge(tree[i << 1], tree[i << 1 | 1]);
    }

    /**
     * Get the number of leaves (assuming a perfect binary tree, the number of leaves should be a power of 2).
     *
     * <p>The size of the tree will be {@code 2 * n - 1}. We are making the tree of size {@code 2 * n}, ignoring {@code tree[0]}.
     *
     * <p>We can also simply take the size of the tree as {@code 4 * n} instead of calling this method.
     */
    private static int ceilingPowerOf2(int n) {
        if ((n & (n - 1)) == 0)
            return n;
        n |= n >> 1;
        n |= n >> 2;
        n |= n >> 4;
        n |= n >> 8;
        n |= n >> 16;
        return n + 1;
    }

    /**
     * @return The number of leaves (need not be the same as the initial size of the segment).
     */
    public int size() {
        return n;
    }

    /**
     * Point-update in the Segment Tree. The function {@code updateTo} allows multiple types of updates.
     *
     * <p>If we have to update the value to {@code x}, {@code updateTo = c -> x}
     *
     * @param position The 0-indexed position we're updating.
     * @param updateTo The function taking the current value as the input and returning the updated value.
     */
    public void update(int position, java.util.function.UnaryOperator<Node> updateTo) {
        update(1, 0, n - 1, position, updateTo);
    }

    private void update(int v, int l, int r, int position, java.util.function.UnaryOperator<Node> updateTo) {
        if (position < l || position > r)
            return;
        if (l == r) {
            tree[v] = updateTo.apply(tree[v]);
            return;
        }
        int mid = (l + r) >> 1;
        update(v << 1, l, mid, position, updateTo);
        update(v << 1 | 1, mid + 1, r, position, updateTo);
        tree[v] = merge(tree[v << 1], tree[v << 1 | 1]);
    }

    /**
     * In simple Segment Tree, we can directly return the {@code p-th} index's value like this, but in case of lazy propagation we have to push the lazy values first.
     *
     * @return The value at the {@code p-th} index (0-based indexing)
     */
    public Node query(int p) {
        return tree[this.n + p];
    }

    /**
     * @return Range query over the segment {@code [l, r]} (both inclusive)
     */
    public Node query(int l, int r) {
        return query(1, 0, n - 1, l, r);
    }

    private Node query(int v, int l, int r, int ql, int qr) {
        if (ql > r || qr < l)
            return IDENTITY;
        if (ql <= l && qr >= r)
            return tree[v];
        int m = (l + r) >> 1;
        return merge(query(v << 1, l, m, ql, qr), query(v << 1 | 1, m + 1, r, ql, qr));
    }

    /**
     * Sets the identity element for the binary operator defined in {@link #merge(Node, Node)}
     */
    public abstract void setIdentity();

    /**
     * The binary operator which we'll apply on the tree.
     */
    protected abstract Node merge(Node a, Node b);
}