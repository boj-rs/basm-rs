// MIT License
//
// Copyright (c) 2020-2021 Han Mertens
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// FROM https://github.com/hanmertens/dary_heap

use core::fmt;
use core::iter::{FromIterator, FusedIterator};
use core::mem::{size_of, swap, ManuallyDrop};
use core::ops::{Deref, DerefMut};
use core::ptr;
use core::slice;

use alloc::{vec, vec::Vec};

/// A binary heap (*d* = 2).
pub type BinaryHeap<T> = DaryHeap<T, 2>;

/// A ternary heap (*d* = 3).
pub type TernaryHeap<T> = DaryHeap<T, 3>;

/// A quaternary heap (*d* = 4).
pub type QuaternaryHeap<T> = DaryHeap<T, 4>;

/// A quinary heap (*d* = 5).
pub type QuinaryHeap<T> = DaryHeap<T, 5>;

/// A senary heap (*d* = 6).
pub type SenaryHeap<T> = DaryHeap<T, 6>;

/// A septenary heap (*d* = 7).
pub type SeptenaryHeap<T> = DaryHeap<T, 7>;

/// An octonary heap (*d* = 8).
pub type OctonaryHeap<T> = DaryHeap<T, 8>;

/// A priority queue implemented with a *d*-ary heap.
///
/// This will be a max-heap.
///
/// It is a logic error for an item to be modified in such a way that the
/// item's ordering relative to any other item, as determined by the [`Ord`]
/// trait, changes while it is in the heap. This is normally only possible
/// through [`Cell`], [`RefCell`], global state, I/O, or unsafe code. The
/// behavior resulting from such a logic error is not specified (it
/// could include panics, incorrect results, aborts, memory leaks, or
/// non-termination) but will not be undefined behavior.
///
/// # Usage
///
/// Rust type interference cannot infer the desired heap arity (value of *d*)
/// automatically. Therefore, it is generally more ergonomic to use one of the
/// [type aliases] instead of `DaryHeap` directly. See the [crate-level
/// documentation][usage] for more information.
///
/// [type aliases]: index.html#types
/// [usage]: index.html#usage
///
/// # Comparison to standard library
///
/// For a comparison with [`std::collections::BinaryHeap`][std], see the [crate-level
/// documentation][comparison].
///
/// [std]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html
/// [comparison]: index.html#comparison-to-standard-library
///
/// # Examples
///
/// ```
/// use dary_heap::BinaryHeap;
///
/// // Type inference lets us omit an explicit type signature (which
/// // would be `BinaryHeap<i32>` in this example).
/// let mut heap = BinaryHeap::new();
///
/// // We can use peek to look at the next item in the heap. In this case,
/// // there's no items in there yet so we get None.
/// assert_eq!(heap.peek(), None);
///
/// // Let's add some scores...
/// heap.push(1);
/// heap.push(5);
/// heap.push(2);
///
/// // Now peek shows the most important item in the heap.
/// assert_eq!(heap.peek(), Some(&5));
///
/// // We can check the length of a heap.
/// assert_eq!(heap.len(), 3);
///
/// // We can iterate over the items in the heap, although they are returned in
/// // a random order.
/// for x in &heap {
///     println!("{x}");
/// }
///
/// // If we instead pop these scores, they should come back in order.
/// assert_eq!(heap.pop(), Some(5));
/// assert_eq!(heap.pop(), Some(2));
/// assert_eq!(heap.pop(), Some(1));
/// assert_eq!(heap.pop(), None);
///
/// // We can clear the heap of any remaining items.
/// heap.clear();
///
/// // The heap should now be empty.
/// assert!(heap.is_empty())
/// ```
///
/// A `DaryHeap` with a known list of items can be initialized from an array:
///
/// ```
/// use dary_heap::QuaternaryHeap;
///
/// let heap = QuaternaryHeap::from([1, 5, 2]);
/// ```
///
/// ## Min-heap
///
/// Either [`core::cmp::Reverse`] or a custom [`Ord`] implementation can be used to
/// make `DaryHeap` a min-heap. This makes `heap.pop()` return the smallest
/// value instead of the greatest one.
///
/// ```
/// use dary_heap::TernaryHeap;
/// use std::cmp::Reverse;
///
/// let mut heap = TernaryHeap::new();
///
/// // Wrap values in `Reverse`
/// heap.push(Reverse(1));
/// heap.push(Reverse(5));
/// heap.push(Reverse(2));
///
/// // If we pop these scores now, they should come back in the reverse order.
/// assert_eq!(heap.pop(), Some(Reverse(1)));
/// assert_eq!(heap.pop(), Some(Reverse(2)));
/// assert_eq!(heap.pop(), Some(Reverse(5)));
/// assert_eq!(heap.pop(), None);
/// ```
///
/// # Time complexity
///
/// | [push]  | [pop]         | [peek]/[peek\_mut] |
/// |---------|---------------|--------------------|
/// | *O*(1)~ | *O*(log(*n*)) | *O*(1)             |
///
/// The value for `push` is an expected cost; the method documentation gives a
/// more detailed analysis.
///
/// [`core::cmp::Reverse`]: core::cmp::Reverse
/// [`Ord`]: core::cmp::Ord
/// [`Cell`]: core::cell::Cell
/// [`RefCell`]: core::cell::RefCell
/// [push]: DaryHeap::push
/// [pop]: DaryHeap::pop
/// [peek]: DaryHeap::peek
/// [peek\_mut]: DaryHeap::peek_mut
pub struct DaryHeap<T, const D: usize> {
    data: Vec<T>,
}

