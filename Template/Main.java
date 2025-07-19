/*
    Author: WORTH
    Problem:
*/
// Solution at end

import java.util.*;
import java.io.*;
import java.util.function.*;

public class Main {
    static ContestScanner sc = new ContestScanner();
    static FastWriter out = new FastWriter();

    public static void main(String[] args) throws Exception {
        // sc = new ContestScanner(new File("input.txt"));
        // out = new FastWriter("output.txt");

        boolean debug = args.length > 0 && args[0].equals("-DEBUG");
        if (debug) {
            out.println("New Sample:");
            boolean append = args.length > 1;
            System.setErr(new PrintStream(new FileOutputStream("D:\\Codes\\CPRelatedFiles\\error.txt", append), true));
        }

        Thread t = new Thread(null, new ActualSolution(sc, out, debug), "actual_solution", 256 << 20);
        t.setUncaughtExceptionHandler(($, e) -> {
            try {
                out.flush();
                throw e;
            } catch (NoSuchElementException fine) {
                System.exit(0);
            } catch (Throwable issue) {
                issue.printStackTrace(System.err);
                System.exit(1);
            }
        });
        t.start();
        t.join();
        out.flush();
        if (debug) main(new String[]{"-DEBUG", "Again"});
    }
}

/**
 * @see <a href = "https://github.com/NASU41/AtCoderLibraryForJava/blob/master/ContestIO/ContestScanner.java">Source code by NASU41 (Slightly Modified)</a>
 */
class ContestScanner {
    private static final long LONG_MAX_TENTHS = 922337203685477580L;
    private static final int LONG_MAX_LAST_DIGIT = 7;
    private static final int LONG_MIN_LAST_DIGIT = 8;
    private final InputStream in;
    private final byte[] buffer = new byte[1024];
    private int ptr = 0;
    private int buflen = 0;

    public ContestScanner(InputStream in) {
        this.in = in;
    }

    public ContestScanner(File file) throws FileNotFoundException {
        this(new BufferedInputStream(new FileInputStream(file)));
    }

    public ContestScanner() {
        this(System.in);
    }

    private static boolean isPrintableChar(int c) {
        return 33 <= c && c <= 126;
    }

    private boolean hasNextByte() {
        if (ptr < buflen) return true;
        ptr = 0;
        try {
            buflen = in.read(buffer);
        } catch (IOException e) {
            e.printStackTrace();
        }
        return buflen > 0;
    }

    private int readByte() {
        if (hasNextByte()) return buffer[ptr++];
        else return -1;
    }

    public boolean hasNext() {
        while (hasNextByte() && !isPrintableChar(buffer[ptr])) ptr++;
        return hasNextByte();
    }

    public String next() {
        if (!hasNext()) throw new NoSuchElementException();
        StringBuilder sb = new StringBuilder();
        int b = readByte();
        while (isPrintableChar(b)) {
            sb.appendCodePoint(b);
            b = readByte();
        }
        return sb.toString();
    }

    public String nextLine() {
        if (!hasNext()) throw new NoSuchElementException();
        StringBuilder sb = new StringBuilder();
        int b = readByte();
        while (isPrintableChar(b) || b == ' ') {
            sb.appendCodePoint(b);
            b = readByte();
        }
        return sb.toString().trim();
    }

    public String[] nextStringArray(int length) {
        String[] array = new String[length];
        for (int i = 0; i < length; i++) array[i] = this.next();
        return array;
    }

    public String[] nextStringArray(int length, UnaryOperator<String> map) {
        String[] array = new String[length];
        for (int i = 0; i < length; i++) array[i] = map.apply(this.next());
        return array;
    }

    public String[][] nextStringMatrix(int height, int width) {
        String[][] mat = new String[height][width];
        for (int h = 0; h < height; h++) {
            for (int w = 0; w < width; w++) {
                mat[h][w] = this.next();
            }
        }
        return mat;
    }

