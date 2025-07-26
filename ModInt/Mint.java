/**
 * Initialize {@link #mod} before using.
 */
final class Mint { // 1000000007 998244353
    public static long mod = 0;
    public static boolean modIsPrime = true;
    private final long val;

    public static final Mint ZERO = new Mint(0L);
    public static final Mint ONE = new Mint(1L);

    public static long norm(long val) {
        return (val %= mod) < 0 ? val + mod : val;
    }

    public static long norm(Integer val) {
        return norm(val.longValue());
    }

    public Mint(long val) {
        this.val = norm(val);
    }

    public Mint() {
        this(0);
    }

    public Mint(Mint arg) {
        this(arg.val);
    }

    public Mint(Integer arg) {
        this(arg.longValue());
    }

    public long get() {
        return val;
    }

    public Mint add(long arg) {
        return new Mint(this.val + norm(arg));
    }

    public Mint add(Mint arg) {
        return add(arg.val);
    }

    public Mint add(Integer arg) {
        return add(arg.longValue());
    }

    public Mint add(long... args) {
        Mint sum = this;
        for (long a : args)
            sum = sum.add(a);
        return sum;
    }

    public Mint add(Mint... args) {
        Mint sum = this;
        for (Mint a : args)
            sum = sum.add(a);
        return sum;
    }

    public Mint add(Integer... args) {
        Mint sum = this;
        for (Integer a : args)
            sum = sum.add(a);
        return sum;
    }

    public Mint sub(long arg) {
        return new Mint(val - norm(arg));
    }

    public Mint sub(Mint arg) {
        return sub(arg.val);
    }

    public Mint sub(Integer arg) {
        return sub(arg.longValue());
    }

    public Mint mul(long arg) {
        return new Mint(this.val * norm(arg));
    }

    public Mint mul(Mint arg) {
        return mul(arg.val);
    }

    public Mint mul(Integer arg) {
        return mul(arg.longValue());
    }

    public Mint mul(long... args) {
        Mint product = this;
        for (long a : args)
            product = product.mul(norm(a));
        return product;
    }

    public Mint mul(Mint... args) {
        Mint product = this;
        for (Mint a : args)
            product = product.mul(a);
        return product;
    }

    public Mint mul(Integer... args) {
        Mint product = this;
        for (Integer a : args)
            product = product.mul(norm(a));
        return product;
    }

    public Mint div(Mint arg) {
        return mul(arg.inv());
    }

    public Mint div(long arg) {
        return div(new Mint(arg));
    }

    public Mint div(Integer arg) {
        return div(new Mint(arg));
    }

    public Mint inv() {
        if (!modIsPrime)
            throw new ArithmeticException(val + " cannot have inverse with mod " + mod + "!");
        return pow(mod - 2);
    }

    public Mint pow(long arg) {
        if (arg < 0)
            return pow(-arg).inv();
        Mint pow = Mint.ONE;
        Mint temp = this;
        while (arg > 0) {
            if ((arg & 1) == 1)
                pow = pow.mul(temp);
            temp = temp.mul(temp);
            arg = arg >> 1;
        }
        return pow;
    }

    public Mint pow(Mint arg) {
        return pow(arg.val);
    }

    public Mint pow(Integer arg) {
        return pow(arg.longValue());
    }

    @Override
    public boolean equals(Object o) {
        if (this == o)
            return true;
        if (o == null || getClass() != o.getClass())
            return false;
        Mint mint = (Mint) o;
        return val == mint.val;
    }

    @Override
    public String toString() {
        return Long.toString(val);
    }
}

class Combinatorics {

    Mint[] factorial, inverseFactorial;

    public Combinatorics(int n) {
        factorial = new Mint[n + 10];
        inverseFactorial = new Mint[n + 10];
        precompute(n + 10);
    }

    public Combinatorics() {
        this(1000000);
    }

    private void precompute(int n) {
        factorial[0] = Mint.ONE;
        for (int i = 1; i < n; i++)
            factorial[i] = factorial[i - 1].mul(i);

        inverseFactorial[n - 1] = factorial[n - 1].inv();
        for (int i = n - 2; i >= 0; i--)
            inverseFactorial[i] = inverseFactorial[i + 1].mul(i + 1);
    }

    public Mint fact(int n) {
        return factorial[n];
    }

    public Mint invFact(int n) {
        if (n < 0)
            return Mint.ZERO;
        return inverseFactorial[n];
    }

    public Mint C(int n, int r) {
        if (r < 0 || r > n)
            return Mint.ZERO;
        return factorial[n].mul(inverseFactorial[r], inverseFactorial[n - r]);
    }

    public static long C(int n, int r, Combinatorics comb, boolean modular) {
        if (modular)
            return comb.C(n, r).get();

        if (r > n - r)
            r = n - r;
        long ans = 1;
        for (int i = r + 1; i <= n; i++) {
            ans *= i;
            ans /= i - r;
        }
        return ans;
    }

    public static long C(int n, int r, Combinatorics comb) {
        return comb == null ? C(n, r, null, false) : C(n, r, comb, true);
    }

    public Mint P(int n, int r) {
        if (r > n || r < 0)
            return Mint.ZERO;
        return factorial[n].mul(inverseFactorial[n - r]);
    }

    public static long P(int n, int r, Combinatorics comb, boolean modular) {
        if (modular)
            return comb.P(n, r).get();
        long ans = 1;
        for (int i = n - r + 1; i <= n; i++)
            ans *= i;
        return ans;
    }

    public static long P(int n, int r, Combinatorics comb) {
        return comb == null ? P(n, r, null, false) : P(n, r, comb, true);
    }

    public Mint inv(int n) {
        return inverseFactorial[n].mul(factorial[n - 1]);
    }

    @Override
    public String toString() {
        return "Combinatorics till " + (factorial.length - 1);
    }
}