/// Structure wrapping a mutable reference to the greatest item on a
/// `DaryHeap`.
///
/// This `struct` is created by the [`peek_mut`] method on [`DaryHeap`]. See
/// its documentation for more.
///
/// [`peek_mut`]: DaryHeap::peek_mut
pub struct PeekMut<'a, T: 'a + Ord, const D: usize> {
    heap: &'a mut DaryHeap<T, D>,
    sift: bool,
}

impl<T: Ord + fmt::Debug, const D: usize> fmt::Debug for PeekMut<'_, T, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PeekMut").field(&self.heap.data[0]).finish()
    }
}

impl<T: Ord, const D: usize> Drop for PeekMut<'_, T, D> {
    fn drop(&mut self) {
        if self.sift {
            // SAFETY: PeekMut is only instantiated for non-empty heaps.
            unsafe { self.heap.sift_down(0) };
        }
    }
}

impl<T: Ord, const D: usize> Deref for PeekMut<'_, T, D> {
    type Target = T;
    fn deref(&self) -> &T {
        debug_assert!(!self.heap.is_empty());
        // SAFE: PeekMut is only instantiated for non-empty heaps
        unsafe { self.heap.data.get_unchecked(0) }
    }
}

impl<T: Ord, const D: usize> DerefMut for PeekMut<'_, T, D> {
    fn deref_mut(&mut self) -> &mut T {
        debug_assert!(!self.heap.is_empty());
        self.sift = true;
        // SAFE: PeekMut is only instantiated for non-empty heaps
        unsafe { self.heap.data.get_unchecked_mut(0) }
    }
}

impl<'a, T: Ord, const D: usize> PeekMut<'a, T, D> {
    /// Removes the peeked value from the heap and returns it.
    pub fn pop(mut this: PeekMut<'a, T, D>) -> T {
        let value = this.heap.pop().unwrap();
        this.sift = false;
        value
    }
}

impl<T: Clone, const D: usize> Clone for DaryHeap<T, D> {
    fn clone(&self) -> Self {
        DaryHeap {
            data: self.data.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.data.clone_from(&source.data);
    }
}

impl<T: Ord, const D: usize> Default for DaryHeap<T, D> {
    /// Creates an empty `DaryHeap<T, D>`.
    #[inline]
    fn default() -> DaryHeap<T, D> {
        DaryHeap::new()
    }
}

impl<T: fmt::Debug, const D: usize> fmt::Debug for DaryHeap<T, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: Ord, const D: usize> DaryHeap<T, D> {
    /// Creates an empty `DaryHeap` as a max-heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::QuaternaryHeap;
    /// let mut heap = QuaternaryHeap::new();
    /// heap.push(4);
    /// ```
    #[must_use]
    pub fn new() -> DaryHeap<T, D> {
        DaryHeap { data: vec![] }
    }

    /// Creates an empty `DaryHeap` with a specific capacity.
    /// This preallocates enough memory for `capacity` elements,
    /// so that the `DaryHeap` does not have to be reallocated
    /// until it contains at least that many values.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::QuaternaryHeap;
    /// let mut heap = QuaternaryHeap::with_capacity(10);
    /// heap.push(4);
    /// ```
    #[must_use]
    pub fn with_capacity(capacity: usize) -> DaryHeap<T, D> {
        DaryHeap {
            data: Vec::with_capacity(capacity),
        }
    }

