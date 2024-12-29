#![allow(clippy::needless_range_loop)]

use alloc::boxed::Box;
use core::cmp::Ordering::{self, Equal, Greater, Less};
use core::marker::PhantomData;
use core::mem::{ManuallyDrop, MaybeUninit};
use core::ops::Bound::*;
use core::ops::RangeBounds;

// The degree of BPTree.
// (degree: the minimum number of children in internal node)
// Each InternalNode (except for root) has T <= #(children) <= 2*T.
// The root has 1 <= #(children) <= 2*T. When the tree is empty, the root does not exist.
// Note that it is required that T >= 2.
const T: usize = 4;
const MAX_STACK_DEPTH: usize = 32;

pub trait LazyOp<V, U> {
    fn binary_op(t1: &V, t2: &V) -> V;
    fn apply(u: &U, t: &V) -> V;
    fn compose(u1: &U, u2: &U) -> U;
    fn id_op() -> U;
    fn apply_option(u: &U, t: Option<&V>) -> Option<V> {
        t.map(|v| Self::apply(u, v))
    }
    fn clone_value(v: &V) -> V {
        Self::apply(&Self::id_op(), v)
    }
    fn clone_op(u: &U) -> U {
        Self::compose(&Self::id_op(), u)
    }
    fn binary_op_option(t1: Option<&V>, t2: Option<&V>) -> Option<V> {
        if let Some(x) = t1 {
            if let Some(y) = t2 {
                Some(Self::binary_op(x, y))
            } else {
                t1.map(|y| Self::apply(&Self::id_op(), y))
            }
        } else {
            t2.map(|y| Self::apply(&Self::id_op(), y))
        }
    }
}

pub struct BPTreeMap<K, V, U, F>
where
    K: Ord,
    F: LazyOp<V, U>,
{
    root: ChildPtr<K, V, U, F>,
    // Starts from 0.
    // When 0, the tree is empty.
    // When 1, root points to a LeafNode.
    // When 2, root points to an InternalNode whose children consist of LeafNodes.
    depth: usize,
    value: Option<V>,
    lazy: U,
    _f: PhantomData<F>,
}

struct InternalNode<K, V, U, F>
where
    K: Ord,
    F: LazyOp<V, U>,
{
    // Filled from the beginning.
    children: [ChildPtr<K, V, U, F>; 2 * T],
    // keys[i] points to the least key in the subtree children[i]
    // Indicates the occupancy for all other MaybeUninit fields and `children`.
    keys: [*const K; 2 * T],
    values: [MaybeUninit<V>; 2 * T], // values[i] denotes the aggregate value of the subtree children[i], with u[i] applied
    // The lazy op u sits above all children of the present node.
    // It is not present in LeafNode.
    lazies: [MaybeUninit<U>; 2 * T],
    _v: PhantomData<V>,
    _f: PhantomData<F>,
}

struct LeafNode<K, V, U, F>
where
    K: Ord,
    F: LazyOp<V, U>,
{
    count: usize,
    keys: [MaybeUninit<K>; 2 * T],
    values: [MaybeUninit<V>; 2 * T],
    _u: PhantomData<U>,
    _f: PhantomData<F>,
}

type ManuallyDropOptionBox<T> = ManuallyDrop<Option<Box<T>>>;
union ChildPtr<K, V, U, F>
where
    K: Ord,
    F: LazyOp<V, U>,
{
    internal_node: ManuallyDropOptionBox<InternalNode<K, V, U, F>>,
    leaf_node: ManuallyDropOptionBox<LeafNode<K, V, U, F>>,
}

pub struct PeekMutPoint<K, V, U>
where
    K: Ord,
{
    _k: PhantomData<K>,
    _v: PhantomData<V>,
    _u: PhantomData<U>,
}

pub struct PeekMutRange<'a, K, V, U, F>
where
    K: Ord,
    F: LazyOp<V, U>,
{
    tree: &'a mut BPTreeMap<K, V, U, F>,
    op: U,
    value: V,
    // [lt_ptr, lt_start, lt_end, rt_ptr, rt_start, rt_end]
    // (rt_ptr is 0 if it does not exist)
    stack: [MaybeUninit<[usize; 6]>; MAX_STACK_DEPTH],
    _k: PhantomData<K>,
    _v: PhantomData<V>,
    _u: PhantomData<U>,
}

impl<K, V, U, F> PeekMutRange<'_, K, V, U, F>
where
    K: Ord,
    F: LazyOp<V, U>,
{
    pub fn value(&self) -> &V {
        &self.value
    }
    pub fn apply(&mut self, u: &U) {
        self.op = F::compose(u, &self.op);
        self.value = F::apply(u, &self.value);
    }
}