    public String[][] nextStringMatrix(int height, int width, UnaryOperator<String> map) {
        String[][] mat = new String[height][width];
        for (int h = 0; h < height; h++) {
            for (int w = 0; w < width; w++) {
                mat[h][w] = map.apply(this.next());
            }
        }
        return mat;
    }

    public char[][] nextCharMatrix(int height, int width) {
        char[][] mat = new char[height][width];
        for (int h = 0; h < height; h++) {
            String s = this.next();
            for (int w = 0; w < width; w++) {
                mat[h][w] = s.charAt(w);
            }
        }
        return mat;
    }

    public char[][] nextCharMatrix(int height, int width, UnaryOperator<Character> map) {
        char[][] mat = new char[height][width];
        for (int h = 0; h < height; h++) {
            String s = this.next();
            for (int w = 0; w < width; w++) {
                mat[h][w] = map.apply(s.charAt(w));
            }
        }
        return mat;
    }

    public int nextInt() {
        long nl = nextLong();
        if (nl < Integer.MIN_VALUE || nl > Integer.MAX_VALUE) throw new NumberFormatException();
        return (int) nl;
    }

    public int[] nextIntArray(int length) {
        int[] array = new int[length];
        for (int i = 0; i < length; i++) array[i] = this.nextInt();
        return array;
    }

    public int[] nextIntArray(int length, IntUnaryOperator map) {
        int[] array = new int[length];
        for (int i = 0; i < length; i++) array[i] = map.applyAsInt(this.nextInt());
        return array;
    }

    public int[][] nextIntMatrix(int height, int width) {
        int[][] mat = new int[height][width];
        for (int h = 0; h < height; h++)
            for (int w = 0; w < width; w++) {
                mat[h][w] = this.nextInt();
            }
        return mat;
    }

    public int[][] nextIntMatrix(int height, int width, IntUnaryOperator map) {
        int[][] mat = new int[height][width];
        for (int h = 0; h < height; h++)
            for (int w = 0; w < width; w++) {
                mat[h][w] = map.applyAsInt(this.nextInt());
            }
        return mat;
    }

    public Integer[] nextIntegerArray(int length) {
        Integer[] array = new Integer[length];
        for (int i = 0; i < length; i++) array[i] = this.nextInt();
        return array;
    }

    public Integer[] nextIntegerArray(int length, IntUnaryOperator map) {
        Integer[] array = new Integer[length];
        for (int i = 0; i < length; i++) array[i] = map.applyAsInt(this.nextInt());
        return array;
    }

    public Integer[][] nextIntegerMatrix(int height, int width) {
        Integer[][] mat = new Integer[height][width];
        for (int h = 0; h < height; h++)
            for (int w = 0; w < width; w++) {
                mat[h][w] = this.nextInt();
            }
        return mat;
    }

    public Integer[][] nextIntegerMatrix(int height, int width, IntUnaryOperator map) {
        Integer[][] mat = new Integer[height][width];
        for (int h = 0; h < height; h++)
            for (int w = 0; w < width; w++) {
                mat[h][w] = map.applyAsInt(this.nextInt());
            }
        return mat;
    }

