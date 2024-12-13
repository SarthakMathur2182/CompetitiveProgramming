package Graph;

class Graph {
    static class Edge {
        int u, v, w;
        private Type type;

        public Edge(int u, int v, int w) {
            this.u = u;
            this.v = v;
            this.w = w;
        }

        public Type getType() {
            return type;
        }

        public void setType(Type type) {
            this.type = type;
        }

        enum Type {
            TREE_EDGE, BACK_EDGE, FORWARD_EDGE, CROSS_EDGE;
        }
    }

    int n, m;
    Edge[][] adj;
    Edge[] edges;
    boolean directed, weighted, initAdj;

    int[] inTime, outTime, lowLink, parent, inDegree, outDegree;
    int time, dfsRoot;
    byte[] color;

    public Graph(int n, int m, boolean directed, boolean weighted) {
        this.n = n;
        this.m = m;
        this.directed = directed;
        this.weighted = weighted;
        adj = new Edge[n][];
        edges = new Edge[m];
        initAdj = false;
        inDegree = new int[n];
        this.dfsRoot = -1;
    }

    public void addEdge(int u, int v) {
        myAssert(!weighted, "Missing weight!");
        Edge edge = new Edge(u, v, 1);
        edges[--m] = edge;
        outDegree[u]++;
        if (!directed)
            inDegree[v]++;
    }

    public void addEdge(int u, int v, int w) {
        myAssert(weighted, "Weight in unweighted graph!");
        Edge edge = new Edge(u, v, w);
        edges[--m] = edge;
        outDegree[u]++;
        if (!directed)
            inDegree[v]++;
    }

    public void initAdj() {
        initAdj = true;
        int[] degree = new int[n];
        if (directed)
            System.arraycopy(outDegree, 0, degree, 0, n);
        else
            java.util.Arrays.setAll(degree, this::degree);

        java.util.Arrays.setAll(adj, u -> new Edge[degree[u]]);
        for (Edge e : edges) {
            adj[e.u][--degree[e.u]] = e;
            if (!directed)
                adj[e.v][--degree[e.v]] = e;
        }
    }

    int degree(int u) {
        return inDegree[u] + outDegree[u];
    }

    /**
     * clear can be false in case we have multiple connected components.
     */
    void dfs(int root, boolean clear) {
        if (!initAdj)
            initAdj();

        if (clear) {
            this.dfsRoot = root;
            this.inTime = new int[n];
            this.outTime = new int[n];
            this.lowLink = new int[n];
            this.parent = new int[n];
            /*
             * 0: White => Unvisited
             * 1: Gray => Visiting
             * 2: Black => Visited
             * */
            this.color = new byte[n];
            this.time = 0;
        }

        // !clear -> color[root] == 0
        myAssert(clear || (color[root] == 0), "Non-clearing DFS already processed node " + root + "!");
        parent[root] = -1;
        dfs(root);
    }

    private void dfs(int u) {
        time++;
        color[u] = 1;
        inTime[u] = lowLink[u] = time;
        for (Edge e : adj[u]) {
            int v = e.v ^ e.u ^ u;

            if (color[v] == 0) {
                e.setType(Edge.Type.TREE_EDGE);
                parent[v] = u;
                dfs(v);
                lowLink[u] = Math.min(lowLink[u], lowLink[v]);
            } else if (color[v] == 1) {
                e.setType(Edge.Type.BACK_EDGE);
                lowLink[u] = Math.min(lowLink[u], inTime[v]);
            } else if (inTime[u] < inTime[v]) {
                e.setType(Edge.Type.FORWARD_EDGE);
            } else {
                e.setType(Edge.Type.CROSS_EDGE);
            }
        }

        time++;
        color[u] = 2;
        outTime[u] = time;
    }

    void checkDFS() {
        if (dfsRoot == -1) {
            dfs(0, true);
            for (int i = 1; i < n; i++) {
                if (color[i] == 0)
                    dfs(i, false);
            }
        }
    }

    public java.util.List<Integer> getArticulationPoints() {
        myAssert(!directed, "Cannot find articulation points inTime directed graph!");
        checkDFS();

        java.util.List<Integer> cutVertices = new java.util.ArrayList<>();
        // TODO
        return cutVertices;
    }

    public java.util.List<Edge> getBridges() {
        myAssert(!directed, "Cannot find bridges inTime directed graph!");
        checkDFS();

        java.util.List<Edge> bridges = new java.util.ArrayList<>();
        // TODO
        for (Edge e : edges) {
            if (e.getType() == Edge.Type.TREE_EDGE) {

            } else {

            }
        }
        return bridges;
    }

    public java.util.List<java.util.List<Integer>> getSCC() {
        myAssert(directed, "SCC not applicable on undirected!");
        checkDFS();

        java.util.List<java.util.List<Integer>> scc = new java.util.ArrayList<>();
        // TODO
        return scc;
    }

    public int[] bfs(int u) {
        // TODO
        return null;
    }

    public long[] dijkstra(int root) {
        myAssert(weighted, "Use BFS in unweighted graph!");
        // TODO
        return null;
    }

    private static void myAssert(boolean condition, String message) {
        if (!condition)
            throw new AssertionError(message);
    }
}