impl<K, V, U, F> Drop for PeekMutRange<'_, K, V, U, F>
where
    K: Ord,
    F: LazyOp<V, U>,
{
    fn drop(&mut self) {
        if self.tree.depth == 0 {
            return;
        }
        unsafe {
            // handle leaf nodes
            let x = self.stack[self.tree.depth - 1].assume_init_ref();
            let ptr = &mut *(x[0] as *mut LeafNode<K, V, U, F>);
            for i in x[1]..x[2] {
                ptr.values[i] =
                    MaybeUninit::new(F::apply(&self.op, &ptr.values[i].assume_init_read()));
            }
            let mut lval = Some(ptr.aggregate());
            let mut rval = if x[3] == 0 {
                None
            } else {
                let ptr = &mut *(x[3] as *mut LeafNode<K, V, U, F>);
                for i in x[4]..x[5] {
                    ptr.values[i] =
                        MaybeUninit::new(F::apply(&self.op, &ptr.values[i].assume_init_read()));
                }
                Some(ptr.aggregate())
            };
            // traverse up the tree to the root
            for d in (0..self.tree.depth - 1).rev() {
                let x = self.stack[d].assume_init_ref();
                if x[3] == 0 {
                    let ptr = &mut *(x[0] as *mut InternalNode<K, V, U, F>);
                    for i in x[1] + 1..x[2] {
                        ptr.lazies[i] = MaybeUninit::new(F::compose(
                            &self.op,
                            &ptr.lazies[i].assume_init_read(),
                        ));
                        ptr.values[i] =
                            MaybeUninit::new(F::apply(&self.op, &ptr.values[i].assume_init_read()));
                    }
                    ptr.values[x[1]].assume_init_drop();
                    ptr.values[x[1]] = MaybeUninit::new(lval.unwrap());
                    if rval.is_some() {
                        ptr.values[x[2]].assume_init_drop();
                        ptr.values[x[2]] = MaybeUninit::new(rval.unwrap());
                        rval = None;
                    }
                    lval = Some(ptr.aggregate());
                } else {
                    // left
                    let ptr = &mut *(x[0] as *mut InternalNode<K, V, U, F>);
                    for i in x[1] + 1..=x[2] {
                        ptr.lazies[i] = MaybeUninit::new(F::compose(
                            &self.op,
                            &ptr.lazies[i].assume_init_read(),
                        ));
                        ptr.values[i] =
                            MaybeUninit::new(F::apply(&self.op, &ptr.values[i].assume_init_read()));
                    }
                    ptr.values[x[1]].assume_init_drop();
                    ptr.values[x[1]] = MaybeUninit::new(lval.unwrap());
                    lval = Some(ptr.aggregate());
                    // right
                    let ptr = &mut *(x[3] as *mut InternalNode<K, V, U, F>);
                    for i in x[4]..x[5] {
                        ptr.lazies[i] = MaybeUninit::new(F::compose(
                            &self.op,
                            &ptr.lazies[i].assume_init_read(),
                        ));
                        ptr.values[i] =
                            MaybeUninit::new(F::apply(&self.op, &ptr.values[i].assume_init_read()));
                    }
                    ptr.values[x[5]].assume_init_drop();
                    ptr.values[x[5]] = MaybeUninit::new(rval.unwrap());
                    rval = Some(ptr.aggregate());
                }
            }
            // replace the sum for the whole tree
            self.tree.value = lval;
        }
    }
}

impl<K, V, U, F: LazyOp<V, U>> Default for BPTreeMap<K, V, U, F>
where
    K: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V, U, F> Default for ChildPtr<K, V, U, F>
where
    K: Ord,
    F: LazyOp<V, U>,
{
    fn default() -> Self {
        Self {
            internal_node: ManuallyDrop::new(None),
        }
    }
}

impl<K, V, U, F> Default for InternalNode<K, V, U, F>
where
    K: Ord,
    F: LazyOp<V, U>,
{
    fn default() -> Self {
        Self {
            children: Default::default(),
            keys: [core::ptr::null(); 2 * T],
            values: MaybeUninit::uninit_array(),
            lazies: MaybeUninit::uninit_array(),
            _v: PhantomData,
            _f: PhantomData,
        }
    }
}

impl<K, V, U, F> Default for LeafNode<K, V, U, F>
where
    K: Ord,
    F: LazyOp<V, U>,
{
    fn default() -> Self {
        Self {
            count: 0,
            keys: MaybeUninit::uninit_array(),
            values: MaybeUninit::uninit_array(),
            _u: PhantomData,
            _f: PhantomData,
        }
    }
}

