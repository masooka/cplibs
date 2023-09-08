use std::collections::HashMap;

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
    // Check if all vertices have even degree
    if !adj_matrix
        .iter()
        .all(|row| row.iter().sum::<u32>() % 2 == 0)
    {
        return None;
    }

    let n = adj_matrix.len();

    // Create a set for each vertex to store its unvisited neighbors
    let mut neighbors: Vec<HashMap<usize, u32>> = vec![HashMap::new(); n];
    for i in 0..n {
        for j in 0..n {
            if adj_matrix[i][j] > 0 {
                neighbors[i].insert(j, adj_matrix[i][j]);
            }
        }
    }

    let mut stack: Vec<usize> = Vec::new();
    let mut cycle: Vec<usize> = Vec::new();
    stack.push(0);
    while !stack.is_empty() {
        let v = *stack.last().unwrap();

        // Find an adjacent vertex with an edge to traverse
        if let Some((&u, &count)) = neighbors[v].iter().next() {
            // Remove the edge between u and v
            if count > 1 {
                neighbors[u].insert(v, count - 1);
                neighbors[v].insert(u, count - 1);
            } else {
                neighbors[u].remove(&v);
                neighbors[v].remove(&u);
            }
            stack.push(u);
        } else {
            // If v has no neighbors, it's part of the cycle
            stack.pop();
            cycle.push(v);
        }
    }

    // Check if we've traversed all edges
    if neighbors.iter().all(|map| map.is_empty()) {
        Some(cycle)
    } else {
        None
    }
}