    /// Returns a mutable reference to the greatest item in the *d*-ary heap, or
    /// `None` if it is empty.
    ///
    /// Note: If the `PeekMut` value is leaked, the heap may be in an
    /// inconsistent state.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::TernaryHeap;
    /// let mut heap = TernaryHeap::new();
    /// assert!(heap.peek_mut().is_none());
    ///
    /// heap.push(1);
    /// heap.push(5);
    /// heap.push(2);
    /// {
    ///     let mut val = heap.peek_mut().unwrap();
    ///     *val = 0;
    /// }
    /// assert_eq!(heap.peek(), Some(&2));
    /// ```
    ///
    /// # Time complexity
    ///
    /// If the item is modified then the worst case time complexity is *O*(log(*n*)),
    /// otherwise it's *O*(1).
    pub fn peek_mut(&mut self) -> Option<PeekMut<'_, T, D>> {
        if self.is_empty() {
            None
        } else {
            Some(PeekMut {
                heap: self,
                sift: false,
            })
        }
    }

    /// Removes the greatest item from the *d*-ary heap and returns it, or `None` if it
    /// is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::BinaryHeap;
    /// let mut heap = BinaryHeap::from([1, 3]);
    ///
    /// assert_eq!(heap.pop(), Some(3));
    /// assert_eq!(heap.pop(), Some(1));
    /// assert_eq!(heap.pop(), None);
    /// ```
    ///
    /// # Time complexity
    ///
    /// The worst case cost of `pop` on a heap containing *n* elements is *O*(log(*n*)).
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop().map(|mut item| {
            if !self.is_empty() {
                swap(&mut item, &mut self.data[0]);
                // SAFETY: !self.is_empty() means that self.len() > 0
                unsafe { self.sift_down_to_bottom(0) };
            }
            item
        })
    }

    /// Pushes an item onto the *d*-ary heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::QuaternaryHeap;
    /// let mut heap = QuaternaryHeap::new();
    /// heap.push(3);
    /// heap.push(5);
    /// heap.push(1);
    ///
    /// assert_eq!(heap.len(), 3);
    /// assert_eq!(heap.peek(), Some(&5));
    /// ```
    ///
    /// # Time complexity
    ///
    /// The expected cost of `push`, averaged over every possible ordering of
    /// the elements being pushed, and over a sufficiently large number of
    /// pushes, is *O*(1). This is the most meaningful cost metric when pushing
    /// elements that are *not* already in any sorted pattern.
    ///
    /// The time complexity degrades if elements are pushed in predominantly
    /// ascending order. In the worst case, elements are pushed in ascending
    /// sorted order and the amortized cost per push is *O*(log(*n*)) against a heap
    /// containing *n* elements.
    ///
    /// The worst case cost of a *single* call to `push` is *O*(*n*). The worst case
    /// occurs when capacity is exhausted and needs a resize. The resize cost
    /// has been amortized in the previous figures.
    pub fn push(&mut self, item: T) {
        let old_len = self.len();
        self.data.push(item);
        // SAFETY: Since we pushed a new item it means that
        //  old_len = self.len() - 1 < self.len()
        unsafe { self.sift_up(0, old_len) };
    }

    /// Consumes the `DaryHeap` and returns a vector in sorted
    /// (ascending) order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::OctonaryHeap;
    ///
    /// let mut heap = OctonaryHeap::from([1, 2, 4, 5, 7]);
    /// heap.push(6);
    /// heap.push(3);
    ///
    /// let vec = heap.into_sorted_vec();
    /// assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7]);
    /// ```
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn into_sorted_vec(mut self) -> Vec<T> {
        let mut end = self.len();
        while end > 1 {
            end -= 1;
            // SAFETY: `end` goes from `self.len() - 1` to 1 (both included),
            //  so it's always a valid index to access.
            //  It is safe to access index 0 (i.e. `ptr`), because
            //  1 <= end < self.len(), which means self.len() >= 2.
            unsafe {
                let ptr = self.data.as_mut_ptr();
                ptr::swap(ptr, ptr.add(end));
            }
            // SAFETY: `end` goes from `self.len() - 1` to 1 (both included) so:
            //  0 < 1 <= end <= self.len() - 1 < self.len()
            //  Which means 0 < end and end < self.len().
            unsafe { self.sift_down_range(0, end) };
        }
        self.into_vec()
    }

    // The implementations of sift_up and sift_down use unsafe blocks in
    // order to move an element out of the vector (leaving behind a
    // hole), shift along the others and move the removed element back into the
    // vector at the final location of the hole.
    // The `Hole` type is used to represent this, and make sure
    // the hole is filled back at the end of its scope, even on panic.
    // Using a hole reduces the constant factor compared to using swaps,
    // which involves twice as many moves.

    /// # Safety
    ///
    /// The caller must guarantee that `pos < self.len()`.
    unsafe fn sift_up(&mut self, start: usize, pos: usize) -> usize {
        assert_ne!(D, 0, "Arity should be greater than zero");
        // Take out the value at `pos` and create a hole.
        // SAFETY: The caller guarantees that pos < self.len()
        let mut hole = Hole::new(&mut self.data, pos);

        while hole.pos() > start {
            let parent = (hole.pos() - 1) / D;

            // SAFETY: hole.pos() > start >= 0, which means hole.pos() > 0
            //  and so hole.pos() - 1 can't underflow.
            //  This guarantees that parent < hole.pos() so
            //  it's a valid index and also != hole.pos().
            if hole.element() <= hole.get(parent) {
                break;
            }

            // SAFETY: Same as above
            hole.move_to(parent);
        }

        hole.pos()
    }

    /// Take an element at `pos` and move it down the heap,
    /// while its children are larger.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `pos < end <= self.len()`.
    unsafe fn sift_down_range(&mut self, pos: usize, end: usize) {
        assert_ne!(D, 0, "Arity should be greater than zero");
        // SAFETY: The caller guarantees that pos < end <= self.len().
        let mut hole = Hole::new(&mut self.data, pos);
        let mut child = D * hole.pos() + 1;

        // Loop invariant: child == d * hole.pos() + 1.
        while child <= end.saturating_sub(D) {
            // compare with the greatest of the d children
            // SAFETY: child < end - d + 1 < self.len() and
            //  child + d - 1 < end <= self.len(), so they're valid indexes.
            //  child + i == d * hole.pos() + 1 + i != hole.pos() for i >= 0
            child = hole.max_sibling::<D>(child);

            // if we are already in order, stop.
            // SAFETY: child is now either the old child or valid sibling
            //  We already proven that all are < self.len() and != hole.pos()
            if hole.element() >= hole.get(child) {
                return;
            }

            // SAFETY: same as above.
            hole.move_to(child);
            child = D * hole.pos() + 1;
        }

        child = hole.max_sibling_to::<D>(child, end);
        // SAFETY: && short circuit, which means that in the
        //  second condition it's already true that child < end <= self.len().
        if child < end && hole.element() < hole.get(child) {
            // SAFETY: child is already proven to be a valid index and
            //  child == d * hole.pos() + 1 != hole.pos().
            hole.move_to(child);
        }
    }

    /// # Safety
    ///
    /// The caller must guarantee that `pos < self.len()`.
    unsafe fn sift_down(&mut self, pos: usize) {
        let len = self.len();
        // SAFETY: pos < len is guaranteed by the caller and
        //  obviously len = self.len() <= self.len().
        self.sift_down_range(pos, len);
    }

    /// Take an element at `pos` and move it all the way down the heap,
    /// then sift it up to its position.
    ///
    /// Note: This is faster when the element is known to be large / should
    /// be closer to the bottom.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `pos < self.len()`.
    unsafe fn sift_down_to_bottom(&mut self, mut pos: usize) {
        assert_ne!(D, 0, "Arity should be greater than zero");
        let end = self.len();
        let start = pos;

        // SAFETY: The caller guarantees that pos < self.len().
        let mut hole = Hole::new(&mut self.data, pos);
        let mut child = D * hole.pos() + 1;

        // Loop invariant: child == d * hole.pos() + 1.
        while child <= end.saturating_sub(D) {
            // SAFETY: child < end - d + 1 < self.len() and
            //  child + d - 1 < end <= self.len(), so they're valid indexes.
            //  child + i == d * hole.pos() + 1 + i != hole.pos() for i >= 0
            child = hole.max_sibling::<D>(child);

            // SAFETY: Same as above
            hole.move_to(child);
            child = D * hole.pos() + 1;
        }

        child = hole.max_sibling_to::<D>(child, end);
        if child < end {
            // SAFETY: child < end <= self.len(), so it's a valid index
            //  and child == d * hole.pos() + i != hole.pos() for i >= 1
            hole.move_to(child);
        }
        pos = hole.pos();
        drop(hole);

        // SAFETY: pos is the position in the hole and was already proven
        //  to be a valid index.
        self.sift_up(start, pos);
    }

    /// Rebuild assuming data[0..start] is still a proper heap.
    fn rebuild_tail(&mut self, start: usize) {
        assert_ne!(D, 0, "Arity should be greater than zero");

        if start == self.len() {
            return;
        }

        let tail_len = self.len() - start;

        // The fix for this lint (usize::BITS) requires Rust 1.53.0, but the
        // MSRV is currently 1.51.0.
        #[allow(clippy::manual_bits)]
        #[inline(always)]
        fn log2_fast(x: usize) -> usize {
            8 * size_of::<usize>() - (x.leading_zeros() as usize) - 1
        }

        // `rebuild` takes O(self.len()) operations
        // and about n * self.len() comparisons in the worst case
        // with n = d / (d - 1)
        // while repeating `sift_up` takes O(tail_len * log(start)) operations
        // and about 1 * tail_len * log(start) comparisons in the worst case,
        // assuming start >= tail_len. For larger heaps, the crossover point
        // no longer follows this reasoning and was determined empirically.
        let better_to_rebuild = if start < tail_len {
            true
        } else if self.len() <= 4096 / D {
            D * self.len() < (D - 1) * tail_len * log2_fast(start)
        } else {
            D * self.len() < (D - 1) * tail_len * 13usize.saturating_sub(D)
        };

        if better_to_rebuild {
            self.rebuild();
        } else {
            for i in start..self.len() {
                // SAFETY: The index `i` is always less than self.len().
                unsafe { self.sift_up(0, i) };
            }
        }
    }

    fn rebuild(&mut self) {
        assert_ne!(D, 0, "Arity should be greater than zero");
        if self.len() < 2 {
            return;
        }
        let mut n = (self.len() - 1) / D + 1;
        while n > 0 {
            n -= 1;
            // SAFETY: n starts from (self.len() - 1) / d + 1 and goes down to 0.
            //  The only case when !(n < self.len()) is if
            //  self.len() == 0, but it's ruled out by the loop condition.
            unsafe { self.sift_down(n) };
        }
    }

    /// Moves all the elements of `other` into `self`, leaving `other` empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::OctonaryHeap;
    ///
    /// let mut a = OctonaryHeap::from([-10, 1, 2, 3, 3]);
    /// let mut b = OctonaryHeap::from([-20, 5, 43]);
    ///
    /// a.append(&mut b);
    ///
    /// assert_eq!(a.into_sorted_vec(), [-20, -10, 1, 2, 3, 3, 5, 43]);
    /// assert!(b.is_empty());
    /// ```
    pub fn append(&mut self, other: &mut Self) {
        if self.len() < other.len() {
            swap(self, other);
        }

        let start = self.data.len();

        self.data.append(&mut other.data);

        self.rebuild_tail(start);
    }
}

