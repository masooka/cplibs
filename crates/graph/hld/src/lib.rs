use std::mem;

use acl_segtree::{Monoid, Segtree};

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

    pub fn depth(&self, u: usize) -> usize {
        self.depth[u]
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

pub struct MonoidTree<M: Monoid> {
    hld: HeavyLightDecomposition,
    segtree: Segtree<M>,
}

impl<M: Monoid> MonoidTree<M> {
    pub fn new(adj: &[Vec<usize>]) -> Self {
        let hld = HeavyLightDecomposition::new(adj);
        let segtree = Segtree::new(adj.len());
        Self { hld, segtree }
    }

    pub fn set(&mut self, u: usize, x: M::S) {
        self.segtree.set(self.hld.pos[u], x);
    }

    /// Computes the product of the values on the path from `u` to `v`, assuming
    /// each node contains the value for the edge between its parent and itself.
    pub fn edge_prod(&self, u: usize, v: usize) -> M::S {
        let (prod, u, v) = self.inner_prod(u, v);
        M::binary_operation(
            &prod,
            &self.segtree.prod(self.hld.pos(u) + 1, self.hld.pos(v) + 1),
        )
    }

    /// Computes the product of the values of the nodes on the path from `u` to
    /// `v`, including `u` and `v`.
    pub fn node_prod(&self, u: usize, v: usize) -> M::S {
        let (prod, u, v) = self.inner_prod(u, v);
        M::binary_operation(
            &prod,
            &self.segtree.prod(self.hld.pos(u), self.hld.pos(v) + 1),
        )
    }

    fn inner_prod(&self, mut u: usize, mut v: usize) -> (M::S, usize, usize) {
        let mut prod = M::identity();
        while self.hld.head(u) != self.hld.head(v) {
            if self.hld.depth(self.hld.head(u)) > self.hld.depth(self.hld.head(v)) {
                mem::swap(&mut u, &mut v);
            }
            prod = M::binary_operation(
                &prod,
                &self
                    .segtree
                    .prod(self.hld.pos(self.hld.head(v)), self.hld.pos(v) + 1),
            );
            v = self.hld.parent(self.hld.head(v));
        }
        if self.hld.depth(u) > self.hld.depth(v) {
            mem::swap(&mut u, &mut v);
        }
        (prod, u, v)
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
