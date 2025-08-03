/**
 * All the functions are static.
 */
class BS {
    /**
     * It is assumed that {@code predicate} will give false for all values less than {@code l}, and true for all values greater than {@code r}.
     */
    public static int firstTrue(int l, int r, java.util.function.IntPredicate predicate) {
        r++;
        while (l < r) {
            int m = l + (r - l) / 2;

            if (predicate.test(m)) r = m;
            else l = m + 1;
        }

        return r;
    }

    /**
     * It is assumed that {@code predicate} will give false for all values less than {@code l}, and true for all values greater than {@code r}.
     */
    public static long firstTrue(long l, long r, java.util.function.LongPredicate predicate) {
        r++;
        while (l < r) {
            long m = l + (r - l) / 2;

            if (predicate.test(m)) r = m;
            else l = m + 1;
        }

        return r;
    }

    /**
     * It is assumed that {@code predicate} will give true for all values less than {@code l}, and false for all values greater than {@code r}.
     */
    public static int lastTrue(int l, int r, java.util.function.IntPredicate predicate) {
        return lastFalse(l, r, predicate.negate());
    }

    /**
     * It is assumed that {@code predicate} will give true for all values less than {@code l}, and false for all values greater than {@code r}.
     */
    public static long lastTrue(long l, long r, java.util.function.LongPredicate predicate) {
        return lastFalse(l, r, predicate.negate());
    }

    /**
     * It is assumed that {@code predicate} will give true for all values less than {@code l}, and false for all values greater than {@code r}.
     */
    public static int firstFalse(int l, int r, java.util.function.IntPredicate predicate) {
        return firstTrue(l, r, predicate.negate());
    }

    /**
     * It is assumed that {@code predicate} will give true for all values less than {@code l}, and false for all values greater than {@code r}.
     */
    public static long firstFalse(long l, long r, java.util.function.LongPredicate predicate) {
        return firstTrue(l, r, predicate.negate());
    }

    /**
     * It is assumed that {@code predicate} will give false for all values less than {@code l}, and true for all values greater than {@code r}.
     */
    public static int lastFalse(int l, int r, java.util.function.IntPredicate predicate) {
        return firstTrue(l, r, predicate) - 1;
    }

    /**
     * It is assumed that {@code predicate} will give false for all values less than {@code l}, and true for all values greater than {@code r}.
     */
    public static long lastFalse(long l, long r, java.util.function.LongPredicate predicate) {
        return firstTrue(l, r, predicate) - 1;
    }


    public static int lowerBound(int[] a, int l, int r, int x) {
        return firstTrue(l, r, (java.util.function.IntPredicate) m -> a[m] >= x);
    }

    public static int lowerBound(int[] a, int x) {
        return lowerBound(a, 0, a.length - 1, x);
    }

    public static int lowerBound(long[] a, int l, int r, long x) {
        return firstTrue(l, r, (java.util.function.IntPredicate) m -> a[m] >= x);
    }

    public static int lowerBound(long[] a, long x) {
        return lowerBound(a, 0, a.length - 1, x);
    }

    public static <T extends Comparable<T>> int lowerBound(T[] a, int l, int r, T x) {
        return lowerBound(a, l, r, x, T::compareTo);
    }

    public static <T extends Comparable<T>> int lowerBound(T[] a, T x) {
        return lowerBound(a, 0, a.length - 1, x);
    }

    public static <T> int lowerBound(T[] a, int l, int r, T x, java.util.Comparator<T> comparator) {
        return firstTrue(l, r, (java.util.function.IntPredicate) m -> comparator.compare(a[m], x) >= 0);
    }

    public static <T> int lowerBound(T[] a, T x, java.util.Comparator<T> comparator) {
        return lowerBound(a, 0, a.length - 1, x, comparator);
    }


    public static int upperBound(int[] a, int l, int r, int x) {
        return firstTrue(l, r, (java.util.function.IntPredicate) m -> a[m] > x);
    }

    public static int upperBound(int[] a, int x) {
        return upperBound(a, 0, a.length - 1, x);
    }

    public static int upperBound(long[] a, int l, int r, long x) {
        return firstTrue(l, r, (java.util.function.IntPredicate) m -> a[m] > x);
    }

    public static int upperBound(long[] a, long x) {
        return upperBound(a, 0, a.length - 1, x);
    }

    public static <T extends Comparable<T>> int upperBound(T[] a, int l, int r, T x) {
        return upperBound(a, l, r, x, T::compareTo);
    }

    public static <T extends Comparable<T>> int upperBound(T[] a, T x) {
        return upperBound(a, 0, a.length - 1, x);
    }

    public static <T> int upperBound(T[] a, int l, int r, T x, java.util.Comparator<T> comparator) {
        return firstTrue(l, r, (java.util.function.IntPredicate) m -> comparator.compare(a[m], x) > 0);
    }

    public static <T> int upperBound(T[] a, T x, java.util.Comparator<T> comparator) {
        return upperBound(a, 0, a.length - 1, x, comparator);
    }
}