impl<T, const D: usize> DaryHeap<T, D> {
    /// Returns an iterator visiting all values in the underlying vector, in
    /// arbitrary order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::TernaryHeap;
    /// let heap = TernaryHeap::from([1, 2, 3, 4]);
    ///
    /// // Print 1, 2, 3, 4 in arbitrary order
    /// for x in heap.iter() {
    ///     println!("{x}");
    /// }
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            iter: self.data.iter(),
        }
    }

    /// Returns the greatest item in the *d*-ary heap, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::BinaryHeap;
    /// let mut heap = BinaryHeap::new();
    /// assert_eq!(heap.peek(), None);
    ///
    /// heap.push(1);
    /// heap.push(5);
    /// heap.push(2);
    /// assert_eq!(heap.peek(), Some(&5));
    ///
    /// ```
    ///
    /// # Time complexity
    ///
    /// Cost is *O*(1) in the worst case.
    #[must_use]
    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    /// Returns the number of elements the *d*-ary heap can hold without reallocating.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::OctonaryHeap;
    /// let mut heap = OctonaryHeap::with_capacity(100);
    /// assert!(heap.capacity() >= 100);
    /// heap.push(4);
    /// ```
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Reserves the minimum capacity for exactly `additional` more elements to be inserted in the
    /// given `DaryHeap`. Does nothing if the capacity is already sufficient.
    ///
    /// Note that the allocator may give the collection more space than it requests. Therefore
    /// capacity can not be relied upon to be precisely minimal. Prefer [`reserve`] if future
    /// insertions are expected.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::OctonaryHeap;
    /// let mut heap = OctonaryHeap::new();
    /// heap.reserve_exact(100);
    /// assert!(heap.capacity() >= 100);
    /// heap.push(4);
    /// ```
    ///
    /// [`reserve`]: DaryHeap::reserve
    pub fn reserve_exact(&mut self, additional: usize) {
        self.data.reserve_exact(additional);
    }

    /// Reserves capacity for at least `additional` more elements to be inserted in the
    /// `DaryHeap`. The collection may reserve more space to avoid frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::BinaryHeap;
    /// let mut heap = BinaryHeap::new();
    /// heap.reserve(100);
    /// assert!(heap.capacity() >= 100);
    /// heap.push(4);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    /// Discards as much additional capacity as possible.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::TernaryHeap;
    /// let mut heap: TernaryHeap<i32> = TernaryHeap::with_capacity(100);
    ///
    /// assert!(heap.capacity() >= 100);
    /// heap.shrink_to_fit();
    /// assert!(heap.capacity() == 0);
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }

    /// Returns a slice of all values in the underlying vector, in arbitrary
    /// order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::OctonaryHeap;
    /// use std::io::{self, Write};
    ///
    /// let heap = OctonaryHeap::from([1, 2, 3, 4, 5, 6, 7]);
    ///
    /// io::sink().write(heap.as_slice()).unwrap();
    /// ```
    #[cfg(feature = "unstable")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unstable")))]
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }

    /// Consumes the `DaryHeap` and returns the underlying vector
    /// in arbitrary order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::QuaternaryHeap;
    /// let heap = QuaternaryHeap::from([1, 2, 3, 4, 5, 6, 7]);
    /// let vec = heap.into_vec();
    ///
    /// // Will print in some order
    /// for x in vec {
    ///     println!("{x}");
    /// }
    /// ```
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn into_vec(self) -> Vec<T> {
        self.into()
    }

    /// Returns the length of the *d*-ary heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::BinaryHeap;
    /// let heap = BinaryHeap::from([1, 3]);
    ///
    /// assert_eq!(heap.len(), 2);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the *d*-ary heap is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::BinaryHeap;
    /// let mut heap = BinaryHeap::new();
    ///
    /// assert!(heap.is_empty());
    ///
    /// heap.push(3);
    /// heap.push(5);
    /// heap.push(1);
    ///
    /// assert!(!heap.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears the *d*-ary heap, returning an iterator over the removed elements
    /// in arbitrary order. If the iterator is dropped before being fully
    /// consumed, it drops the remaining elements in arbitrary order.
    ///
    /// The returned iterator keeps a mutable borrow on the heap to optimize
    /// its implementation.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::QuaternaryHeap;
    /// let mut heap = QuaternaryHeap::from([1, 3]);
    ///
    /// assert!(!heap.is_empty());
    ///
    /// for x in heap.drain() {
    ///     println!("{x}");
    /// }
    ///
    /// assert!(heap.is_empty());
    /// ```
    #[inline]
    pub fn drain(&mut self) -> Drain<'_, T> {
        Drain {
            iter: self.data.drain(..),
        }
    }

    /// Drops all items from the *d*-ary heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::TernaryHeap;
    /// let mut heap = TernaryHeap::from([1, 3]);
    ///
    /// assert!(!heap.is_empty());
    ///
    /// heap.clear();
    ///
    /// assert!(heap.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.drain();
    }
}

