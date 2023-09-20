use acl_dsu::Dsu;

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

    dfs_lca(
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

fn dfs_lca(
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
            dfs_lca(u, adj, dsu, ancestors, visited, qs, answers);
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
