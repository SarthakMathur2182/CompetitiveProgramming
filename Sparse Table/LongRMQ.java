/**
 * For Range Minimum/Maximum Queries, both value and index.
 * <p>
 * Inspired from Neal Wu's template. The difference is the language, and I prefer the query range ends both inclusive.
 */

class LongRMQ {
    private static int highestBit(int x) {
        return x == 0 ? -1 : 31 - Integer.numberOfLeadingZeros(x);
    }

    private final int n;
    private final long[] values;
    private int[][] rangeIndex;
    private final boolean findingMaximum;

    public LongRMQ(long[] values, boolean findingMaximum) {
        this.n = values.length;
        this.values = java.util.Arrays.copyOf(values, n);
        this.findingMaximum = findingMaximum;
        build();
    }

    // Note: when `values[a] == values[b]`, returns b.
    int betterIndex(int a, int b) {
        return (findingMaximum ? values[b] < values[a] : values[a] < values[b]) ? a : b;
    }

    void build() {
        int levels = highestBit(n) + 1;
        rangeIndex = new int[levels][];

        for (int k = 0; k < levels; k++)
            rangeIndex[k] = new int[n - (1 << k) + 1];

        for (int i = 0; i < n; i++)
            rangeIndex[0][i] = i;

        for (int k = 1; k < levels; k++)
            for (int i = 0; i <= n - (1 << k); i++)
                rangeIndex[k][i] = betterIndex(rangeIndex[k - 1][i], rangeIndex[k - 1][i + (1 << (k - 1))]);
    }

    // Note: breaks ties by choosing the largest index.
    int queryIndex(int a, int b) {
        int level = highestBit(b - a + 1);
        return betterIndex(rangeIndex[level][a], rangeIndex[level][b - (1 << level) + 1]);
    }

    long queryValue(int a, int b) {
        return values[queryIndex(a, b)];
    }
}