/// Hole represents a hole in a slice i.e., an index without valid value
/// (because it was moved from or duplicated).
/// In drop, `Hole` will restore the slice by filling the hole
/// position with the value that was originally removed.
struct Hole<'a, T: 'a> {
    data: &'a mut [T],
    elt: ManuallyDrop<T>,
    pos: usize,
}

impl<'a, T> Hole<'a, T> {
    /// Create a new `Hole` at index `pos`.
    ///
    /// Unsafe because pos must be within the data slice.
    #[inline]
    unsafe fn new(data: &'a mut [T], pos: usize) -> Self {
        debug_assert!(pos < data.len());
        // SAFE: pos should be inside the slice
        let elt = ptr::read(data.get_unchecked(pos));
        Hole {
            data,
            elt: ManuallyDrop::new(elt),
            pos,
        }
    }

    #[inline]
    fn pos(&self) -> usize {
        self.pos
    }

    /// Returns a reference to the element removed.
    #[inline]
    fn element(&self) -> &T {
        &self.elt
    }

    /// Returns a reference to the element at `index`.
    ///
    /// Unsafe because index must be within the data slice and not equal to pos.
    #[inline]
    unsafe fn get(&self, index: usize) -> &T {
        debug_assert!(index != self.pos);
        debug_assert!(index < self.data.len());
        self.data.get_unchecked(index)
    }