    public long nextLong() {
        if (!hasNext()) throw new NoSuchElementException();
        long n = 0;
        boolean minus = false;
        int b = readByte();
        if (b == '-') {
            minus = true;
            b = readByte();
        }
        if (b < '0' || '9' < b) {
            throw new NumberFormatException();
        }
        while (true) {
            if ('0' <= b && b <= '9') {
                int digit = b - '0';
                if (n >= LONG_MAX_TENTHS) {
                    if (n == LONG_MAX_TENTHS) {
                        if (minus) {
                            if (digit <= LONG_MIN_LAST_DIGIT) {
                                n = -n * 10 - digit;
                                b = readByte();
                                if (!isPrintableChar(b)) {
                                    return n;
                                } else if (b < '0' || '9' < b) {
                                    throw new NumberFormatException(String.format("%d%s... is not number", n, Character.toString(b)));
                                }
                            }
                        } else {
                            if (digit <= LONG_MAX_LAST_DIGIT) {
                                n = n * 10 + digit;
                                b = readByte();
                                if (!isPrintableChar(b)) {
                                    return n;
                                } else if (b < '0' || '9' < b) {
                                    throw new NumberFormatException(String.format("%d%s... is not number", n, Character.toString(b)));
                                }
                            }
                        }
                    }
                    throw new ArithmeticException(String.format("%s%d%d... overflows long.", minus ? "-" : "", n, digit));
                }
                n = n * 10 + digit;
            } else if (b == -1 || !isPrintableChar(b)) {
                return minus ? -n : n;
            } else {
                throw new NumberFormatException();
            }
            b = readByte();
        }
    }

    public long[] nextLongArray(int length) {
        long[] array = new long[length];
        for (int i = 0; i < length; i++) array[i] = this.nextLong();
        return array;
    }

    public long[] nextLongArray(int length, LongUnaryOperator map) {
        long[] array = new long[length];
        for (int i = 0; i < length; i++) array[i] = map.applyAsLong(this.nextLong());
        return array;
    }

    public long[][] nextLongMatrix(int height, int width) {
        long[][] mat = new long[height][width];
        for (int h = 0; h < height; h++)
            for (int w = 0; w < width; w++) {
                mat[h][w] = this.nextLong();
            }
        return mat;
    }

    public long[][] nextLongMatrix(int height, int width, LongUnaryOperator map) {
        long[][] mat = new long[height][width];
        for (int h = 0; h < height; h++)
            for (int w = 0; w < width; w++) {
                mat[h][w] = map.applyAsLong(this.nextLong());
            }
        return mat;
    }

    public Long[] nextLongWrapperArray(int length) {
        Long[] array = new Long[length];
        for (int i = 0; i < length; i++) array[i] = this.nextLong();
        return array;
    }

    public Long[] nextLongWrapperArray(int length, LongUnaryOperator map) {
        Long[] array = new Long[length];
        for (int i = 0; i < length; i++) array[i] = map.applyAsLong(this.nextLong());
        return array;
    }

    public Long[][] nextLongWrapperMatrix(int height, int width) {
        Long[][] mat = new Long[height][width];
        for (int h = 0; h < height; h++)
            for (int w = 0; w < width; w++) {
                mat[h][w] = this.nextLong();
            }
        return mat;
    }

    public Long[][] nextLongWrapperMatrix(int height, int width, LongUnaryOperator map) {
        Long[][] mat = new Long[height][width];
        for (int h = 0; h < height; h++)
            for (int w = 0; w < width; w++) {
                mat[h][w] = map.applyAsLong(this.nextLong());
            }
        return mat;
    }

    public double nextDouble() {
        return Double.parseDouble(next());
    }

    public double[] nextDoubleArray(int length) {
        double[] array = new double[length];
        for (int i = 0; i < length; i++) array[i] = this.nextDouble();
        return array;
    }

    public double[] nextDoubleArray(int length, DoubleUnaryOperator map) {
        double[] array = new double[length];
        for (int i = 0; i < length; i++) array[i] = map.applyAsDouble(this.nextDouble());
        return array;
    }

    public double[][] nextDoubleMatrix(int height, int width) {
        double[][] mat = new double[height][width];
        for (int h = 0; h < height; h++)
            for (int w = 0; w < width; w++) {
                mat[h][w] = this.nextDouble();
            }
        return mat;
    }

    public double[][] nextDoubleMatrix(int height, int width, DoubleUnaryOperator map) {
        double[][] mat = new double[height][width];
        for (int h = 0; h < height; h++)
            for (int w = 0; w < width; w++) {
                mat[h][w] = map.applyAsDouble(this.nextDouble());
            }
        return mat;
    }
}

