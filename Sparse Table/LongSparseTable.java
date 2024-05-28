class LongSparseTable {
    private static int highestBit(int x) {
        return x == 0 ? -1 : 31 - Integer.numberOfLeadingZeros(x);
    }

    private long[][] rangeValue;
    private final java.util.function.LongBinaryOperator operator;

    public LongSparseTable(long[] values, java.util.function.LongBinaryOperator operator) {
        this.operator = operator;
        build(values);
    }

    private void build(long[] values) {
        int n = values.length;
        int levels = highestBit(n) + 1;

        rangeValue = new long[levels][];
        rangeValue[0] = new long[n];
        System.arraycopy(values, 0, rangeValue[0], 0, n);

        for (int k = 1; k < levels; k++) {
            rangeValue[k] = new long[n - (1 << k) + 1];
            for (int i = 0; i <= n - (1 << k); i++)
                rangeValue[k][i] = operator.applyAsLong(rangeValue[k - 1][i], rangeValue[k - 1][i + (1 << (k - 1))]);
        }
    }

    public long query(int a, int b) {
        int level = highestBit(b - a + 1);
        return operator.applyAsLong(rangeValue[level][a], rangeValue[level][b - (1 << level) + 1]);
    }
}