use alloc::boxed::Box;
use core::marker::PhantomData;
use core::mem::{ManuallyDrop, MaybeUninit};
use core::ops::RangeBounds;

// The degree of BPTree.
// (degree: the minimum number of children in internal node)
// Each InternalNode (except for root) has T <= #(children) <= 2*T.
// The root has 0 <= #(children) <= 2*T, with 0 only allowed when the tree is empty.
const T: usize = 4;

pub trait LazyOp<V, U> {
    fn binary_op(t1: V, t2: V) -> V;
    fn apply(u: U, t: V) -> V;
    fn compose(u1: U, u2: U) -> U;
    fn id_op() -> U;
}

pub struct BPTreeMap<K, V, U, F>
where
    F: LazyOp<V, U>,
{
    root: InternalNode<K, V, U>,
    // Starts from 0.
    // When 0, the tree is empty.
    // When 1, root points an InternalNode whose children consist of LeafNodes.
    depth: usize,
    _f: PhantomData<F>,
}

struct InternalNode<K, V, U> {
    // Filled from the beginning.
    children: [ChildPtr<K, V, U>; 2*T],
    keys: [MaybeUninit<K>; 2*T - 1],
    values: [MaybeUninit<V>; 2*T],
    // The lazy op u sits above all children of the present node.
    // It is not present in LeafNode.
    u: U,
    _v: PhantomData<V>,
}

struct LeafNode<K, V> {
    count: usize,
    keys: [MaybeUninit<K>; 2*T],
    values: [MaybeUninit<V>; 2*T],
}

union ChildPtr<K, V, U> {
    internal_node: ManuallyDrop<Option<Box<InternalNode<K, V, U>>>>,
    leaf_node: ManuallyDrop<Option<Box<LeafNode<K, V>>>>,
}

pub struct PeekMutPoint<K, V, U> {
    _k: PhantomData<K>,
    _v: PhantomData<V>,
    _u: PhantomData<U>,
}

pub struct PeekMutRange<K, V, U> {
    _k: PhantomData<K>,
    _v: PhantomData<V>,
    _u: PhantomData<U>,
}

impl<K, V, U> PeekMutRange<K, V, U> {
    pub fn value(&mut self) -> V {
        todo!()
    }
    pub fn apply(&mut self, _u: &U) {
        todo!()
    }
}

impl<K, V, U, F: LazyOp<V, U>> Default for BPTreeMap<K, V, U, F> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V, U> Default for ChildPtr<K, V, U> {
    fn default() -> Self {
        Self {
            internal_node: ManuallyDrop::new(None)
        }
    }
}

impl<K, V, U, F> BPTreeMap<K, V, U, F>
where
    F: LazyOp<V, U>,
{
    pub fn new() -> Self {
        assert!(T >= 2);
        Self {
            root: InternalNode {
                children: Default::default(),
                keys: MaybeUninit::uninit_array(),
                values: MaybeUninit::uninit_array(),
                u: F::id_op(),
                _v: Default::default(),
            },
            depth: 0,
            _f: Default::default(),
        }
    }
    pub fn clear(&mut self) {}
    pub fn get(&self, _key: &K) -> Option<&V> {
        None
    }
    pub fn get_mut(&mut self, _key: &K) -> Option<PeekMutPoint<K, V, U>> {
        None
    }
    pub fn get_range<R: RangeBounds<K>>(&self, _range: R) -> Option<V> {
        None
    }
    pub fn get_range_mut<R: RangeBounds<K>>(&mut self, _range: R) -> Option<PeekMutRange<K, V, U>> {
        None
    }
    pub fn insert(&mut self, _key: K, _value: V) -> Option<V> {
        None
    }
    pub fn remove(&mut self, _key: &K) -> Option<V> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_btree_interval_sum() {
        struct F;
        impl LazyOp<(i64, usize), i64> for F {
            fn binary_op(t1: (i64, usize), t2: (i64, usize)) -> (i64, usize) {
                (t1.0 + t2.0, t1.1 + t2.1)
            }
            fn apply(u: i64, t: (i64, usize)) -> (i64, usize) {
                (t.0 + u * t.1 as i64, t.1)
            }
            fn compose(u1: i64, u2: i64) -> i64 {
                u1 + u2
            }
            fn id_op() -> i64 {
                0
            }
        }
        let mut bptm = BPTreeMap::<usize, (i64, usize), i64, F>::new();
        let n = 10;
        for i in 1..=n {
            bptm.insert(i, (i as i64, 1));
        }
        assert_eq!(Some((18, 4)), bptm.get_range(3..=6));
        assert_eq!(Some((18, 4)), bptm.get_range(3..7));
        assert_eq!(Some((55, 10)), bptm.get_range(..));
        assert_eq!(Some((36, 8)), bptm.get_range(..9));
        assert_eq!(Some((36, 8)), bptm.get_range(..=8));
        assert_eq!(Some((52, 8)), bptm.get_range(3..));
        bptm.get_range_mut(4..=6).unwrap().apply(&1i64);
        assert_eq!(Some((21, 4)), bptm.get_range(3..=6));
        assert_eq!(Some((21, 4)), bptm.get_range(3..7));
        assert_eq!(Some((58, 10)), bptm.get_range(..));
        assert_eq!(Some((39, 8)), bptm.get_range(..9));
        assert_eq!(Some((39, 8)), bptm.get_range(..=8));
        assert_eq!(Some((55, 8)), bptm.get_range(3..));
    }
}
