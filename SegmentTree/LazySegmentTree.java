/**
 * This is an abstract class, so you have to override the methods: {@link #setDataIdentity()}, {@link #setLazyIdentity()}, {@link #merge(Data, Data)}, {@link #apply(Data, int, int, Lazy)} and {@link #compose(Lazy, Lazy)}.
 * <p>The implementation could've also called them as constructor parameters, but this way seemed cleaner because we have so many functions, especially in lazy propagation.
 */
abstract class LazySegmentTree<Data, Lazy> {
    private final int n;
    private final Data[] dataNodes;
    private final Lazy[] lazyNodes;
    Data DATA_IDENTITY;
    Lazy LAZY_IDENTITY;

    /**
     * All the elements are initialized with the identity element.
     *
     * @param n The length of the segment
     */
    @SuppressWarnings("unchecked")
    public LazySegmentTree(int n) {
        this.n = ceilingPowerOf2(n);
        dataNodes = (Data[]) new Object[this.n << 1];
        setDataIdentity();
        java.util.Arrays.fill(dataNodes, DATA_IDENTITY);
        lazyNodes = (Lazy[]) new Object[this.n << 1];
        setLazyIdentity();
        java.util.Arrays.fill(lazyNodes, LAZY_IDENTITY);
    }

    /**
     * @param dataNodes The array we're considering as the segment
     */
    public LazySegmentTree(Data[] dataNodes) {
        this(i -> dataNodes[i], dataNodes.length);
    }

    /**
     * In case the array is not declared, we can use this to get the array value at the i-th index.
     *
     * @param init The function giving us the value corresponding to the {@code i-th} index.
     * @param n    The length of the segment
     */
    public LazySegmentTree(java.util.function.IntFunction<Data> init, int n) {
        this(n);
        for (int i = 0; i < n; i++)
            dataNodes[this.n + i] = init.apply(i);
        for (int i = this.n - 1; i > 0; i--)
            dataNodes[i] = merge(dataNodes[i << 1], dataNodes[i << 1 | 1]);
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
     * Range update over the range {@code [l, r]} (both inclusive).
     */
    public void rangeUpdate(int l, int r, Lazy updateTo) {
        rangeUpdate(1, 0, n - 1, l, r, updateTo);
    }

    private void rangeUpdate(int p, int l, int r, int ql, int qr, Lazy updateTo) {
        if (qr < l || ql > r)
            return;

        if (ql <= l && r <= qr) {
            applyAt(p, l, r, updateTo);
            return;
        }

        int m = (l + r) >> 1;
        push(p, l, r);

        rangeUpdate(p << 1, l, m, ql, qr, updateTo);
        rangeUpdate(p << 1 | 1, m + 1, r, ql, qr, updateTo);
        dataNodes[p] = merge(dataNodes[p << 1], dataNodes[p << 1 | 1]);
    }

    /**
     * Applying the lazy operation over a node's data value and the previous lazy value.
     *
     * @param p The index of the node we're applying the operation on.
     */
    private void applyAt(int p, int l, int r, Lazy lazy) {
        dataNodes[p] = apply(dataNodes[p], l, r, lazy);
        lazyNodes[p] = compose(lazyNodes[p], lazy);
    }

    /**
     * Propagate the lazy operation to the node's children.
     *
     * @param p The parent, whose lazy value we are propagating.
     */
    private void push(int p, int l, int r) {
        int m = (l + r) >> 1;
        applyAt(p << 1, l, m, lazyNodes[p]);
        applyAt(p << 1 | 1, m + 1, r, lazyNodes[p]);
        lazyNodes[p] = LAZY_IDENTITY;
    }

    /**
     * @return Range query over the segment {@code [l, r]} (both inclusive)
     */
    public Data rangeQuery(int l, int r) {
        return rangeQuery(1, 0, n - 1, l, r);
    }

    private Data rangeQuery(int p, int l, int r, int ql, int qr) {
        if (qr < l || ql > r)
            return DATA_IDENTITY;

        if (ql <= l && r <= qr)
            return dataNodes[p];

        int m = (l + r) >> 1;
        push(p, l, r);
        return merge(rangeQuery(p << 1, l, m, ql, qr), rangeQuery(p << 1 | 1, m + 1, r, ql, qr));
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();
        for (int i = 1; i < n << 1; i++)
            sb.append(String.format("{%d-{%s, %s} ", i, dataNodes[i].toString(), lazyNodes[i].toString()));
        return sb.toString();
    }

    /**
     * The binary operator which we'll apply on the tree.
     */
    public abstract Data merge(Data a, Data b);

    /**
     * Applying the lazy operation over a node's data.
     */
    public abstract Data apply(Data data, int l, int r, Lazy lazy);

    /**
     * Update the lazy operation.
     */
    public abstract Lazy compose(Lazy prev, Lazy next);

    /**
     * Sets the identity element for the lazy operation, which signifies that there is no lazy update operation.
     */
    public abstract void setLazyIdentity();

    /**
     * Sets the identity element for the binary operator defined in {@link #merge(Data, Data)}
     */
    public abstract void setDataIdentity();
}