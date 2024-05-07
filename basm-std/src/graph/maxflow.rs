use alloc::collections::VecDeque;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::{min, max};

#[derive(Clone)]
struct Edge {
    v: usize,
    c: i64,
    f: i64,
}

/// Maximum flow solver without costs.
pub struct FlowGraph {
    adj: Vec<Vec<usize>>,
    e: Vec<Edge>
}

impl Default for FlowGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl FlowGraph {
    pub fn new() -> Self {
        Self { adj: vec![], e: vec![] }
    }
    pub fn add_edge(&mut self, u: usize, v: usize, c: i64, bidirectional: bool) {
        assert!(c >= 0);
        if self.adj.len() < u + 1 { self.adj.resize(u + 1, vec![]); }
        if self.adj.len() < v + 1 { self.adj.resize(v + 1, vec![]); }
        self.adj[u].push(self.e.len());
        self.e.push(Edge { v, c, f: 0 });
        self.adj[v].push(self.e.len());
        self.e.push(Edge { v: u, c: if bidirectional { c } else { 0 }, f: 0 });
    }
    /// Solves the maximum flow problem with source `s` and sink `t`.
    /// `s` and `t` must be distinct.
    /// 
    /// Return value: (maximum flow value, vertices in s-cut, vertices in t-cut)
    pub fn solve(&self, s: usize, t: usize) -> (i64, Vec<usize>, Vec<usize>) {
        assert!(s != t);
        let (mut adj, mut e) = (self.adj.clone(), self.e.clone());
        let bound = max(s, t);
        if bound >= adj.len() {
            adj.resize(bound + 1, vec![]);
        }
        let n = adj.len();

        let mut h = vec![0usize; n];
        h[s] = n;
        let mut p = vec![0; n];

        let mut next = vec![usize::MAX; 2*n + 1];
        let mut active = vec![vec![]; 2*n + 1];
        let mut head = usize::MAX;

        let push = |my_e: &mut [Edge], my_p: &mut [i64], u: usize, eid: usize, df: i64| -> bool {
            let v = my_e[eid].v;
            my_e[eid].f += df;
            my_p[u] -= df;
            my_e[eid ^ 1].f -= df;
            my_p[v] += df;
            my_p[v] == df && v != s && v != t
        };
        for &eid in &adj[s] {
            // push preflow from s
            let df = e[eid].c;
            if df > 0 && push(&mut e, &mut p, s, eid, df) {
                active[0].push(e[eid].v);
                head = 0;
            }
        }

        let mut e_last = vec![0usize; n];
        while head != usize::MAX {
            if let Some(u) = active[head].pop() {
                debug_assert!(p[u] > 0);
                'outer: loop {
                    for _ in 0..adj[u].len() {
                        let eii = if e_last[u] + 1 == adj[u].len() { 0 } else { e_last[u] + 1 };
                        let eid = adj[u][eii];
                        let v = e[eid].v;
                        if h[u] == h[v] + 1 {
                            let df = min(p[u], e[eid].c - e[eid].f);
                            if df > 0 {
                                if push(&mut e, &mut p, u, eid, df) {
                                    if active[h[v]].is_empty() {
                                        (next[h[u]], next[h[v]]) = (h[v], next[h[u]]);
                                    }
                                    active[h[v]].push(v);
                                }
                                if p[u] == 0 {
                                    if active[head].is_empty() {
                                        head = next[head];
                                        debug_assert!(head == usize::MAX || !active[head].is_empty());
                                    }
                                    break 'outer;
                                }
                            }
                        }
                        e_last[u] = eii;
                    }
                    h[u] = 2*n; // just needs to be large enough
                    for &eid in &adj[u] {
                        let v = e[eid].v;
                        if e[eid].c - e[eid].f > 0 {
                            h[u] = min(h[u], h[v] + 1);
                        }
                    }
                    (head, next[h[u]]) = (h[u], if active[head].is_empty() { next[head] } else { head });
                }
            } else {
                // Panic for debugging
                panic!();
            }
        }

        // Compute cut
        let mut visited = vec![false; adj.len()];
        let mut queue = VecDeque::<usize>::new();
        visited[s] = true;
        queue.push_back(s);
        while !queue.is_empty() {
            let u = *queue.front().unwrap();
            queue.pop_front();
            for &eid in &adj[u] {
                let v = e[eid].v;
                if e[eid].c - e[eid].f > 0 && !visited[v] {
                    visited[v] = true;
                    queue.push_back(v);
                }
            }
        }
        let mut s_cut = vec![];
        let mut t_cut = vec![];
        for (u, &visited_u) in visited.iter().enumerate().take(n) {
            if visited_u {
                s_cut.push(u);
            } else {
                t_cut.push(u);
            }
        }
        (p[t], s_cut, t_cut)
    }
}