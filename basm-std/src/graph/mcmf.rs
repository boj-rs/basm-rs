use alloc::collections::BinaryHeap;
use alloc::{vec, vec::Vec};
use core::cmp::Reverse;

#[derive(Clone)]
struct Edge {
    u: usize,
    v: usize,
    capacity: i64,
    cost: i64,
    f: i64,
}

/// (Maximum) flow solver with costs.
pub struct MinCostFlowGraph {
    adj: Vec<Vec<usize>>,
    e: Vec<Edge>,
    edge_count: usize,
}

#[derive(Debug, PartialEq)]
pub struct MinCostFlowResult {
    pub flow: i64,
    pub cost: i64,
}

impl Default for MinCostFlowGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl MinCostFlowGraph {
    pub fn new() -> Self {
        Self {
            adj: vec![],
            e: vec![],
            edge_count: 0,
        }
    }
    pub fn add_edge(&mut self, u: usize, v: usize, capacity: i64, cost: i64, bidirectional: bool) {
        assert!(capacity >= 0);
        if self.adj.len() < u + 1 {
            self.adj.resize(u + 1, vec![]);
        }
        if self.adj.len() < v + 1 {
            self.adj.resize(v + 1, vec![]);
        }
        self.adj[u].push(self.e.len());
        self.e.push(Edge {
            u,
            v,
            capacity,
            cost,
            f: 0,
        });
        self.adj[v].push(self.e.len());
        self.e.push(Edge {
            u: v,
            v: u,
            capacity: if bidirectional { capacity } else { 0 },
            cost: -cost,
            f: 0,
        });
        self.edge_count += 1;
    }
    /// Computes maximum flow that has the minimum cost (the maximum cost if `maximize_cost` is true).
    ///
    /// Returns `None` if the graph has a negative cost cycle.
    pub fn solve(&self, s: usize, t: usize, maximize_cost: bool) -> Option<MinCostFlowResult> {
        assert!(s != t);
        let (mut adj, mut e) = (self.adj.clone(), self.e.clone());
        let bound = s.max(t);
        if bound >= adj.len() {
            adj.resize(bound + 1, vec![]);
        }
        let n = adj.len();

        // Negate costs if cost maximization is requested
        if maximize_cost {
            for e_ent in e.iter_mut() {
                e_ent.cost = -e_ent.cost;
            }
        }

        // Step 1: Compute all-pair costs with Bellman-Ford
        let mut s_dist = vec![(i64::MAX, usize::MAX); n]; // (dist, last_edge)
        s_dist[s] = (0, usize::MAX);
        for _ in 0..n {
            for u in 0..n {
                if s_dist[u].0 == i64::MAX {
                    continue;
                }
                for &eid in adj[u].iter() {
                    if e[eid].capacity - e[eid].f > 0 {
                        let v = e[eid].v;
                        s_dist[v] = s_dist[v].min((s_dist[u].0 + e[eid].cost, eid));
                    }
                }
            }
        }

        // Step 2: Main loop
        let mut ans = MinCostFlowResult { flow: 0, cost: 0 };
        loop {
            // Run Dijkstra with weights adjusted as w'[u->v] = s_dist[u] + w[u->v] - s_dist[v]
            // (i.e., Johnson's algorithm; see section 9.4 of Jeff Erickson's Algorithm book (2019) for details)
            let mut s_dist_new = vec![(i64::MAX, usize::MAX); n]; // (dist, last_edge)
            s_dist_new[s] = (s_dist[s].0, usize::MAX);
            let mut pq = BinaryHeap::new();
            pq.push((Reverse(s_dist_new[s].0), s));
            while let Some(x) = pq.pop() {
                if x.0.0 != s_dist_new[x.1].0 {
                    // Entry is stale
                    continue;
                }
                let u = x.1;
                for &eid in adj[u].iter() {
                    if e[eid].capacity - e[eid].f > 0 {
                        let v = e[eid].v;
                        let new_dist = x.0.0 + (s_dist[u].0 + e[eid].cost - s_dist[v].0);
                        if new_dist < s_dist_new[v].0 {
                            s_dist_new[v] = (new_dist, eid);
                            pq.push((Reverse(s_dist_new[v].0), v));
                        }
                    }
                }
            }
            for u in 0..n {
                if s_dist_new[u].0 != i64::MAX {
                    s_dist_new[u].0 -= s_dist[s].0 - s_dist[u].0;
                }
            }

            // Terminate if t is not reachable
            let mut x = s_dist_new[t];
            if x.0 == i64::MAX {
                break;
            }

            // Augment if t is reachable
            let mut flow = i64::MAX;
            while x.1 != usize::MAX {
                let eid = x.1;
                flow = flow.min(e[eid].capacity - e[eid].f);
                x = s_dist_new[e[eid].u];
            }
            x = s_dist_new[t];
            while x.1 != usize::MAX {
                let eid = x.1;
                e[eid].f += flow;
                e[eid ^ 1].f -= flow;
                ans.cost += e[eid].cost * flow;
                x = s_dist_new[e[eid].u];
            }
            ans.flow += flow;

            // Update s_dist
            s_dist = s_dist_new;
        }

        // Negate cost if cost maximization is requested
        if maximize_cost {
            ans.cost = -ans.cost;
        }

        Some(ans)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mcmf() {
        let mut g = MinCostFlowGraph::new();
        g.add_edge(0, 1, 10, 5, false);
        g.add_edge(1, 3, 10, -2, false);
        g.add_edge(0, 2, 10, 4, false);
        g.add_edge(2, 3, 10, -2, false);
        g.add_edge(3, 4, 13, 0, false);
        assert_eq!(
            Some(MinCostFlowResult { flow: 13, cost: 29 }),
            g.solve(0, 4, false),
        );
        assert_eq!(
            Some(MinCostFlowResult { flow: 13, cost: 36 }),
            g.solve(0, 4, true),
        );
    }
}