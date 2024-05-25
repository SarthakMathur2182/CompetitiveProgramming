class IntSparseTable {
    private static int highestBit(int x) {
        return x == 0 ? -1 : 31 - Integer.numberOfLeadingZeros(x);
    }

    private int[][] rangeValue;
    java.util.function.IntBinaryOperator operator;

    public IntSparseTable(int[] values, java.util.function.IntBinaryOperator operator) {
        this.operator = operator;
        build(values);
    }

    void build(int[] values) {
        int n = values.length;
        int levels = highestBit(n) + 1;

        rangeValue = new int[levels][];
        rangeValue[0] = new int[n];
        System.arraycopy(values, 0, rangeValue[0], 0, n);

        for (int k = 1; k < levels; k++) {
            rangeValue[k] = new int[n - (1 << k) + 1];
            for (int i = 0; i <= n - (1 << k); i++)
                rangeValue[k][i] = operator.applyAsInt(rangeValue[k - 1][i], rangeValue[k - 1][i + (1 << (k - 1))]);
        }
    }

    int query(int a, int b) {
        int level = highestBit(b - a + 1);
        return operator.applyAsInt(rangeValue[level][a], rangeValue[level][b - (1 << level) + 1]);
    }
}