impl<K, V, U, F> Drop for LeafNode<K, V, U, F>
where
    K: Ord,
    F: LazyOp<V, U>,
{
    fn drop(&mut self) {
        for i in 0..self.count {
            unsafe {
                self.keys[i].assume_init_drop();
                self.values[i].assume_init_drop();
            }
        }
    }
}

impl<K, V, U, F> ChildPtr<K, V, U, F>
where
    K: Ord,
    F: LazyOp<V, U>,
{
    unsafe fn drop_by_depth(&mut self, depth: usize) {
        unsafe {
            if depth == 0 {
                ManuallyDrop::drop(&mut self.leaf_node);
                self.leaf_node = ManuallyDrop::new(None);
            } else if let Some(mut x) = Option::take(&mut self.internal_node) {
                for i in 0..x.children.len() {
                    if x.keys[i].is_null() {
                        break;
                    } else {
                        x.children[i].drop_by_depth(depth - 1);
                        x.values[i].assume_init_drop();
                        x.lazies[i].assume_init_drop();
                    }
                }
                self.internal_node = ManuallyDrop::new(None);
            }
        }
    }
    unsafe fn as_internal_node_ref(&self) -> &InternalNode<K, V, U, F> {
        unsafe { self.internal_node.as_ref().unwrap() }
    }
    unsafe fn as_leaf_node_ref(&self) -> &LeafNode<K, V, U, F> {
        unsafe { self.leaf_node.as_ref().unwrap() }
    }
    unsafe fn as_internal_node_mut(&mut self) -> &mut InternalNode<K, V, U, F> {
        unsafe { self.internal_node.as_mut().unwrap() }
    }
    unsafe fn as_leaf_node_mut(&mut self) -> &mut LeafNode<K, V, U, F> {
        unsafe { self.leaf_node.as_mut().unwrap() }
    }
    /// Splits the node pointed to by self, if it is full.
    /// If the node is full, it gets split, and we return Some(right_node),
    /// where right_node points to the right half of the split node.
    /// If the node is not full, it stays the same, and we return None.
    ///
    /// `level` is used to determine whether the current node points to
    /// InternalNode (> 0) or LeafNode (== 0).
    fn split_if_full(&mut self, level: usize) -> Option<ChildPtr<K, V, U, F>> {
        unsafe {
            if level > 0 {
                self.as_internal_node_mut().split_if_full(level)
            } else {
                self.as_leaf_node_mut().split_if_full()
            }
        }
    }
    fn least_key(&self, level: usize) -> *const K {
        unsafe {
            if level > 0 {
                self.as_internal_node_ref().keys[0]
            } else {
                self.as_leaf_node_ref().keys[0].assume_init_ref() as *const K
            }
        }
    }
    fn aggregate(&self, level: usize) -> V {
        unsafe {
            if level > 0 {
                self.as_internal_node_ref().aggregate()
            } else {
                self.as_leaf_node_ref().aggregate()
            }
        }
    }
}

