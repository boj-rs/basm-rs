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

pub enum MinCostFlowMode {
    /// Compute a maximum flow, and if multiple cost values are possible for the flow, choose minimum.
    MaxFlowMinCost,
    /// Compute a maximum flow, and if multiple cost values are possible for the flow, choose maximum.
    MaxFlowMaxCost,
    /// Compute a flow that minimizes cost, and if multiple flow values are possible for the cost, choose maximum.
    MinCostMaxFlow,
    /// Compute a flow that minimizes cost, and if multiple flow values are possible for the cost, choose minimum.
    MinCostMinFlow,
    /// Compute a flow that maximum cost, and if multiple flow values are possible for the cost, choose maximum.
    MaxCostMaxFlow,
    /// Compute a flow that maximum cost, and if multiple flow values are possible for the cost, choose minimum.
    MaxCostMinFlow,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
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
    /// `s` and `t` must be distinct.
    ///
    /// Returns `None` if the graph has a negative cost cycle.
    pub fn solve(&self, s: usize, t: usize, mode: MinCostFlowMode) -> Option<MinCostFlowResult> {
        assert!(s != t);
        let (mut adj, mut e) = (self.adj.clone(), self.e.clone());
        let bound = s.max(t);
        if bound >= adj.len() {
            adj.resize(bound + 1, vec![]);
        }
        let n = adj.len();

        // Negate costs if cost maximization is requested
        type Mode = MinCostFlowMode;
        let maximize_cost = matches!(
            mode,
            Mode::MaxFlowMaxCost | Mode::MaxCostMaxFlow | Mode::MaxCostMinFlow
        );
        if maximize_cost {
            for e_ent in e.iter_mut() {
                e_ent.cost = -e_ent.cost;
            }
        }

        // Step 1: Compute cost distances from the source with Bellman-Ford
        // (Dijkstra won't work on the first step because of possible negative costs)
        // * TODO: replace Bellman-Ford with SPFA for speed
        let mut s_dist = vec![(i64::MAX, usize::MAX); n]; // (dist, last_edge)
        s_dist[s] = (0, usize::MAX);
        for i in 0..n + 1 {
            let mut updated = false;
            for u in 0..n {
                if s_dist[u].0 == i64::MAX {
                    continue;
                }
                for &eid in adj[u].iter() {
                    if e[eid].capacity - e[eid].f > 0 {
                        let v = e[eid].v;
                        let new_dist = s_dist[u].0 + e[eid].cost;
                        if new_dist < s_dist[v].0 {
                            s_dist[v] = (new_dist, eid);
                            updated = true;
                        }
                    }
                }
            }
            if !updated {
                break;
            } else if i == n {
                // Bellman-Ford detected a negative cycle
                return None;
            }
        }

        // Step 2: Main loop
        let mut ans = MinCostFlowResult::default();
        let mut ans_mincost_maxflow = MinCostFlowResult::default();
        let mut ans_mincost_minflow = MinCostFlowResult::default();
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
            let mut cost = 0;
            while x.1 != usize::MAX {
                let eid = x.1;
                e[eid].f += flow;
                e[eid ^ 1].f -= flow;
                cost += e[eid].cost;
                x = s_dist_new[e[eid].u];
            }
            ans.flow += flow;
            ans.cost += cost * flow;
            if cost < 0 {
                ans_mincost_minflow = ans;
            }
            if cost <= 0 {
                ans_mincost_maxflow = ans;
            }

            // Update s_dist
            s_dist = s_dist_new;
        }

        // Select the right one
        ans = match mode {
            Mode::MaxFlowMinCost | Mode::MaxFlowMaxCost => ans,
            Mode::MinCostMaxFlow | Mode::MaxCostMaxFlow => ans_mincost_maxflow,
            Mode::MinCostMinFlow | Mode::MaxCostMinFlow => ans_mincost_minflow,
        };

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
    fn check_mcmf() {
        let mut g = MinCostFlowGraph::new();
        g.add_edge(0, 1, 10, 5, false);
        g.add_edge(1, 3, 10, -2, false);
        g.add_edge(0, 2, 10, 4, false);
        g.add_edge(2, 3, 10, -2, false);
        g.add_edge(3, 4, 13, 0, false);
        g.add_edge(0, 4, 7, -1, false);
        g.add_edge(0, 4, 3, 0, false);
        g.add_edge(0, 4, 2, 1, false);
        assert_eq!(
            Some(MinCostFlowResult { flow: 25, cost: 24 }),
            g.solve(0, 4, MinCostFlowMode::MaxFlowMinCost),
        );
        assert_eq!(
            Some(MinCostFlowResult { flow: 25, cost: 31 }),
            g.solve(0, 4, MinCostFlowMode::MaxFlowMaxCost),
        );
        assert_eq!(
            Some(MinCostFlowResult { flow: 10, cost: -7 }),
            g.solve(0, 4, MinCostFlowMode::MinCostMaxFlow),
        );
        assert_eq!(
            Some(MinCostFlowResult { flow: 7, cost: -7 }),
            g.solve(0, 4, MinCostFlowMode::MinCostMinFlow),
        );
        assert_eq!(
            Some(MinCostFlowResult { flow: 18, cost: 38 }),
            g.solve(0, 4, MinCostFlowMode::MaxCostMaxFlow),
        );
        assert_eq!(
            Some(MinCostFlowResult { flow: 15, cost: 38 }),
            g.solve(0, 4, MinCostFlowMode::MaxCostMinFlow),
        );
    }

    #[test]
    fn check_mcmf_negative_cycle_detection() {
        let mut g = MinCostFlowGraph::new();
        g.add_edge(0, 1, 10, 5, false);
        g.add_edge(1, 2, 10, 5, false);
        g.add_edge(2, 3, 10, -8, false);
        g.add_edge(3, 1, 10, 2, false);
        assert_eq!(None, g.solve(0, 3, MinCostFlowMode::MaxFlowMinCost));
        assert_eq!(
            Some(MinCostFlowResult { flow: 10, cost: 20 }),
            g.solve(0, 3, MinCostFlowMode::MaxFlowMaxCost),
        );
    }
}