/**
 * @see <a href = "https://codeforces.com/profile/uwi">Source code by uwi (Slightly Modified)</a>
 */
class FastWriter {
    private static final int BUF_SIZE = 1 << 13;
    private final byte[] buf = new byte[BUF_SIZE];
    private final OutputStream out;
    private int ptr = 0;

    public FastWriter() {
        this(System.out);
    }

    public FastWriter(OutputStream os) {
        this.out = os;
    }

    public FastWriter(String path) {
        try {
            this.out = new FileOutputStream(path);
        } catch (FileNotFoundException e) {
            throw new RuntimeException(path + " not found!");
        }
    }

    private static int countDigits(int l) {
        if (l >= 1000000000) return 10;
        if (l >= 100000000) return 9;
        if (l >= 10000000) return 8;
        if (l >= 1000000) return 7;
        if (l >= 100000) return 6;
        if (l >= 10000) return 5;
        if (l >= 1000) return 4;
        if (l >= 100) return 3;
        if (l >= 10) return 2;
        return 1;
    }

    private static int countDigits(long l) {
        if (l >= 1000000000000000000L) return 19;
        if (l >= 100000000000000000L) return 18;
        if (l >= 10000000000000000L) return 17;
        if (l >= 1000000000000000L) return 16;
        if (l >= 100000000000000L) return 15;
        if (l >= 10000000000000L) return 14;
        if (l >= 1000000000000L) return 13;
        if (l >= 100000000000L) return 12;
        if (l >= 10000000000L) return 11;
        if (l >= 1000000000L) return 10;
        if (l >= 100000000L) return 9;
        if (l >= 10000000L) return 8;
        if (l >= 1000000L) return 7;
        if (l >= 100000L) return 6;
        if (l >= 10000L) return 5;
        if (l >= 1000L) return 4;
        if (l >= 100L) return 3;
        if (l >= 10L) return 2;
        return 1;
    }

    private void innerFlush() {
        try {
            out.write(buf, 0, ptr);
            ptr = 0;
        } catch (IOException e) {
            throw new RuntimeException("inner flush");
        }
    }

    public void flush() {
        innerFlush();
        try {
            out.flush();
        } catch (IOException e) {
            throw new RuntimeException("flush");
        }
    }

    public FastWriter print(byte b) {
        buf[ptr++] = b;
        if (ptr == BUF_SIZE) innerFlush();
        return this;
    }

    public FastWriter print(char c) {
        return print((byte) c);
    }

    public FastWriter print(String s) {
        s.chars().forEach(c -> {
            buf[ptr++] = (byte) c;
            if (ptr == BUF_SIZE) innerFlush();
        });
        return this;
    }

    public FastWriter print(int x) {
        if (x == Integer.MIN_VALUE) {
            return print((long) x);
        }
        if (ptr + 12 >= BUF_SIZE) innerFlush();
        if (x < 0) {
            print((byte) '-');
            x = -x;
        }
        int d = countDigits(x);
        for (int i = ptr + d - 1; i >= ptr; i--) {
            buf[i] = (byte) ('0' + x % 10);
            x /= 10;
        }
        ptr += d;
        return this;
    }

    public FastWriter print(long x) {
        if (x == Long.MIN_VALUE) {
            return print(String.valueOf(x));
        }
        if (ptr + 21 >= BUF_SIZE) innerFlush();
        if (x < 0) {
            print((byte) '-');
            x = -x;
        }
        int d = countDigits(x);
        for (int i = ptr + d - 1; i >= ptr; i--) {
            buf[i] = (byte) ('0' + x % 10);
            x /= 10;
        }
        ptr += d;
        return this;
    }

    public FastWriter print(double x, int precision) {
        if (x < 0) {
            print('-');
            x = -x;
        }
        x += Math.pow(10, -precision) / 2;
        //      if(x < 0){ x = 0; }
        print((long) x).print(".");
        x -= (long) x;
        for (int i = 0; i < precision; i++) {
            x *= 10;
            print((char) ('0' + (int) x));
            x -= (int) x;
        }
        return this;
    }