impl<K, V, U, F> InternalNode<K, V, U, F>
where
    K: Ord,
    F: LazyOp<V, U>,
{
    fn split_if_full(&mut self, level: usize) -> Option<ChildPtr<K, V, U, F>> {
        debug_assert!(level > 0);
        if self.keys.last().unwrap().is_null() {
            None
        } else {
            let mut right_node = Box::new(Self {
                children: Default::default(),
                keys: [core::ptr::null(); 2 * T],
                values: MaybeUninit::uninit_array(),
                lazies: MaybeUninit::uninit_array(),
                _v: PhantomData,
                _f: PhantomData,
            });

            // Move keys
            right_node.keys[..T].swap_with_slice(&mut self.keys[T..]);
            // Move children
            right_node.children[..T].swap_with_slice(&mut self.children[T..]);
            // Move values
            right_node.values[..T].swap_with_slice(&mut self.values[T..]);
            // Move lazies
            right_node.lazies[..T].swap_with_slice(&mut self.lazies[T..]);

            Some(ChildPtr {
                internal_node: ManuallyDrop::new(Some(right_node)),
            })
        }
    }
    // Inserts at position i, and right-shift existing elements in i and afterwards by one.
    // If the node is already full, this function will panic.
    fn insert_at(&mut self, i: usize, child_ptr: ChildPtr<K, V, U, F>, level: usize) {
        assert!(self.keys.last().unwrap().is_null());
        for j in (i..self.children.len() - 1).rev() {
            self.children.swap(j, j + 1);
            self.keys[j + 1] = self.keys[j];
            self.values.swap(j, j + 1);
            self.lazies.swap(j, j + 1);
        }
        self.keys[i] = child_ptr.least_key(level - 1);
        self.values[i] = MaybeUninit::new(child_ptr.aggregate(level - 1));
        self.lazies[i] = MaybeUninit::new(F::id_op());
        self.children[i] = child_ptr;
    }
    fn push(&mut self, u: &U) {
        unsafe {
            for i in 0..self.lazies.len() {
                if !self.keys[i].is_null() {
                    self.lazies[i] =
                        MaybeUninit::new(F::compose(u, &self.lazies[i].assume_init_read()));
                    self.values[i] =
                        MaybeUninit::new(F::apply(u, &self.values[i].assume_init_read()));
                } else {
                    break;
                }
            }
        }
    }
    /// Pulls key and value from `self.children[i]`.
    fn pull_at(&mut self, i: usize, level: usize) {
        unsafe {
            debug_assert!(level >= 1);
            let v = self.children[i].aggregate(level - 1);
            self.keys[i] = self.children[i].least_key(level - 1);
            self.values[i].assume_init_drop();
            self.values[i] = MaybeUninit::new(F::apply(self.lazies[i].assume_init_ref(), &v));
        }
    }
    /// Returns the aggregate value of the current node.
    fn aggregate(&self) -> V {
        unsafe {
            let out = self.values[0].assume_init_ref();
            let mut out2 = None;
            for i in 1..self.values.len() {
                if !self.keys[i].is_null() {
                    out2 = Some(F::binary_op(
                        if i == 1 { out } else { out2.as_ref().unwrap() },
                        self.values[i].assume_init_ref(),
                    ));
                } else {
                    break;
                }
            }
            out2.unwrap_or(F::apply(&F::id_op(), out))
        }
    }
    /// \[start, end\] only potentially has overlap; outside it, no overlap is guaranteed.
    /// Returns (start, end, sum on start+1..=end-1)
    ///
    /// For `Unbounded`, we ignore any sort of safety margin.
    fn aggregate_range<R: RangeBounds<K>>(
        &self,
        range: &R,
        lt_unbounded: bool,
        rt_unbounded: bool,
    ) -> (usize, usize, Option<V>) {
        let mut start = 0;
        while start < self.keys.len()
            && !self.keys[start].is_null()
            && match range.start_bound() {
                Included(k) => k.cmp(unsafe { &*self.keys[start] }) == Ordering::Greater,
                Excluded(k) => k.cmp(unsafe { &*self.keys[start] }) != Ordering::Less,
                Unbounded => false,
            }
        {
            start += 1;
        }
        let mut end = start;
        while end < self.keys.len()
            && !self.keys[end].is_null()
            && match range.end_bound() {
                Included(k) => k.cmp(unsafe { &*self.keys[end] }) != Ordering::Less,
                Excluded(k) => k.cmp(unsafe { &*self.keys[end] }) == Ordering::Greater,
                Unbounded => true,
            }
        {
            end += 1;
        }
        start = start.saturating_sub(1); // we consider [start, end)
        let mut out = None;
        let rstart = if lt_unbounded { start } else { start + 1 };
        let rend = if rt_unbounded { end } else { end - 1 };
        for i in rstart..rend {
            let v = unsafe { self.values[i].assume_init_ref() };
            out = F::binary_op_option(out.as_ref(), Some(v));
        }
        (start, end.saturating_sub(1), out)
    }
}