    /// Move hole to new location
    ///
    /// Unsafe because index must be within the data slice and not equal to pos.
    #[inline]
    unsafe fn move_to(&mut self, index: usize) {
        debug_assert!(index != self.pos);
        debug_assert!(index < self.data.len());
        let ptr = self.data.as_mut_ptr();
        let index_ptr: *const _ = ptr.add(index);
        let hole_ptr = ptr.add(self.pos);
        ptr::copy_nonoverlapping(index_ptr, hole_ptr, 1);
        self.pos = index;
    }
}

impl<'a, T: Ord> Hole<'a, T> {
    /// Get largest element
    ///
    /// Unsafe because both elements must be within the data slice and not equal
    /// to pos.
    #[inline]
    unsafe fn max(&self, elem1: usize, elem2: usize) -> usize {
        if self.get(elem1) <= self.get(elem2) {
            elem2
        } else {
            elem1
        }
    }

    /// Get index of greatest sibling
    ///
    /// Unsafe because all siblings must be within the data slice and not equal
    /// to pos.
    #[inline]
    unsafe fn max_sibling<const D: usize>(&self, first_sibling: usize) -> usize {
        let mut sibling = first_sibling;
        match D {
            2 => {
                sibling += (self.get(sibling) <= self.get(sibling + 1)) as usize;
            }
            3 => {
                let sibling_a = self.max_sibling::<2>(sibling);
                let sibling_b = sibling + 2;
                sibling = self.max(sibling_a, sibling_b);
            }
            4 => {
                let sibling_a = self.max_sibling::<2>(sibling);
                let sibling_b = self.max_sibling::<2>(sibling + 2);
                sibling = self.max(sibling_a, sibling_b);
            }
            _ => {
                for other_sibling in sibling + 1..sibling + D {
                    if self.get(sibling) <= self.get(other_sibling) {
                        sibling = other_sibling;
                    }
                }
            }
        }
        sibling
    }

