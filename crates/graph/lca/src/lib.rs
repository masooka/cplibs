use acl_dsu::Dsu;

pub struct Lca {
    up: Vec<Vec<usize>>,
    depth: Vec<usize>,
}

impl Lca {
    pub fn new(tree: &Vec<Vec<usize>>) -> Self {
        let n = tree.len();
        let log_n = (n as f64).log2().ceil() as usize;

        let mut up = vec![vec![0; log_n]; tree.len()];
        let mut depth = vec![0; n];
        dfs_lca(0, 0, 0, tree, &mut up, &mut depth);
        Self { up, depth }
    }

    /// Finds the lowest common ancestor of two vertices in O(log N).
    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        if self.depth[a] < self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }

        for k in (0..self.up[a].len()).rev() {
            if self.depth[a].saturating_sub(1 << k) >= self.depth[b] {
                a = self.up[a][k];
            }
        }

        if a == b {
            return a;
        }

        for k in (0..self.up[a].len()).rev() {
            if self.up[a][k] != self.up[b][k] {
                a = self.up[a][k];
                b = self.up[b][k];
            }
        }

        self.up[a][0]
    }
}

fn dfs_lca(
    v: usize,
    p: usize,
    height: usize,
    tree: &Vec<Vec<usize>>,
    up: &mut Vec<Vec<usize>>,
    depth: &mut Vec<usize>,
) {
    up[v][0] = p;
    depth[v] = height;

    for i in 1..up[v].len() {
        up[v][i] = up[up[v][i - 1]][i - 1];
    }

    for &child in &tree[v] {
        if child != p {
            dfs_lca(child, v, height + 1, tree, up, depth);
        }
    }
}

/// Finds the lowest common ancestor of two vertices for each query using
/// Tarjan's offline algorithm.
pub fn offline_lca(adj: &Vec<Vec<usize>>, root: usize, queries: &[(usize, usize)]) -> Vec<usize> {
    let n = adj.len();
    let mut dsu = Dsu::new(n);
    let mut ancestors = vec![0; n];
    let mut visited = vec![false; n];
    let mut answers = vec![0; queries.len()];

    let mut qs = vec![vec![]; n];
    for (i, &(u, v)) in queries.iter().enumerate() {
        qs[u].push((v, i));
        qs[v].push((u, i));
    }

    dfs_offline_lca(
        root,
        adj,
        &mut dsu,
        &mut ancestors,
        &mut visited,
        &qs,
        &mut answers,
    );

    answers
}

fn dfs_offline_lca(
    v: usize,
    adj: &Vec<Vec<usize>>,
    dsu: &mut Dsu,
    ancestors: &mut [usize],
    visited: &mut Vec<bool>,
    qs: &Vec<Vec<(usize, usize)>>,
    answers: &mut [usize],
) {
    ancestors[v] = v;
    visited[v] = true;

    for &u in &adj[v] {
        if !visited[u] {
            dfs_offline_lca(u, adj, dsu, ancestors, visited, qs, answers);
            dsu.merge(v, u);
            ancestors[dsu.leader(v)] = v;
        }
    }

    for &(u, i) in &qs[v] {
        if visited[u] {
            answers[i] = ancestors[dsu.leader(u)];
        }
    }
}