impl<K, V, U, F> LeafNode<K, V, U, F>
where
    K: Ord,
    F: LazyOp<V, U>,
{
    fn split_if_full(&mut self) -> Option<ChildPtr<K, V, U, F>> {
        if self.count == self.keys.len() {
            self.count = T;
            let mut right_node = Box::new(Self {
                count: T,
                keys: MaybeUninit::uninit_array(),
                values: MaybeUninit::uninit_array(),
                _u: PhantomData,
                _f: PhantomData,
            });

            // Move keys
            right_node.keys[..T].swap_with_slice(&mut self.keys[T..]);
            // Move values
            right_node.values[..T].swap_with_slice(&mut self.values[T..]);

            Some(ChildPtr {
                leaf_node: ManuallyDrop::new(Some(right_node)),
            })
        } else {
            None
        }
    }
    fn insert(&mut self, key: K, mut value: V) -> Option<V> {
        for i in 0..self.count {
            match unsafe { self.keys[i].assume_init_ref() }.cmp(&key) {
                Less => {
                    continue;
                }
                Equal => unsafe {
                    core::mem::swap(self.values[i].assume_init_mut(), &mut value);
                    return Some(value);
                },
                Greater => {
                    assert!(self.count < self.keys.len());
                    for j in (i..self.count).rev() {
                        self.keys.swap(j, j + 1);
                        self.values.swap(j, j + 1);
                    }
                    self.keys[i] = MaybeUninit::new(key);
                    self.values[i] = MaybeUninit::new(value);
                    self.count += 1;
                    return None;
                }
            }
        }
        // Failed to match any entry. So we insert anew.
        assert!(self.count < self.keys.len());
        self.keys[self.count] = MaybeUninit::new(key);
        self.values[self.count] = MaybeUninit::new(value);
        self.count += 1;
        None
    }
    fn push(&mut self, u: &U) {
        for i in 0..self.count {
            self.values[i] =
                MaybeUninit::new(F::apply(u, unsafe { self.values[i].assume_init_ref() }));
        }
    }
    fn aggregate(&self) -> V {
        unsafe {
            let out = self.values[0].assume_init_ref();
            let mut out2 = None;
            for i in 1..self.count {
                out2 = Some(F::binary_op(
                    if i == 1 { out } else { out2.as_ref().unwrap() },
                    self.values[i].assume_init_ref(),
                ));
            }
            out2.unwrap_or(F::clone_value(out))
        }
    }
    /// Returns sum of all values whose keys fall in `range`.
    ///
    /// Note: usize values represent [start, end)
    fn aggregate_range<R: RangeBounds<K>>(&self, range: &R) -> (usize, usize, Option<V>) {
        let mut start = 0;
        while start < self.count && !range.contains(unsafe { self.keys[start].assume_init_ref() }) {
            start += 1;
        }
        let mut end = start;
        let mut out = None;
        while end < self.count && range.contains(unsafe { self.keys[end].assume_init_ref() }) {
            let y = unsafe { self.values[end].assume_init_ref() };
            out = Some(if let Some(x) = out {
                F::binary_op(&x, y)
            } else {
                F::clone_value(y)
            });
            end += 1;
        }
        (start, end, out)
    }
}

impl<K, V, U, F> Drop for BPTreeMap<K, V, U, F>
where
    K: Ord,
    F: LazyOp<V, U>,
{
    fn drop(&mut self) {
        self.clear();
    }
}