    /// Get index of greatest sibling within range
    ///
    /// Unsafe because end must be the length of the data slice, last sibling
    /// must be outside of the data slice and no sibling may be equal to pos.
    /// It is allowed for first_sibling to be outside of the data slice.
    #[inline]
    unsafe fn max_sibling_to<const D: usize>(&self, first_sibling: usize, end: usize) -> usize {
        let mut sibling = first_sibling;
        match D {
            2 => {}
            3 => {
                if sibling + 1 < end {
                    sibling = self.max_sibling::<2>(sibling);
                }
            }
            _ => {
                for other_sibling in sibling + 1..end {
                    if self.get(sibling) <= self.get(other_sibling) {
                        sibling = other_sibling;
                    }
                }
            }
        }
        sibling
    }
}

impl<T> Drop for Hole<'_, T> {
    #[inline]
    fn drop(&mut self) {
        // fill the hole again
        unsafe {
            let pos = self.pos;
            ptr::copy_nonoverlapping(&*self.elt, self.data.get_unchecked_mut(pos), 1);
        }
    }
}

/// An iterator over the elements of a `DaryHeap`.
///
/// This `struct` is created by [`DaryHeap::iter()`]. See its
/// documentation for more.
///
/// [`iter`]: DaryHeap::iter
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Iter<'a, T: 'a> {
    iter: slice::Iter<'a, T>,
}

impl<T: fmt::Debug> fmt::Debug for Iter<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Iter").field(&self.iter.as_slice()).finish()
    }
}

