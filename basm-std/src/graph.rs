pub mod maxflow;
use crate::collections::JaggedVec;

pub trait DfsTarget<T> {
    type Iter: DfsIter<T>;
    fn dfs_iter(&self, from: <Self::Iter as DfsIter<T>>::V) -> Self::Iter;
}

pub trait DfsIter<T>: Clone {
    type G;
    type V;
    type E;
    fn from(&self, graph: &Self::G) -> Self::V;
    fn to(&self, graph: &Self::G) -> Self::V;
    fn id(&self, graph: &Self::G) -> Self::E;
    fn data<'a>(&self, graph: &'a Self::G) -> &'a T;
    fn next(&mut self, graph: &Self::G) -> bool;
}

impl<T> DfsTarget<T> for JaggedVec<(u32, T)> {
    type Iter = JaggedDfsIter;
    fn dfs_iter(&self, from: <Self::Iter as DfsIter<T>>::V) -> Self::Iter {
        JaggedDfsIter(from as u32, self.head[from])
    }
}

#[derive(Clone)]
pub struct JaggedDfsIter(u32, u32);

impl<T> DfsIter<T> for JaggedDfsIter {
    type G = JaggedVec<(u32, T)>;
    type V = usize;
    type E = usize;

    fn from(&self, _: &Self::G) -> Self::V {
        self.0 as usize
    }

    fn to(&self, graph: &Self::G) -> Self::V {
        graph.link[self.1 as usize].1 .0 as usize
    }

    fn id(&self, _: &Self::G) -> Self::E {
        self.1 as usize
    }

    fn data<'a>(&self, graph: &'a Self::G) -> &'a T {
        &graph.link[self.1 as usize].1 .1
    }

    fn next(&mut self, graph: &Self::G) -> bool {
        if self.1 == u32::MAX {
            false
        } else {
            self.1 = graph.link[self.1 as usize].0;
            true
        }
    }
}

#[macro_export]
macro_rules! dfs {
    {
        ($g:ident, $node:expr)
        |$from:ident, $to:ident, $data:ident, $edge:ident| =>
        $begin:expr => $before:expr => recurse => $after:expr => $end:expr
    } => {
        let $from = $node;
        $begin;
        let mut stack = vec![DfsTarget::dfs_iter(&$g, $from)];
        #[allow(unused_variables)]
        while let Some(iter) = stack.last_mut() {
            let $from = iter.from(&$g);
            let $edge = iter.id(&$g);
            let current = iter.clone();
            if iter.next(&$g) {
                let $to = current.to(&$g);
                let $data = current.data(&$g);
                $before;
                *iter = current;
                let $from = $to;
                $begin;
                stack.push(DfsTarget::dfs_iter(&$g, $from));
            } else {
                stack.pop();
                if let Some(iter) = stack.last_mut() {
                    let $from = iter.from(&$g);
                    let $to = iter.to(&$g);
                    let $edge = iter.id(&$g);
                    let $data = iter.data(&$g);
                    iter.next(&$g);
                    {
                        let $from = $to;
                        $end;
                    }
                    $after;
                } else {
                    $end;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dfs_runs_on_jagged_vec() {
        let mut vec = JaggedVec::new();
        vec.resize(5);
        vec.push(0, (1, ()));
        vec.push(1, (2, ()));
        vec.push(0, (3, ()));
        vec.push(2, (3, ()));
        vec.push(1, (3, ()));
        let mut begin = vec![];
        let mut before = vec![];
        let mut after = vec![];
        let mut end = vec![];
        let mut visited = [false; 5];
        dfs! {
            (vec, 0)
            |from, to, data, edge| => {
                visited[from] = true;
                begin.push(from);
            } => {
                before.push(edge);
                if visited[to] {
                    continue;
                }
            } => recurse => {
                after.push(edge);
            } => {
                end.push(from);
            }
        }
        assert_eq!(vec![0, 3, 1, 2], begin);
        assert_eq!(vec![2, 0, 4, 1, 3], before);
        assert_eq!(vec![2, 1, 0], after);
        assert_eq!(vec![3, 2, 1, 0], end);
    }
}
