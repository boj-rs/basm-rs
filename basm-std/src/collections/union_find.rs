use alloc::{vec, vec::Vec};

#[derive(Default)]
pub struct RemUnionFind {
    up: Vec<u32>,
}

impl RemUnionFind {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_len(n: usize) -> Self {
        Self {
            up: (0..n as u32).collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.up.len()
    }

    pub fn is_empty(&self) -> bool {
        self.up.is_empty()
    }

    pub fn resize(&mut self, n: usize) {
        let mut i = self.up.len();
        self.up.resize_with(n, || {
            let v = i;
            i += 1;
            v as u32
        });
    }

    pub fn push(&mut self) {
        self.up.push(self.up.len() as u32);
    }

    pub fn try_union(&mut self, u: usize, v: usize) -> bool {
        let mut u = u;
        let mut v = v;
        while self.up[u] != self.up[v] {
            if self.up[u] > self.up[v] {
                core::mem::swap(&mut u, &mut v);
            }
            if u == self.up[u] as usize {
                self.up[u] = self.up[v];
                return true;
            }
            let up = self.up[u];
            self.up[u] = self.up[v];
            u = up as usize;
        }
        false
    }
}

#[derive(Default)]
pub struct UnionFind {
    up: Vec<u32>,
    rank: Vec<u32>,
}

impl UnionFind {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_len(n: usize) -> Self {
        Self {
            up: (0..n as u32).collect(),
            rank: vec![1; n],
        }
    }

    pub fn len(&self) -> usize {
        self.up.len()
    }

    pub fn is_empty(&self) -> bool {
        self.up.is_empty()
    }

    pub fn resize(&mut self, n: usize) {
        let mut i = self.up.len();
        self.up.resize_with(n, || {
            let v = i;
            i += 1;
            v as u32
        });
        self.rank.resize(n, 1);
    }

    pub fn push(&mut self) {
        self.up.push(self.up.len() as u32);
        self.rank.push(1);
    }

    pub fn find(&mut self, mut u: usize) -> usize {
        while u != self.up[u] as usize {
            self.up[u] = self.up[self.up[u] as usize];
            u = self.up[u] as usize;
        }
        u
    }

    pub fn union(&mut self, mut pu: usize, mut pv: usize) -> usize {
        if self.rank[pu] < self.rank[pv] {
            core::mem::swap(&mut pu, &mut pv);
        }
        self.up[pv] = pu as u32;
        if self.rank[pu] == self.rank[pv] {
            self.rank[pu] += 1;
        }
        pu
    }
}
