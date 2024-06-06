use alloc::{vec, vec::Vec};

#[derive(Default)]
pub struct RemUnionFind {
    up: Vec<u32>,
    connected_component_count: usize,
}

impl RemUnionFind {
    /// Creates a new instance of `RemUnionFind` with length `n`.
    /// 
    /// Pass `n = 0` if an empty instance is desired.
    pub fn new(n: usize) -> Self {
        Self {
            up: (0..n as u32).collect(),
            connected_component_count: n,
        }
    }

    /// Returns the number of elements in the current instance.
    pub fn len(&self) -> usize {
        self.up.len()
    }

    /// Returns `true` if the current instance contains no elements.
    pub fn is_empty(&self) -> bool {
        self.up.is_empty()
    }

    /// Alias for `connected_component_count`.
    pub fn cc_count(&self) -> usize {
        self.connected_component_count()
    }

    /// Returns the number of connected components.
    pub fn connected_component_count(&self) -> usize {
        self.connected_component_count
    }

    /// Resizes to increase (or keep) the number of elements.
    /// 
    /// A runtime error will occur if `n` is smaller than `self.len()`.
    pub fn resize(&mut self, n: usize) {
        assert!(n >= self.len());
        self.connected_component_count += n - self.len();
        let mut i = self.up.len();
        self.up.resize_with(n, || {
            let v = i;
            i += 1;
            v as u32
        });
    }

    /// Increases the number of elements by exactly one.
    pub fn push(&mut self) {
        self.up.push(self.up.len() as u32);
    }

    /// Tries to unite `u` and `v`.
    /// 
    /// Returns `true` if a new union is created, `false` otherwise.
    /// 
    /// Both `u` and `v` should be strictly less than `self.len()`.
    /// A runtime error will occur otherwise.
    pub fn try_union(&mut self, u: usize, v: usize) -> bool {
        let mut u = u;
        let mut v = v;
        while self.up[u] != self.up[v] {
            if self.up[u] > self.up[v] {
                core::mem::swap(&mut u, &mut v);
            }
            if u == self.up[u] as usize {
                self.up[u] = self.up[v];
                self.connected_component_count -= 1;
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
    connected_component_count: usize,
}

impl UnionFind {
    /// Creates a new instance of `UnionFind` with length `n`.
    /// 
    /// Pass `n = 0` if an empty instance is desired.
    pub fn new(n: usize) -> Self {
        Self {
            up: (0..n as u32).collect(),
            rank: vec![1; n],
            connected_component_count: n
        }
    }

    /// Returns the number of elements in the current instance.
    pub fn len(&self) -> usize {
        self.up.len()
    }

    /// Returns `true` if the current instance contains no elements.
    pub fn is_empty(&self) -> bool {
        self.up.is_empty()
    }

    /// Alias for `connected_component_count`.
    pub fn cc_count(&self) -> usize {
        self.connected_component_count()
    }

    /// Returns the number of connected components.
    pub fn connected_component_count(&self) -> usize {
        self.connected_component_count
    }

    /// Resizes to increase (or keep) the number of elements.
    /// 
    /// A runtime error will occur if `n` is smaller than `self.len()`.
    pub fn resize(&mut self, n: usize) {
        assert!(n >= self.len());
        self.connected_component_count += n - self.len();
        let mut i = self.up.len();
        self.up.resize_with(n, || {
            let v = i;
            i += 1;
            v as u32
        });
        self.rank.resize(n, 1);
    }

    /// Increases the number of elements by exactly one.
    pub fn push(&mut self) {
        self.up.push(self.up.len() as u32);
        self.rank.push(1);
        self.connected_component_count += 1;
    }

    /// Finds the representative of `u`.
    pub fn find(&mut self, mut u: usize) -> usize {
        while u != self.up[u] as usize {
            self.up[u] = self.up[self.up[u] as usize];
            u = self.up[u] as usize;
        }
        u
    }

    /// Unites `pu` and `pv`.
    /// 
    /// Returns the new parent of the united tree.
    /// 
    /// Both `pu` and `pv` should be strictly less than `self.len()` and be roots.
    /// A runtime error will occur otherwise.
    pub fn union(&mut self, mut pu: usize, mut pv: usize) -> usize {
        assert!(pu < self.len() && pv < self.len());
        assert!(self.up[pu] as usize == pu && self.up[pv] as usize == pv);
        if pu != pv {
            if self.rank[pu] < self.rank[pv] {
                core::mem::swap(&mut pu, &mut pv);
            }
            self.up[pv] = pu as u32;
            if self.rank[pu] == self.rank[pv] {
                self.rank[pu] += 1;
            }
            self.connected_component_count -= 1;
        }
        pu
    }

    /// Tries to unite `u` and `v`.
    /// 
    /// Returns `true` if a new union is created, `false` otherwise.
    /// 
    /// Both `u` and `v` should be strictly less than `self.len()`.
    /// A runtime error will occur otherwise.
    pub fn try_union(&mut self, u: usize, v: usize) -> bool {
        let (pu, pv) = (self.find(u), self.find(v));
        self.union(pu, pv);
        pu != pv
    }
}