impl<K, V, U, F> BPTreeMap<K, V, U, F>
where
    K: Ord,
    F: LazyOp<V, U>,
{
    pub fn new() -> Self {
        Self {
            root: Default::default(),
            depth: 0,
            value: None,
            lazy: F::id_op(),
            _f: Default::default(),
        }
    }
    pub fn clear(&mut self) {
        if self.depth != 0 {
            unsafe {
                self.root.drop_by_depth(self.depth - 1);
            }
            self.depth = 0;
            self.root = Default::default();
            self.value = None;
            self.lazy = F::id_op();
        }
    }
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        unsafe {
            let out;
            if self.depth == 0 {
                let mut leaf_node: Box<LeafNode<K, V, U, F>> = Box::default();
                out = leaf_node.insert(key, value);
                self.root = ChildPtr {
                    leaf_node: ManuallyDrop::new(Some(leaf_node)),
                };
                self.depth = 1;
            } else {
                #[allow(clippy::type_complexity)]
                let mut stack: [MaybeUninit<(*mut InternalNode<K, V, U, F>, usize)>;
                    MAX_STACK_DEPTH] = MaybeUninit::uninit_array();
                let mut stack_size = 0usize;

                // Phase 1: Go down to the leaf node, along which we propagate the lazy op.
                // Invariant:
                //   1) cur_ptr is at cur_level
                //   2) cur_ptr is fixed, it does not need to be modified
                //   3) cur_ptr is not full
                let mut cur_ptr = &mut self.root;
                let mut cur_level = self.depth - 1;
                let mut u = F::id_op();
                core::mem::swap(&mut u, &mut self.lazy);
                while cur_level > 0 {
                    let y = cur_ptr.internal_node.as_mut().unwrap().as_mut()
                        as *mut InternalNode<K, V, U, F>;

                    // Push-down the lazy op
                    let x = cur_ptr.as_internal_node_mut();
                    x.push(&u);

                    // Find which child to traverse
                    let mut i = 0;
                    while i < 2 * T - 1 {
                        if x.keys[i + 1].is_null() {
                            break;
                        } else {
                            match Ord::cmp(&*x.keys[i + 1], &key) {
                                Less | Equal => {
                                    i += 1;
                                    continue;
                                }
                                Greater => {
                                    // Updating x.key is needed since current key comes first.
                                    // It will be handled during follow-up from the leaf to the root.
                                    break;
                                }
                            }
                        }
                    }

                    // Save lazy op for the node right below the current level
                    u = x.lazies[i].assume_init_read();
                    x.lazies[i] = MaybeUninit::new(F::id_op());

                    // Save cur_ptr on stack
                    stack[stack_size] = MaybeUninit::new((y, i));
                    stack_size += 1;

                    // Update cur_level
                    cur_level -= 1;

                    // Go to the child node
                    cur_ptr = &mut x.children[i];
                }

                // Apply lazy op to the leaf node
                cur_ptr.as_leaf_node_mut().push(&u);

                // Phase 2:
                //   Find the duplicate key if it exists,
                //   and return the existing value by move if found
                let mut up_ptr = cur_ptr.split_if_full(0);
                let leaf_node = cur_ptr.as_leaf_node_mut();
                if let Some(n) = &mut up_ptr {
                    let x = n.as_leaf_node_mut();
                    if key.cmp(x.keys[0].assume_init_ref()) == Less {
                        out = leaf_node.insert(key, value);
                    } else {
                        out = x.insert(key, value);
                    }
                } else {
                    out = leaf_node.insert(key, value);
                }

                // Phase 3:
                //   Re-compute the aggregate values from leaf to root,
                //   with node splitting as needed along the journey.

                // Note: we need to keep track of the minimum key
                while stack_size > 0 {
                    let level = self.depth - stack_size;
                    let (n, i) = {
                        let (ptr, i) = stack[stack_size - 1].assume_init();
                        (&mut *ptr, i)
                    };
                    if let Some(up_ptr_inner) = up_ptr {
                        // We need to insert up_ptr in the node n after position i
                        if let Some(mut right) = n.split_if_full(level) {
                            if i >= T {
                                // insert to right.1 at i + 1 - T
                                let r = right.as_internal_node_mut();
                                r.insert_at(i + 1 - T, up_ptr_inner, level);
                                r.pull_at(i - T, level);
                            } else {
                                // insert to n at i + 1
                                n.insert_at(i + 1, up_ptr_inner, level);
                                n.pull_at(i, level);
                            }
                            // Update up_ptr
                            up_ptr = Some(right);
                        } else {
                            n.insert_at(i + 1, up_ptr_inner, level);
                            up_ptr = None;
                            n.pull_at(i, level);
                        }
                    } else {
                        n.pull_at(i, level);
                    }

                    // Pop
                    stack_size -= 1;
                }

                // Create a new root if needed
                if let Some(up_ptr_inner) = up_ptr {
                    let mut root_node: Box<InternalNode<K, V, U, F>> = Box::default();
                    let mut tmp = ChildPtr {
                        internal_node: ManuallyDrop::new(None),
                    };
                    core::mem::swap(&mut tmp, &mut self.root);
                    root_node.insert_at(0, tmp, self.depth);
                    root_node.insert_at(1, up_ptr_inner, self.depth);
                    self.root = ChildPtr {
                        internal_node: ManuallyDrop::new(Some(root_node)),
                    };
                    self.depth += 1;
                }
            }

            // Update global value
            // Note: self.lazy has been already set to F::id_op()
            self.value = Some(self.root.aggregate(self.depth - 1));

            out
        }
    }
    pub fn remove(&mut self, _key: &K) -> Option<V> {
        None
    }
    pub fn get(&self, key: &K) -> Option<V> {
        self.get_range(key..=key)
    }
    pub fn get_mut(&mut self, _key: &K) -> Option<PeekMutPoint<K, V, U>> {
        None
    }
    pub fn get_range<R: RangeBounds<K>>(&self, range: R) -> Option<V> {
        if self.depth == 0 {
            None
        } else {
            // we must consider lazy propagation downwards
            let mut out = None;
            let mut l = &self.root;
            let mut r = &ChildPtr::<K, V, U, F>::default();
            let mut u_l = F::clone_op(&self.lazy);
            let mut u_r = F::id_op();
            for _ in 0..self.depth - 1 {
                if unsafe { r.internal_node.is_some() } {
                    let (s, _, mut partial_sum) =
                        unsafe { l.as_internal_node_ref() }.aggregate_range(&range, false, true);
                    partial_sum = F::apply_option(&u_l, partial_sum.as_ref());
                    out = F::binary_op_option(partial_sum.as_ref(), out.as_ref());
                    u_l = F::compose(&u_l, unsafe {
                        l.as_internal_node_ref().lazies[s].assume_init_ref()
                    });
                    l = unsafe { &l.as_internal_node_ref().children[s] };
                    let (_, e, mut partial_sum) =
                        unsafe { r.as_internal_node_ref() }.aggregate_range(&range, true, false);
                    partial_sum = F::apply_option(&u_r, partial_sum.as_ref());
                    out = F::binary_op_option(out.as_ref(), partial_sum.as_ref());
                    u_r = F::compose(&u_r, unsafe {
                        r.as_internal_node_ref().lazies[e].assume_init_ref()
                    });
                    r = unsafe { &r.as_internal_node_ref().children[e] };
                } else {
                    let (s, e, mut partial_sum) =
                        unsafe { l.as_internal_node_ref() }.aggregate_range(&range, false, false);
                    partial_sum = F::apply_option(&u_l, partial_sum.as_ref());
                    if e > s {
                        u_r = F::compose(&u_l, unsafe {
                            l.as_internal_node_ref().lazies[e].assume_init_ref()
                        });
                        r = unsafe { &l.as_internal_node_ref().children[e] };
                    }
                    u_l = F::compose(&u_l, unsafe {
                        l.as_internal_node_ref().lazies[s].assume_init_ref()
                    });
                    l = unsafe { &l.as_internal_node_ref().children[s] };
                    out = partial_sum;
                }
            }
            let partial_sum = F::apply_option(
                &u_l,
                unsafe { l.as_leaf_node_ref() }
                    .aggregate_range(&range)
                    .2
                    .as_ref(),
            );
            out = F::binary_op_option(partial_sum.as_ref(), out.as_ref());
            if unsafe { r.leaf_node.is_some() } {
                let partial_sum = F::apply_option(
                    &u_r,
                    unsafe { r.as_leaf_node_ref() }
                        .aggregate_range(&range)
                        .2
                        .as_ref(),
                );
                out = F::binary_op_option(out.as_ref(), partial_sum.as_ref());
            }
            out
        }
    }
    pub fn get_range_mut<R: RangeBounds<K>>(
        &mut self,
        range: R,
    ) -> Option<PeekMutRange<K, V, U, F>> {
        if self.depth == 0 {
            None
        } else {
            // we must consider lazy propagation downwards
            let mut stack = MaybeUninit::uninit_array();
            let mut out = None;
            let mut l = &mut self.root;
            let mut r = &mut ChildPtr::<K, V, U, F>::default();
            let mut u_l = F::clone_op(&self.lazy);
            let mut u_r = F::id_op();
            self.lazy = F::id_op();
            for i in 0..self.depth - 1 {
                if unsafe { r.internal_node.is_some() } {
                    unsafe { l.as_internal_node_mut() }.push(&u_l);
                    let (ls, le, partial_sum) =
                        unsafe { l.as_internal_node_ref() }.aggregate_range(&range, false, true);
                    out = F::binary_op_option(partial_sum.as_ref(), out.as_ref());
                    u_l = unsafe { l.as_internal_node_ref().lazies[ls].assume_init_read() };
                    unsafe { l.as_internal_node_mut() }.lazies[ls] = MaybeUninit::new(F::id_op());

                    unsafe { r.as_internal_node_mut() }.push(&u_r);
                    let (rs, re, partial_sum) =
                        unsafe { r.as_internal_node_ref() }.aggregate_range(&range, true, false);
                    out = F::binary_op_option(out.as_ref(), partial_sum.as_ref());
                    u_r = unsafe { r.as_internal_node_ref().lazies[re].assume_init_read() };
                    unsafe { r.as_internal_node_mut() }.lazies[re] = MaybeUninit::new(F::id_op());

                    unsafe {
                        stack[i] = MaybeUninit::new([
                            l.as_leaf_node_ref() as *const _ as usize,
                            ls,
                            le,
                            r.as_leaf_node_ref() as *const _ as usize,
                            rs,
                            re,
                        ]);
                    }

                    l = unsafe { &mut l.as_internal_node_mut().children[ls] };
                    r = unsafe { &mut r.as_internal_node_mut().children[re] };
                } else {
                    unsafe { l.as_internal_node_mut() }.push(&u_l);
                    let (s, e, partial_sum) =
                        unsafe { l.as_internal_node_ref() }.aggregate_range(&range, false, false);
                    if e > s {
                        u_r = unsafe { l.as_internal_node_ref().lazies[e].assume_init_read() };
                        unsafe { l.as_internal_node_mut() }.lazies[e] =
                            MaybeUninit::new(F::id_op());
                    }
                    u_l = unsafe { l.as_internal_node_ref().lazies[s].assume_init_read() };
                    unsafe { l.as_internal_node_mut() }.lazies[s] = MaybeUninit::new(F::id_op());
                    unsafe {
                        stack[i] = MaybeUninit::new([
                            l.as_leaf_node_ref() as *const _ as usize,
                            s,
                            e,
                            0,
                            0,
                            0,
                        ]);
                    }
                    if e > s {
                        let (ll, lr) = unsafe { l.as_internal_node_mut().children.split_at_mut(e) };
                        r = &mut lr[0];
                        l = &mut ll[s];
                    } else {
                        l = unsafe { &mut l.as_internal_node_mut().children[s] };
                    }
                    out = partial_sum;
                }
            }
            unsafe { l.as_leaf_node_mut() }.push(&u_l);
            let (ls, le, partial_sum) = unsafe { l.as_leaf_node_ref() }.aggregate_range(&range);
            out = F::binary_op_option(partial_sum.as_ref(), out.as_ref());
            if unsafe { r.leaf_node.is_some() } {
                unsafe { r.as_leaf_node_mut() }.push(&u_r);
                let (rs, re, partial_sum) = unsafe { r.as_leaf_node_ref() }.aggregate_range(&range);
                out = F::binary_op_option(out.as_ref(), partial_sum.as_ref());
                unsafe {
                    stack[self.depth - 1] = MaybeUninit::new([
                        l.as_leaf_node_ref() as *const _ as usize,
                        ls,
                        le,
                        r.as_leaf_node_ref() as *const _ as usize,
                        rs,
                        re,
                    ]);
                }
            } else {
                unsafe {
                    stack[self.depth - 1] = MaybeUninit::new([
                        l.as_leaf_node_ref() as *const _ as usize,
                        ls,
                        le,
                        0,
                        0,
                        0,
                    ]);
                }
            }
            out.map(|x| PeekMutRange {
                tree: self,
                op: F::id_op(),
                value: x,
                stack,
                _k: PhantomData,
                _v: PhantomData,
                _u: PhantomData,
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_btree_interval_sum() {
        struct F;
        impl LazyOp<(i64, usize), i64> for F {
            fn binary_op(t1: &(i64, usize), t2: &(i64, usize)) -> (i64, usize) {
                (t1.0 + t2.0, t1.1 + t2.1)
            }
            fn apply(u: &i64, t: &(i64, usize)) -> (i64, usize) {
                (t.0 + u * t.1 as i64, t.1)
            }
            fn compose(u1: &i64, u2: &i64) -> i64 {
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
        assert_eq!(Some((5, 2)), bptm.get_range(2..=3));
        assert_eq!(Some((23, 5)), bptm.get_range(2..=6));

        let mut bptm = BPTreeMap::<usize, (i64, usize), i64, F>::new();
        let n = 100;
        let mut v = vec![0; n + 1];
        for i in 1..=n {
            v[i] = i as i64;
            bptm.insert(i, (v[i], 1));
        }
        for j in 0..20 {
            let mut s = ((j * j * j + j * 37 + 394) % n as i64 + 1) as usize;
            let mut e = (s * s * s + s * 37 + 394) % n + 1;
            if s > e {
                (s, e) = (e, s);
            }
            if j < 10 {
                let delta = j * j;
                bptm.get_range_mut(s..=e).unwrap().apply(&delta);
                for i in s..=e {
                    v[i] += delta;
                }
            } else {
                let gt = (v[s..=e].iter().sum::<i64>(), e - s + 1);
                let val = bptm.get_range(s..=e).unwrap();
                assert_eq!(gt, val);
            }
        }
    }
    #[test]
    fn check_btree_duplicate_inserts() {
        struct F;
        impl LazyOp<usize, ()> for F {
            fn binary_op(t1: &usize, t2: &usize) -> usize {
                t1 + t2
            }
            fn apply(_u: &(), t: &usize) -> usize {
                *t
            }
            fn compose(_u1: &(), _u2: &()) {}
            fn id_op() {}
        }
        let mut b = BPTreeMap::<i64, usize, (), F>::new();
        let mut v = vec![];
        let n = 100;
        for i in 0..n {
            let x = (i * i * i * i * i) % 1000;
            let exists = b.insert(x, 1).is_some();
            assert_eq!((i, x, exists), (i, x, v.contains(&x)));
            v.push(x);
        }
        v.sort();
        v.dedup();
        for (i, &x) in v.iter().enumerate() {
            assert_eq!(i, b.get_range(..=x).unwrap() - 1);
        }
    }
}
