class DistLCA {
    int[] depth;
    int[][] up;
    int log;

    public DistLCA(ArrayList<Integer>[] adj, int log) {
        int n = adj.length;
        this.log = log;
        depth = new int[n];
        Arrays.fill(depth, -1);
        up = new int[log + 1][n];

        bfs(adj);
        for (int i = 1; i <= log; i++) {
            for (int j = 0; j < n; j++)
                up[i][j] = up[i - 1][up[i - 1][j]];
        }
    }

    public DistLCA(ArrayList<Integer>[] adj) {
        this(adj, 31 - Integer.numberOfLeadingZeros(adj.length));
    }

    private void bfs(ArrayList<Integer>[] adj) {
        depth[0] = 0;
        up[0][0] = 0;

        Queue<Integer> q = new ArrayDeque<>();
        q.add(0);
        while (!q.isEmpty()) {
            int u = q.poll();

            for (int v : adj[u]) {
                if (depth[v] != -1)
                    continue;

                depth[v] = depth[u] + 1;
                up[0][v] = u;
                q.add(v);
            }
        }
    }

    public int lift(int u, int k) {
        for (int i = 0; i <= log; i++) {
            if ((k >> i & 1) == 1)
                u = up[i][u];
        }
        return u;
    }

    public int lca(int u, int v) {
        if (depth[u] < depth[v]) {
            int temp = u;
            u = v;
            v = temp;
        }

        u = lift(u, depth[u] - depth[v]);
        if (u == v)
            return u;
        for (int i = log; i >= 0; i--) {
            if (up[i][u] != up[i][v]) {
                u = up[i][u];
                v = up[i][v];
            }
        }
        return up[0][u];
    }

    public int dist(int u, int v) {
        int lca = lca(u, v);
        return (depth[u] - depth[lca]) + (depth[v] - depth[lca]);
    }
}