    public FastWriter print(double x) {
        return print(x, 20);
    }

    public FastWriter print(Object x) {
        return print(x.toString());
    }

    public FastWriter println() {
        return print((byte) '\n');
    }

    public FastWriter println(char c) {
        return print(c).println();
    }

    public FastWriter println(int x) {
        return print(x).println();
    }

    public FastWriter println(long x) {
        return print(x).println();
    }

    public FastWriter println(double x, int precision) {
        return print(x, precision).println();
    }

    public FastWriter println(double x) {
        return println(x, 20);
    }

    public FastWriter println(String s) {
        return print(s).println();
    }

    public FastWriter println(Object x) {
        return println(x.toString());
    }

    public void printArray(char[] array, String separator) {
        int n = array.length;
        if (n == 0) {
            println();
            return;
        }
        for (int i = 0; i < n - 1; i++) {
            print(array[i]);
            print(separator);
        }
        println(array[n - 1]);
    }

    public void printArray(char[] array) {
        this.printArray(array, " ");
    }

    public void printArray(char[] array, String separator, java.util.function.UnaryOperator<Character> map) {
        int n = array.length;
        if (n == 0) {
            println();
            return;
        }
        for (int i = 0; i < n - 1; i++) {
            print(map.apply(array[i]));
            print(separator);
        }
        println(map.apply(array[n - 1]));
    }

    public void printArray(char[] array, java.util.function.UnaryOperator<Character> map) {
        this.printArray(array, " ", map);
    }

    public void printArray(int[] array, String separator) {
        int n = array.length;
        if (n == 0) {
            println();
            return;
        }
        for (int i = 0; i < n - 1; i++) {
            print(array[i]);
            print(separator);
        }
        println(array[n - 1]);
    }

    public void printArray(int[] array) {
        this.printArray(array, " ");
    }

    public void printArray(int[] array, String separator, java.util.function.IntUnaryOperator map) {
        int n = array.length;
        if (n == 0) {
            println();
            return;
        }
        for (int i = 0; i < n - 1; i++) {
            print(map.applyAsInt(array[i]));
            print(separator);
        }
        println(map.applyAsInt(array[n - 1]));
    }

    public void printArray(int[] array, java.util.function.IntUnaryOperator map) {
        this.printArray(array, " ", map);
    }

    public void printArray(long[] array, String separator) {
        int n = array.length;
        if (n == 0) {
            println();
            return;
        }
        for (int i = 0; i < n - 1; i++) {
            print(array[i]);
            print(separator);
        }
        println(array[n - 1]);
    }

    public void printArray(long[] array) {
        this.printArray(array, " ");
    }

    public void printArray(long[] array, String separator, java.util.function.LongUnaryOperator map) {
        int n = array.length;
        if (n == 0) {
            println();
            return;
        }
        for (int i = 0; i < n - 1; i++) {
            print(map.applyAsLong(array[i]));
            print(separator);
        }
        println(map.applyAsLong(array[n - 1]));
    }

    public void printArray(long[] array, java.util.function.LongUnaryOperator map) {
        this.printArray(array, " ", map);
    }

    public void printArray(double[] array, String separator) {
        int n = array.length;
        if (n == 0) {
            println();
            return;
        }
        for (int i = 0; i < n - 1; i++) {
            print(array[i]);
            print(separator);
        }
        println(array[n - 1]);
    }

    public void printArray(double[] array) {
        this.printArray(array, " ");
    }

    public void printArray(double[] array, String separator, java.util.function.DoubleUnaryOperator map) {
        int n = array.length;
        if (n == 0) {
            println();
            return;
        }
        for (int i = 0; i < n - 1; i++) {
            print(map.applyAsDouble(array[i]));
            print(separator);
        }
        println(map.applyAsDouble(array[n - 1]));
    }

