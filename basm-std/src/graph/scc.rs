use alloc::vec;
use alloc::vec::Vec;

/// Strongly Connected Components solver using Kosaraju's algorithm.
/// Uses `alloc` so it works in no_std environments with an allocator.
pub struct SCCGraph {
    /// Adjacency list of the graph
    adj: Vec<Vec<usize>>,
    /// Adjacency list of the reversed graph
    radj: Vec<Vec<usize>>,
    /// Number of edges added
    edge_count: usize,
}

impl Default for SCCGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl SCCGraph {
    /// Create a new, empty SCC graph.
    pub fn new() -> Self {
        SCCGraph {
            adj: vec![],
            radj: vec![],
            edge_count: 0,
        }
    }

    /// Read-only access to the original adjacency lists.
    pub fn adj_list(&self) -> &Vec<Vec<usize>> {
        &self.adj
    }

    /// Add a directed edge from `u` to `v`.
    /// Automatically resizes internal storage to accommodate higher node indices.
    pub fn add_edge(&mut self, u: usize, v: usize) {
        let required = core::cmp::max(u, v) + 1;
        if self.adj.len() < required {
            self.adj.resize(required, Vec::new());
            self.radj.resize(required, Vec::new());
        }
        self.adj[u].push(v);
        self.radj[v].push(u);
        self.edge_count += 1;
    }

    /// Computes strongly connected components using Kosaraju's two-pass algorithm.
    ///
    /// Returns `(component_count, comp_id, components)`:
    /// - `component_count`: number of SCCs found.
    /// - `comp_id[u]`: SCC index of node `u` in `[0..component_count)`.
    /// - `components`: vector of SCCs, each listing its member nodes.
    pub fn solve(&self) -> (usize, Vec<usize>, Vec<Vec<usize>>) {
        let n = self.adj.len();
        // 1) First pass: DFS on the original graph to compute finish order.
        let mut visited = vec![false; n];
        let mut order = Vec::with_capacity(n);

        fn dfs1(u: usize, adj: &Vec<Vec<usize>>, visited: &mut [bool], order: &mut Vec<usize>) {
            visited[u] = true;
            for &v in &adj[u] {
                if !visited[v] {
                    dfs1(v, adj, visited, order);
                }
            }
            order.push(u);
        }

        for u in 0..n {
            if !visited[u] {
                dfs1(u, &self.adj, &mut visited, &mut order);
            }
        }

        // 2) Second pass: DFS on reversed graph in decreasing finish time.
        visited.fill(false);
        let mut comp_id = vec![0; n];
        let mut components: Vec<Vec<usize>> = Vec::new();
        let mut cid = 0;

        fn dfs2(
            u: usize,
            radj: &Vec<Vec<usize>>,
            visited: &mut [bool],
            comp_id: &mut [usize],
            cid: usize,
            comp: &mut Vec<usize>,
        ) {
            visited[u] = true;
            comp_id[u] = cid;
            comp.push(u);
            for &v in &radj[u] {
                if !visited[v] {
                    dfs2(v, radj, visited, comp_id, cid, comp);
                }
            }
        }

        // Process nodes in reverse finish order
        for &u in order.iter().rev() {
            if !visited[u] {
                let mut comp = Vec::new();
                dfs2(u, &self.radj, &mut visited, &mut comp_id, cid, &mut comp);
                components.push(comp);
                cid += 1;
            }
        }

        (cid, comp_id, components)
    }
}

#[cfg(test)]
mod tests {
    use super::SCCGraph;

    #[test]
    fn test_empty_graph() {
        let graph = SCCGraph::new();
        let (count, ids, comps) = graph.solve();
        assert_eq!(count, 0);
        assert!(ids.is_empty());
        assert!(comps.is_empty());
    }

    #[test]
    fn test_single_node() {
        let mut graph = SCCGraph::new();
        graph.add_edge(0, 0);
        let (count, ids, comps) = graph.solve();
        assert_eq!(count, 1);
        assert_eq!(ids, vec![0]);
        assert_eq!(comps, vec![vec![0]]);
    }

    #[test]
    fn test_two_nodes_no_edge() {
        let mut graph = SCCGraph::new();
        graph.add_edge(1, 1);
        let (count, _ids, comps) = graph.solve();
        assert_eq!(count, 2);
        let mut sorted: Vec<_> = comps
            .iter()
            .map(|c| {
                let mut v = c.clone();
                v.sort();
                v
            })
            .collect();
        sorted.sort();
        assert_eq!(sorted, vec![vec![0], vec![1]]);
    }

    #[test]
    fn test_simple_cycle() {
        let mut graph = SCCGraph::new();
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);
        let (count, ids, comps) = graph.solve();
        assert_eq!(count, 1);
        let mut comp = comps[0].clone();
        comp.sort();
        assert_eq!(comp, vec![0, 1, 2]);
        assert_eq!(ids, vec![0, 0, 0]);
    }

    #[test]
    fn test_two_sccs() {
        let mut graph = SCCGraph::new();
        graph.add_edge(0, 1);
        graph.add_edge(1, 0);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 2);
        graph.add_edge(1, 2);

        let (count, ids, comps) = graph.solve();
        assert_eq!(count, 2);
        let mut sizes: Vec<_> = comps.iter().map(Vec::len).collect();
        sizes.sort();
        assert_eq!(sizes, vec![2, 3]);
        for &u in &[0, 1] {
            assert_eq!(ids[u], ids[0]);
        }
        for &u in &[2, 3, 4] {
            assert_eq!(ids[u], ids[2]);
        }
    }
}