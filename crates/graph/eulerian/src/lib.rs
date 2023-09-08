/// Finds an Eulerian circuit in an undirected graph represented by an adjacency
/// matrix.
///
/// # Examples
///
/// ```
/// # use eulerian::find_eulerian_circuit;
/// let mut adj_matrix = vec![
///     vec![0, 1, 1],
///     vec![1, 0, 1],
///     vec![1, 1, 0],
/// ];
/// assert!(find_eulerian_circuit(&mut adj_matrix).is_some());
/// ```
pub fn find_eulerian_circuit(adj_matrix: &[Vec<u32>]) -> Option<Vec<usize>> {
    #[derive(Clone, Copy, Default)]
    struct Node {
        value: u32,
        prev: Option<usize>,
        next: Option<usize>,
    }

    // Check if all vertices have even degree
    if !adj_matrix
        .iter()
        .all(|row| row.iter().sum::<u32>() % 2 == 0)
    {
        return None;
    }

    let n = adj_matrix.len();

    // Create a list for each vertex to store its unvisited neighbors
    let mut neighbors = vec![vec![Node::default(); n]; n];
    let mut heads = vec![None; n];
    for i in 0..n {
        let mut prev: Option<usize> = None;
        for j in 0..n {
            if adj_matrix[i][j] > 0 {
                neighbors[i][j].value = adj_matrix[i][j];
                if let Some(prev) = prev {
                    neighbors[i][prev].next = Some(j);
                    neighbors[i][j].prev = Some(prev);
                } else {
                    heads[i] = Some(j);
                }
                prev = Some(j);
            }
        }
    }

    let mut stack: Vec<usize> = Vec::new();
    let mut cycle: Vec<usize> = Vec::new();
    stack.push(0);
    while !stack.is_empty() {
        let v = *stack.last().unwrap();

        // Find an adjacent vertex with an edge to traverse
        if let Some(u) = heads[v] {
            neighbors[v][u].value -= 1;
            neighbors[u][v].value -= 1;
            if neighbors[v][u].value == 0 {
                if let Some(prev) = neighbors[v][u].prev {
                    neighbors[v][prev].next = neighbors[v][u].next;
                } else {
                    heads[v] = neighbors[v][u].next;
                }
                if let Some(next) = neighbors[v][u].next {
                    neighbors[v][next].prev = neighbors[v][u].prev;
                }

                if let Some(prev) = neighbors[u][v].prev {
                    neighbors[u][prev].next = neighbors[u][v].next;
                } else {
                    heads[u] = neighbors[u][v].next;
                }
                if let Some(next) = neighbors[u][v].next {
                    neighbors[u][next].prev = neighbors[u][v].prev;
                }
            }
            stack.push(u);
        } else {
            // If v has no neighbors, it's part of the cycle
            stack.pop();
            cycle.push(v);
        }
    }

    // Check if we've traversed all edges
    if heads.iter().all(|head| head.is_none()) {
        Some(cycle)
    } else {
        None
    }
}
