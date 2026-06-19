use alloc::boxed::Box;

/// Li Chao segment tree for lines of the form `y = m * x + b` with `i64` coefficients.
///
/// The tree maintains a set of lines and supports queries of the form:
///   - Given `x`, return the minimum (or maximum) value over all lines at `x`.
///
/// Coordinates are in a fixed `x`-range `[lo, hi]` given at construction time.
/// All operations are `O(log(hi - lo + 1))`.
///
/// Example:
/// ```
/// use basm_std::collections::LiChaoTree;
///
/// // Maintain minimum values, for x in [-1_000_000, 1_000_000].
/// let mut lc = LiChaoTree::new(-1_000_000, 1_000_000, true);
///
/// // Add y = 2x + 3
/// lc.add_line(2, 3);
///
/// // Add y = -x + 10
/// lc.add_line(-1, 10);
///
/// // Query at x = 5
/// let ans = lc.query(5);
/// // ans is min(2*5 + 3, -5 + 10) = min(13, 5) = 5
/// ```
pub struct LiChaoTree {
    root: Option<Box<Node>>,
    lo: i64,
    hi: i64,
    is_min: bool,
}

#[derive(Clone, Copy)]
struct Line {
    m: i64,
    b: i64,
}

impl Line {
    #[inline]
    fn value(&self, x: i64) -> i64 {
        // Use saturating arithmetic for some safety against overflow.
        self.m.saturating_mul(x).saturating_add(self.b)
    }
}

struct Node {
    line: Line,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl LiChaoTree {
    const INF: i64 = (1_i64 << 60);

    /// Creates a new Li Chao Tree on the inclusive range `[lo, hi]`.
    ///
    /// If `is_min` is `true`, the tree returns minimum values.
    /// If `is_min` is `false`, the tree returns maximum values.
    pub fn new(lo: i64, hi: i64, is_min: bool) -> Self {
        assert!(lo <= hi);
        Self {
            root: None,
            lo,
            hi,
            is_min,
        }
    }

    /// Inserts a new line `y = m * x + b` into the structure.
    pub fn add_line(&mut self, m: i64, b: i64) {
        let line = Line { m, b };
        Self::add_line_inner(&mut self.root, self.lo, self.hi, line, self.is_min);
    }

    /// Queries the structure at a single point `x`.
    ///
    /// If no line has been inserted, returns:
    /// * `+INF` for a min tree
    /// * `-INF` for a max tree
    pub fn query(&self, x: i64) -> i64 {
        if self.root.is_none() {
            return if self.is_min { Self::INF } else { -Self::INF };
        }
        Self::query_inner(&self.root, self.lo, self.hi, x, self.is_min).unwrap_or(if self.is_min {
            Self::INF
        } else {
            -Self::INF
        })
    }

    #[inline]
    fn better(is_min: bool, new_val: i64, cur_val: i64) -> bool {
        if is_min {
            new_val < cur_val
        } else {
            new_val > cur_val
        }
    }

    fn add_line_inner(
        node: &mut Option<Box<Node>>,
        l: i64,
        r: i64,
        mut new_line: Line,
        is_min: bool,
    ) {
        if node.is_none() {
            *node = Some(Box::new(Node {
                line: new_line,
                left: None,
                right: None,
            }));
            return;
        }

        let n = node.as_mut().unwrap();
        let mid = l + (r - l) / 2;

        // Current line stored at this node
        let cur = &mut n.line;

        // 1) Ensure `cur` is the better line at `mid`.
        let cur_mid = cur.value(mid);
        let new_mid = new_line.value(mid);
        if Self::better(is_min, new_mid, cur_mid) {
            core::mem::swap(cur, &mut new_line);
        }

        // 2) Leaf: nothing more to do.
        if l == r {
            return;
        }

        // 3) Decide where the "other" line (`new_line`) can still be better.
        let cur_l = cur.value(l);
        let cur_r = cur.value(r);
        let new_l = new_line.value(l);
        let new_r = new_line.value(r);

        if Self::better(is_min, new_l, cur_l) {
            // On the left side, new_line is better.
            Self::add_line_inner(&mut n.left, l, mid, new_line, is_min);
        } else if Self::better(is_min, new_r, cur_r) {
            // On the right side, new_line is better.
            Self::add_line_inner(&mut n.right, mid + 1, r, new_line, is_min);
        }
    }

    fn query_inner(node: &Option<Box<Node>>, l: i64, r: i64, x: i64, is_min: bool) -> Option<i64> {
        let n = node.as_ref()?;

        let mut res = Some(n.line.value(x));
        if l == r {
            return res;
        }

        let mid = l + (r - l) / 2;
        let child_res = if x <= mid {
            Self::query_inner(&n.left, l, mid, x, is_min)
        } else {
            Self::query_inner(&n.right, mid + 1, r, x, is_min)
        };

        if let Some(v) = child_res {
            if let Some(cur) = res {
                if Self::better(is_min, v, cur) {
                    res = Some(v);
                }
            } else {
                res = Some(v);
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::LiChaoTree;

    #[test]
    fn single_line_min() {
        let mut lc = LiChaoTree::new(-10, 10, true);
        lc.add_line(2, 3); // y = 2x + 3
        assert_eq!(lc.query(0), 3);
        assert_eq!(lc.query(5), 13);
        assert_eq!(lc.query(-5), -7);
    }

    #[test]
    fn single_line_max() {
        let mut lc = LiChaoTree::new(-10, 10, false);
        lc.add_line(2, 3); // y = 2x + 3
        assert_eq!(lc.query(0), 3);
        assert_eq!(lc.query(5), 13);
        assert_eq!(lc.query(-5), -7);
    }

    #[test]
    fn two_lines_min() {
        // y1 = x + 2, y2 = 2x + 3 (min envelope)
        let mut lc = LiChaoTree::new(-100, 100, true);
        lc.add_line(1, 2);
        lc.add_line(2, 3);

        // At x = -100:
        // y1 = -98, y2 = -197 => min = -197
        assert_eq!(lc.query(-100), -197);

        // At x = 100:
        // y1 = 102, y2 = 203 => min = 102
        assert_eq!(lc.query(100), 102);
    }

    #[test]
    fn two_lines_max() {
        // y1 = x + 2, y2 = 2x + 3 (max envelope)
        let mut lc = LiChaoTree::new(-100, 100, false);
        lc.add_line(1, 2);
        lc.add_line(2, 3);

        // At x = -100:
        // y1 = -98, y2 = -197 => max = -98
        assert_eq!(lc.query(-100), -98);

        // At x = 100:
        // y1 = 102, y2 = 203 => max = 203
        assert_eq!(lc.query(100), 203);
    }
}
