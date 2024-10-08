use alloc::{vec, vec::Vec};

/// Jagged Array using Vec.
///
/// Jagged Array is like an 2d array but length of each row is all different.
pub struct JaggedVec<T> {
    pub(crate) head: Vec<u32>,
    pub(crate) link: Vec<(u32, T)>,
}

impl<T> JaggedVec<T> {
    pub fn new() -> Self {
        Self {
            head: vec![],
            link: vec![],
        }
    }

    pub fn resize(&mut self, row: usize) {
        self.head.resize(row, u32::MAX);
    }

    pub fn reserve(&mut self, capacity: usize) {
        self.link.reserve(capacity);
    }

    pub fn reserve_exact(&mut self, capacity: usize) {
        self.link.reserve_exact(capacity);
    }

    pub fn row(&self) -> usize {
        self.head.len()
    }

    pub fn len(&self) -> usize {
        self.link.len()
    }

    pub fn is_empty(&self) -> bool {
        self.link.is_empty()
    }

    pub fn push(&mut self, row: usize, data: T) {
        let prev = self.head[row];
        self.head[row] = self.len() as u32;
        self.link.push((prev, data));
    }

    pub fn row_iter(&self, row: usize) -> RowIter<T> {
        RowIter {
            vec: self,
            idx: self.head[row],
        }
    }

    pub fn link(&self, id: usize) -> &T {
        &self.link[id].1
    }

    pub fn link_mut(&mut self, id: usize) -> &mut T {
        &mut self.link[id].1
    }

    pub fn first_link(&self, row: usize) -> Option<usize> {
        let head = self.head[row];
        (head != u32::MAX).then_some(head as usize)
    }
}

impl<T> Default for JaggedVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RowIter<'a, T> {
    vec: &'a JaggedVec<T>,
    idx: u32,
}

impl<T> RowIter<'_, T> {
    pub fn id(&self) -> Option<usize> {
        (self.idx != u32::MAX).then_some(self.idx as usize)
    }
}

impl<'a, T> Iterator for RowIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        (self.idx != u32::MAX).then(|| {
            let &(next, ref data) = &self.vec.link[self.idx as usize];
            self.idx = next;
            data
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_initializes_empty_head_and_link() {
        let vec = JaggedVec::<()>::new();
        assert!(vec.head.is_empty());
        assert!(vec.link.is_empty());
    }

    #[test]
    fn resize_resizes_head() {
        let mut vec = JaggedVec::<()>::new();
        vec.resize(10);
        assert_eq!(10, vec.head.len());
    }

    #[test]
    fn reserve_reserves_link() {
        let mut vec = JaggedVec::<()>::new();
        vec.reserve(1000);
        assert!(vec.link.capacity() >= 1000);
    }

    #[test]
    fn reserve_exact_reserves_link() {
        let mut vec = JaggedVec::<()>::new();
        vec.reserve_exact(1000);
        assert_eq!(1000, vec.link.capacity());
    }

    #[test]
    fn push_pushes_data() {
        let mut vec = JaggedVec::new();
        vec.resize(2);
        vec.push(0, 1);
        vec.push(0, 2);
        vec.push(0, 3);
        vec.push(1, 5);
        vec.push(1, 4);
        assert_eq!(vec![&3, &2, &1], vec.row_iter(0).collect::<Vec<_>>());
        assert_eq!(vec![&4, &5], vec.row_iter(1).collect::<Vec<_>>());
    }
}
