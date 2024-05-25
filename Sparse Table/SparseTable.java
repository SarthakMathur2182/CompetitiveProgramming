class SparseTable<T> {
    private static int highestBit(int x) {
        return x == 0 ? -1 : 31 - Integer.numberOfLeadingZeros(x);
    }

    private T[][] rangeValue;
    java.util.function.BinaryOperator<T> operator;

    public SparseTable(T[] values, java.util.function.BinaryOperator<T> operator) {
        this.operator = operator;
        build(values);
    }

    @SuppressWarnings("unchecked")
    void build(T[] values) {
        int n = values.length;
        int levels = highestBit(n) + 1;

        rangeValue = (T[][]) new Object[levels][];
        rangeValue[0] = (T[]) new Object[n];
        System.arraycopy(values, 0, rangeValue[0], 0, n);

        for (int k = 1; k < levels; k++) {
            rangeValue[k] = (T[]) new Object[n - (1 << k) + 1];
            for (int i = 0; i <= n - (1 << k); i++)
                rangeValue[k][i] = operator.apply(rangeValue[k - 1][i], rangeValue[k - 1][i + (1 << (k - 1))]);
        }
    }

    T query(int a, int b) {
        int level = highestBit(b - a + 1);
        return operator.apply(rangeValue[level][a], rangeValue[level][b - (1 << level) + 1]);
    }
}