// FIXME(#26925) Remove in favor of `#[derive(Clone)]`
impl<T> Clone for Iter<'_, T> {
    fn clone(&self) -> Self {
        Iter {
            iter: self.iter.clone(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn last(self) -> Option<&'a T> {
        self.iter.last()
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a T> {
        self.iter.next_back()
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {}

impl<T> FusedIterator for Iter<'_, T> {}

/// An owning iterator over the elements of a `DaryHeap`.
///
/// This `struct` is created by [`DaryHeap::into_iter()`]
/// (provided by the [`IntoIterator`] trait). See its documentation for more.
///
/// [`into_iter`]: DaryHeap::into_iter
/// [`IntoIterator`]: core::iter::IntoIterator
#[derive(Clone)]
pub struct IntoIter<T> {
    iter: vec::IntoIter<T>,
}

impl<T: fmt::Debug> fmt::Debug for IntoIter<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IntoIter")
            .field(&self.iter.as_slice())
            .finish()
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}

impl<T> FusedIterator for IntoIter<T> {}

/// A draining iterator over the elements of a `DaryHeap`.
///
/// This `struct` is created by [`DaryHeap::drain()`]. See its
/// documentation for more.
///
/// [`drain`]: DaryHeap::drain
#[derive(Debug)]
pub struct Drain<'a, T: 'a> {
    iter: vec::Drain<'a, T>,
}

impl<T> Iterator for Drain<'_, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> DoubleEndedIterator for Drain<'_, T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

impl<T> ExactSizeIterator for Drain<'_, T> {}

impl<T> FusedIterator for Drain<'_, T> {}

impl<T: Ord, const D: usize> From<Vec<T>> for DaryHeap<T, D> {
    /// Converts a `Vec<T>` into a `DaryHeap<T, D>`.
    ///
    /// This conversion happens in-place, and has *O*(*n*) time complexity.
    fn from(vec: Vec<T>) -> DaryHeap<T, D> {
        let mut heap = DaryHeap { data: vec };
        heap.rebuild();
        heap
    }
}

impl<T: Ord, const D: usize, const N: usize> From<[T; N]> for DaryHeap<T, D> {
    /// ```
    /// use dary_heap::TernaryHeap;
    ///
    /// let mut h1 = TernaryHeap::from([1, 4, 2, 3]);
    /// let mut h2: TernaryHeap<_> = [1, 4, 2, 3].into();
    /// while let Some((a, b)) = h1.pop().zip(h2.pop()) {
    ///     assert_eq!(a, b);
    /// }
    /// ```
    fn from(arr: [T; N]) -> Self {
        // With newer Rust versions `Self::from_iter(arr)` should be used, as
        // using `IntoIter::new` is deprecated from 1.59.0. However, this would
        // require a MSRV of 1.53.0, and both are equivalent behind the scenes.
        #[allow(deprecated)]
        core::array::IntoIter::new(arr).collect()
    }
}

impl<T, const D: usize> From<DaryHeap<T, D>> for Vec<T> {
    /// Converts a `DaryHeap<T, D>` into a `Vec<T>`.
    ///
    /// This conversion requires no data movement or allocation, and has
    /// constant time complexity.
    fn from(heap: DaryHeap<T, D>) -> Vec<T> {
        heap.data
    }
}

impl<T: Ord, const D: usize> FromIterator<T> for DaryHeap<T, D> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> DaryHeap<T, D> {
        DaryHeap::from(iter.into_iter().collect::<Vec<_>>())
    }
}

impl<T, const D: usize> IntoIterator for DaryHeap<T, D> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Creates a consuming iterator, that is, one that moves each value out of
    /// the *d*-ary heap in arbitrary order. The *d*-ary heap cannot be used
    /// after calling this.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dary_heap::BinaryHeap;
    /// let heap = BinaryHeap::from([1, 2, 3, 4]);
    ///
    /// // Print 1, 2, 3, 4 in arbitrary order
    /// for x in heap.into_iter() {
    ///     // x has type i32, not &i32
    ///     println!("{x}");
    /// }
    /// ```
    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            iter: self.data.into_iter(),
        }
    }
}

impl<'a, T, const D: usize> IntoIterator for &'a DaryHeap<T, D> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<T: Ord, const D: usize> Extend<T> for DaryHeap<T, D> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.extend_desugared(iter.into_iter());
    }
}

impl<T: Ord, const D: usize> DaryHeap<T, D> {
    fn extend_desugared<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let iterator = iter.into_iter();
        let (lower, _) = iterator.size_hint();

        self.reserve(lower);

        iterator.for_each(move |elem| self.push(elem));
    }
}

impl<'a, T: 'a + Ord + Copy, const D: usize> Extend<&'a T> for DaryHeap<T, D> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().cloned());
    }
}
