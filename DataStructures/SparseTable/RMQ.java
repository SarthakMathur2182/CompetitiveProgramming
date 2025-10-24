class RMQ<T> {
    private static int highestBit(int x) {
        return x == 0 ? -1 : 31 - Integer.numberOfLeadingZeros(x);
    }

    private final int n;
    private final T[] values;
    private int[][] rangeIndex;
    private final java.util.Comparator<T> comparator;

    /**
     * @param comparator The comparator we'll use to compare values.
     *                   The smaller value (according to the comparator) will be considered.
     */
    public RMQ(T[] values, java.util.Comparator<T> comparator) {
        this.n = values.length;
        this.values = java.util.Arrays.copyOf(values, n);
        this.comparator = comparator;
        build();
    }

    /**
     * Returns the index having the minimum value according to the comparator.
     * <p> If both the values are the same, {@code b} is returned.
     */
    private int betterIndex(int a, int b) {
        return comparator.compare(values[a], values[b]) < 0 ? a : b;
    }

    private void build() {
        int levels = highestBit(n) + 1;
        rangeIndex = new int[levels][];

        rangeIndex[0] = new int[n];
        for (int i = 0; i < n; i++)
            rangeIndex[0][i] = i;

        for (int k = 1; k < levels; k++) {
            rangeIndex[k] = new int[n - (1 << k) + 1];
            for (int i = 0; i <= n - (1 << k); i++)
                rangeIndex[k][i] = betterIndex(rangeIndex[k - 1][i], rangeIndex[k - 1][i + (1 << (k - 1))]);
        }
    }

    /**
     * Returns the index having the smallest value (according to the comparator) in the range {@code [a, b]} (both inclusive).
     * <p>If there are multiple possible indices, we're considering the largest index.
     *
     * @see #queryValue(int, int)
     */
    public int queryIndex(int a, int b) {
        int level = highestBit(b - a + 1);
        return betterIndex(rangeIndex[level][a], rangeIndex[level][b - (1 << level) + 1]);
    }

    /**
     * Returns the smallest value (according to the comparator) in the index range {@code [a, b]} (both inclusive).
     *
     * @see #queryIndex(int, int)
     */
    public T queryValue(int a, int b) {
        return values[queryIndex(a, b)];
    }
}