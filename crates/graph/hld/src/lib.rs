pub struct HeavyLightDecomposition {
    parent: Vec<usize>,
    depth: Vec<usize>,
    heavy: Vec<usize>,
    head: Vec<usize>,
    pos: Vec<usize>,
    len: usize,
}

impl HeavyLightDecomposition {
    pub fn new(adj: &[Vec<usize>]) -> Self {
        let n = adj.len();
        let mut hld = Self {
            parent: vec![0; n],
            depth: vec![0; n],
            heavy: vec![usize::MAX; n],
            head: vec![0; n],
            pos: vec![0; n],
            len: 0,
        };
        hld.dfs(adj, 0);
        hld.decompose(adj, 0, 0);
        hld
    }

    pub fn head(&self, u: usize) -> usize {
        self.head[u]
    }

    pub fn parent(&self, u: usize) -> usize {
        self.parent[u]
    }

    pub fn pos(&self, u: usize) -> usize {
        self.pos[u]
    }

    /// Finds the heavy path from `u` and returns the size of the subtree rooted
    /// at `u`.
    fn dfs(&mut self, adj: &[Vec<usize>], u: usize) -> usize {
        let mut size = 1;
        let mut max_subtree_size = 0;
        for &v in &adj[u] {
            if v == self.parent[u] {
                continue;
            }
            self.parent[v] = u;
            self.depth[v] = self.depth[u] + 1;
            let subtree_size = self.dfs(adj, v);
            size += subtree_size;
            if max_subtree_size < subtree_size {
                max_subtree_size = subtree_size;
                self.heavy[u] = v;
            }
        }
        size
    }

    fn decompose(&mut self, adj: &[Vec<usize>], u: usize, head: usize) {
        self.head[u] = head;
        self.pos[u] = self.len;
        self.len += 1;
        if self.heavy[u] != usize::MAX {
            self.decompose(adj, self.heavy[u], head);
        }
        for &v in &adj[u] {
            if v != self.parent[u] && v != self.heavy[u] {
                self.decompose(adj, v, v);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn single_path() {
        let adj = vec![vec![1], vec![2], vec![3], vec![4], vec![5], vec![]];
        let hld = super::HeavyLightDecomposition::new(&adj);
        assert_eq!(hld.head, vec![0, 0, 0, 0, 0, 0]);
        assert_eq!(hld.pos, vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(hld.heavy, vec![1, 2, 3, 4, 5, usize::MAX]);
    }
}
