use alloc::collections::VecDeque;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::{max, min};

#[derive(Clone)]
struct Edge {
    v: usize,
    c: i64,
    f: i64,
}

/// Maximum flow solver without costs.
pub struct FlowGraph {
    adj: Vec<Vec<usize>>,
    e: Vec<Edge>,
    edge_count: usize,
}

impl Default for FlowGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl FlowGraph {
    pub fn new() -> Self {
        Self {
            adj: vec![],
            e: vec![],
            edge_count: 0,
        }
    }
    pub fn add_edge(&mut self, u: usize, v: usize, c: i64, bidirectional: bool) {
        assert!(c >= 0);
        if self.adj.len() < u + 1 {
            self.adj.resize(u + 1, vec![]);
        }
        if self.adj.len() < v + 1 {
            self.adj.resize(v + 1, vec![]);
        }
        self.adj[u].push(self.e.len());
        self.e.push(Edge { v, c, f: 0 });
        self.adj[v].push(self.e.len());
        self.e.push(Edge {
            v: u,
            c: if bidirectional { c } else { 0 },
            f: 0,
        });
        self.edge_count += 1;
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
        let m = self.edge_count;

        let mut h = vec![0usize; n];
        h[s] = n;
        let mut p = vec![0; n];

        let mut next = vec![usize::MAX; 2 * n + 1];
        let mut active = vec![vec![]; 2 * n + 1];
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

        // Phase I: find maximum flow to sink
        let mut relabel_count = 0;
        let mut e_last = vec![0usize; n];
        'outermost: while head != usize::MAX {
            if let Some(u) = active[head].pop() {
                debug_assert!(p[u] > 0);
                'outer: loop {
                    for _ in 0..adj[u].len() {
                        let eii = if e_last[u] + 1 == adj[u].len() {
                            0
                        } else {
                            e_last[u] + 1
                        };
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
                                        debug_assert!(
                                            head == usize::MAX || !active[head].is_empty()
                                        );
                                    }
                                    break 'outer;
                                }
                            }
                        }
                        e_last[u] = eii;
                    }
                    h[u] = 2 * n; // just needs to be large enough
                    for &eid in &adj[u] {
                        let v = e[eid].v;
                        if e[eid].c - e[eid].f > 0 {
                            h[u] = min(h[u], h[v] + 1);
                        }
                    }
                    (head, next[h[u]]) = (
                        h[u],
                        if active[head].is_empty() {
                            next[head]
                        } else {
                            head
                        },
                    );
                    relabel_count += 1;
                }
            } else {
                // Panic for debugging
                panic!();
            }
            if relabel_count >= m {
                h.fill(n);
                h[t] = 0;
                for active_u in active.iter_mut() {
                    active_u.clear();
                }

                // Perform global relabeling.
                // This takes O(m), hence if we perform global relabeling every m relabels
                //   the amortized cost of global relabeling is O(1).
                let mut visited = vec![false; n];
                let mut queue = VecDeque::<usize>::new();
                visited[t] = true;
                queue.push_back(t);
                while !queue.is_empty() {
                    let u = queue.pop_front().unwrap();
                    for &eid in &adj[u] {
                        let v = e[eid].v;
                        if e[eid ^ 1].c - e[eid ^ 1].f > 0 && !visited[v] && v != s && v != t {
                            visited[v] = true;
                            h[v] = h[u] + 1;
                            queue.push_back(v);
                        }
                    }
                }

                for u in 0..n {
                    if u != s && u != t && p[u] > 0 {
                        active[h[u]].push(u);
                    }
                }
                head = usize::MAX;
                let mut lowest = usize::MAX;
                for h in 0..=2 * n {
                    if !active[h].is_empty() {
                        next[h] = head;
                        head = h;
                        lowest = min(lowest, h);
                    }
                }
                if lowest == usize::MAX || lowest >= n {
                    break 'outermost;
                }
                relabel_count = 0;
            }
        }

        // Phase II: send excesses back to source via doing a DFS from the source
        fn dfs_phase2(
            adj: &Vec<Vec<usize>>,
            e: &mut [Edge],
            p: &mut [i64],
            h: &[usize],
            visited: &mut [bool],
            stack: &mut Vec<usize>,
            u: usize,
        ) {
            if visited[u] {
                // Back-edge; cancel the minimal flow along the cycle
                let mut df = i64::MAX;
                for &eid in stack.iter().rev() {
                    df = min(df, e[eid].f);
                    if e[eid ^ 1].v == u {
                        break;
                    }
                }
                for &eid in stack.iter().rev() {
                    e[eid].f -= df;
                    e[eid ^ 1].f += df;
                    if e[eid ^ 1].v == u {
                        break;
                    }
                }
            } else {
                // Continue on DFS
                visited[u] = true;
                for &eid in &adj[u] {
                    let v = e[eid].v;
                    if e[eid].f > 0 && h[v] >= adj.len() {
                        stack.push(eid);
                        dfs_phase2(adj, e, p, h, visited, stack, v);
                        stack.pop();
                        let df = e[eid].f;
                        e[eid].f -= df;
                        e[eid ^ 1].f += df;
                        p[u] += df;
                        p[v] -= df;
                    }
                }
            }
        }
        let mut visited = vec![false; n];
        let mut stack = vec![];
        dfs_phase2(&adj, &mut e, &mut p, &h, &mut visited, &mut stack, s);

        // Compute cut
        let mut visited = vec![false; n];
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
