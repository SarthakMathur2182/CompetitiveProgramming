class IntRMQ {
    private static int highestBit(int x) {
        return x == 0 ? -1 : 31 - Integer.numberOfLeadingZeros(x);
    }

    private final int n;
    private final int[] values;
    private int[][] rangeIndex;
    private final boolean findingMaximum;

    /**
     * @param findingMaximum If {@code false}, we are doing Range Minimum Query.
     *                       <p>If {@code true}, we are doing Range Maximum Query.
     */
    public IntRMQ(int[] values, boolean findingMaximum) {
        this.n = values.length;
        this.values = java.util.Arrays.copyOf(values, n);
        this.findingMaximum = findingMaximum;
        build();
    }

    /**
     * Returns the index having the minimum/maximum value, depending on what we've defined.
     * <p> If both the values are the same, {@code b} is returned.
     */
    private int betterIndex(int a, int b) {
        return (findingMaximum ? values[b] < values[a] : values[a] < values[b]) ? a : b;
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
     * Returns the index having the smallest (or the largest) value in the range {@code [a, b]} (both inclusive).
     * <p>If there are multiple possible indices, we're considering the largest index.
     *
     * @see #queryValue(int, int)
     */
    public int queryIndex(int a, int b) {
        int level = highestBit(b - a + 1);
        return betterIndex(rangeIndex[level][a], rangeIndex[level][b - (1 << level) + 1]);
    }

    /**
     * Returns the smallest (or the largest) value in the index range {@code [a, b]} (both inclusive).
     *
     * @see #queryIndex(int, int)
     */
    public int queryValue(int a, int b) {
        return values[queryIndex(a, b)];
    }
}