    public void printArray(double[] array, java.util.function.DoubleUnaryOperator map) {
        this.printArray(array, " ", map);
    }

    public <T> void printArray(T[] array, String separator) {
        int n = array.length;
        if (n == 0) {
            println();
            return;
        }
        for (int i = 0; i < n - 1; i++) {
            print(array[i]);
            print(separator);
        }
        println(array[n - 1]);
    }

    public <T> void printArray(T[] array) {
        this.printArray(array, " ");
    }

    public <T> void printArray(T[] array, String separator, java.util.function.UnaryOperator<T> map) {
        int n = array.length;
        if (n == 0) {
            println();
            return;
        }
        for (int i = 0; i < n - 1; i++) {
            print(map.apply(array[i]));
            print(separator);
        }
        println(map.apply(array[n - 1]));
    }

    public <T> void printArray(T[] array, java.util.function.UnaryOperator<T> map) {
        this.printArray(array, " ", map);
    }

    public <T> void printCollection(Collection<T> collection, String separator) {
        boolean first = true;
        for (T c : collection) {
            if (!first) print(separator);
            first = false;
            print(c);
        }
        println();
    }

    public <T> void printCollection(Collection<T> collection) {
        this.printCollection(collection, " ");
    }

    public <T> void printCollection(Collection<T> collection, String separator, java.util.function.UnaryOperator<T> map) {
        boolean first = true;
        for (T c : collection) {
            if (!first) print(separator);
            first = false;
            print(map.apply(c));
        }
        println();
    }

    public <T> void printCollection(Collection<T> collection, java.util.function.UnaryOperator<T> map) {
        this.printCollection(collection, " ", map);
    }
}

class ActualSolution implements Runnable {

    boolean debug;
    ContestScanner sc;
    FastWriter out;


    public ActualSolution(ContestScanner sc, FastWriter out, boolean debug) {
        this.sc = sc;
        this.out = out;
        this.debug = debug;
    }

    @SuppressWarnings("unchecked")
    <T> String debugIt(T t) {
        if (t == null) return "null";
        try {
            return debugIt((Iterable<T>) t);
        } catch (ClassCastException e) {
            if (t instanceof int[]) return Arrays.toString((int[]) t);
            else if (t instanceof long[]) return Arrays.toString((long[]) t);
            else if (t instanceof char[]) return Arrays.toString((char[]) t);
            else if (t instanceof float[]) return Arrays.toString((float[]) t);
            else if (t instanceof double[]) return Arrays.toString((double[]) t);
            else if (t instanceof boolean[]) return Arrays.toString((boolean[]) t);
            try {
                return debugIt((Object[]) t);
            } catch (ClassCastException e1) {
                return t.toString();
            }
        }
    }

    <T> String debugIt(T[] arr) {
        StringBuilder ret = new StringBuilder("[");
        boolean first = true;
        for (T t : arr) {
            if (!first) ret.append(", ");
            first = false;
            ret.append(debugIt(t));
        }
        return ret.append("]").toString();
    }

    <T> String debugIt(Iterable<T> it) {
        StringBuilder ret = new StringBuilder("[");
        boolean first = true;
        for (T t : it) {
            if (!first) ret.append(", ");
            first = false;
            ret.append(debugIt(t));
        }
        return ret.append("]").toString();
    }

    void debug(Object... obj) {
        if (!debug) return;

        System.err.print("#" + Thread.currentThread().getStackTrace()[2].getLineNumber() + ": ");
        for (Object x : obj)
            System.err.print(debugIt(x) + " ");
        System.err.println();
    }


    @Override
    public void run() {
        solve();

//        int testCases = sc.nextInt();
//        for (int testCase = 1; testCase <= testCases; testCase++) {
//            debug("......");
//            debug("Test case ", testCase);
//
//            //out.print("Case #");
//            //out.print(testCase);
//            //out.print(": ");
//
//            solve();
//        }
    }

    void solve() {
    }
}