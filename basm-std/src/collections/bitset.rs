/// A fixed-size bitset backed by a constant number of `u64` words.
///
/// `N` is the total number of bits, and `WORDS` is the number of `u64` entries
/// required to store those bits. `WORDS` must be at least `(N + 63) / 64`.
#[derive(Debug, Clone, PartialEq)]
pub struct BitSet<const N: usize, const WORDS: usize> {
    data: [u64; WORDS],
}

impl<const N: usize, const WORDS: usize> Default for BitSet<N, WORDS> {
     fn default() -> Self {
         Self::new()
     }
}

impl<const N: usize, const WORDS: usize> BitSet<N, WORDS> {
    /// Creates a new bitset with all bits cleared.
    ///
    /// # Panics
    ///
    /// Panics if `WORDS` is not sufficient to store `N` bits.
    pub const fn new() -> Self {
        assert!(WORDS >= N.div_ceil(64), "WORDS must be sufficient for N bits");
        Self {
            data: [0; WORDS],
        }
    }

    /// Clears all bits in the bitset.
    pub fn clear(&mut self) {
        self.data.fill(0);
    }

    /// Sets the bit at the given index. Returns `false` if the index is out of bounds.
    pub fn set(&mut self, index: usize) -> bool {
        if index >= N {
            return false;
        }
        
        let word_index = index / 64;
        let bit_index = index % 64;
        
        self.data[word_index] |= 1u64 << bit_index;
        true
    }

    /// Unsets the bit at the given index. Returns `false` if the index is out of bounds.
    pub fn unset(&mut self, index: usize) -> bool {
        if index >= N {
            return false;
        }
        
        let word_index = index / 64;
        let bit_index = index % 64;
        
        self.data[word_index] &= !(1u64 << bit_index);
        true
    }

    /// Toggles the bit at the given index. Returns `false` if the index is out of bounds.
    pub fn toggle(&mut self, index: usize) -> bool {
        if index >= N {
            return false;
        }
        
        let word_index = index / 64;
        let bit_index = index % 64;
        
        self.data[word_index] ^= 1u64 << bit_index;
        true
    }

    /// Returns `true` if the bit at the given index is set. Returns `false` if index is out of bounds.
    pub fn is_set(&self, index: usize) -> bool {
        if index >= N {
            return false;
        }
        
        let word_index = index / 64;
        let bit_index = index % 64;
        
        (self.data[word_index] & (1u64 << bit_index)) != 0
    }

    /// Returns the number of bits set to 1.
    pub fn count_ones(&self) -> usize {
        self.data.iter().map(|&word| word.count_ones() as usize).sum()
    }

    /// Returns the number of bits set to 0.
    pub fn count_zeros(&self) -> usize {
        N - self.count_ones()
    }

    /// Returns `true` if all bits are 0.
    pub fn is_empty(&self) -> bool {
        self.data.iter().all(|&word| word == 0)
    }

    /// Returns `true` if all bits are 1.
    pub fn is_full(&self) -> bool {
        let full_words = N / 64;
        let remaining_bits = N % 64;
        
        for i in 0..full_words {
            if self.data[i] != u64::MAX {
                return false;
            }
        }
        
        if remaining_bits > 0 {
            let mask = (1u64 << remaining_bits) - 1;
            if (self.data[full_words] & mask) != mask {
                return false;
            }
        }
        
        true
    }

    /// Returns the index of the first bit set to 1, or `None` if none are set.
    pub fn first_set(&self) -> Option<usize> {
        for (word_idx, &word) in self.data.iter().enumerate() {
            if word != 0 {
                let bit_idx = word.trailing_zeros() as usize;
                let global_idx = word_idx * 64 + bit_idx;
                if global_idx < N {
                    return Some(global_idx);
                }
            }
        }
        None
    }

    /// Returns the index of the last bit set to 1, or `None` if none are set.
    pub fn last_set(&self) -> Option<usize> {
        for (word_idx, &word) in self.data.iter().enumerate().rev() {
            if word != 0 {
                let bit_idx = 63 - word.leading_zeros() as usize;
                let global_idx = word_idx * 64 + bit_idx;
                if global_idx < N {
                    return Some(global_idx);
                }
            }
        }
        None
    }

    /// Returns a new bitset which is the intersection (AND) of `self` and `other`.
    pub fn intersection(&self, other: &Self) -> Self {
        let mut result = Self::new();
        for i in 0..WORDS {
            result.data[i] = self.data[i] & other.data[i];
        }
        result
    }

    /// Returns a new bitset which is the union (OR) of `self` and `other`.
    pub fn union(&self, other: &Self) -> Self {
        let mut result = Self::new();
        for i in 0..WORDS {
            result.data[i] = self.data[i] | other.data[i];
        }
        result
    }

    /// Returns a new bitset which is the difference (XOR) of `self` and `other`.
    pub fn difference(&self, other: &Self) -> Self {
        let mut result = Self::new();
        for i in 0..WORDS {
            result.data[i] = self.data[i] ^ other.data[i];
        }
        result
    }

    /// Returns the bitwise NOT of `self` (i.e., all bits flipped).
    pub fn complement(&self) -> Self {
        let mut result = Self::new();
        for i in 0..WORDS {
            result.data[i] = !self.data[i];
        }
        
        let remaining_bits = N % 64;
        if remaining_bits > 0 {
            let mask = (1u64 << remaining_bits) - 1;
            result.data[WORDS - 1] &= mask;
        }
        
        result
    }

    /// Returns an iterator over the indices of set bits (bits with value `1`).
    pub fn iter_ones<'a>(&'a self) -> BitSetIterator<'a, N, WORDS> {
        BitSetIterator {
            bitset: self,
            current_word: 0,
            current_bit: 0,
        }
    }

    /// Sets all bits in the bitset.
    pub fn fill_all(&mut self) {
        let full_words = N / 64;
        for i in 0..full_words {
            self.data[i] = u64::MAX;
        }

        let remaining_bits = N % 64;
        if remaining_bits > 0 {
            let mask = (1u64 << remaining_bits) - 1;
            self.data[full_words] = mask;
        }
    }
}

/// Iterator over the indices of set bits in a `BitSet`.
pub struct BitSetIterator<'a, const N: usize, const WORDS: usize> {
    bitset: &'a BitSet<N, WORDS>,
    current_word: usize,
    current_bit: usize,
}

impl<'a, const N: usize, const WORDS: usize> Iterator for BitSetIterator<'a, N, WORDS> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_word < WORDS {
            let word = self.bitset.data[self.current_word];
            
            while self.current_bit < 64 {
                let global_index = self.current_word * 64 + self.current_bit;
                
                if global_index >= N {
                    return None;
                }
                
                if (word & (1u64 << self.current_bit)) != 0 {
                    self.current_bit += 1;
                    return Some(global_index);
                }
                
                self.current_bit += 1;
            }

            self.current_word += 1;
            self.current_bit = 0;
        }
        
        None
    }
}

/// Type alias for a 64-bit bitset.
pub type BitSet64 = BitSet<64, 1>;
/// Type alias for a 128-bit bitset.
pub type BitSet128 = BitSet<128, 2>;
/// Type alias for a 256-bit bitset.
pub type BitSet256 = BitSet<256, 4>;
/// Type alias for a 512-bit bitset.
pub type BitSet512 = BitSet<512, 8>;
/// Type alias for a 1024-bit bitset.
pub type BitSet1024 = BitSet<1024, 16>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut bitset = BitSet64::new();
        assert!(bitset.set(5));
        assert!(bitset.set(10));
        assert!(bitset.set(15));
        assert!(bitset.is_set(5));
        assert!(bitset.is_set(10));
        assert!(bitset.is_set(15));
        assert!(!bitset.is_set(0));
        assert_eq!(bitset.count_ones(), 3);
        assert_eq!(bitset.count_zeros(), 61);
        assert!(bitset.unset(10));
        assert!(!bitset.is_set(10));
        assert_eq!(bitset.count_ones(), 2);
    }

    #[test]
    fn test_iterator() {
        let mut bitset = BitSet64::new();
        bitset.set(1);
        bitset.set(5);
        bitset.set(10);
        bitset.set(20);
        
        let ones: Vec<usize> = bitset.iter_ones().collect();
        assert_eq!(ones, vec![1, 5, 10, 20]);
    }

    #[test]
    fn test_set_operations() {
        let mut bitset1 = BitSet64::new();
        let mut bitset2 = BitSet64::new();
        
        bitset1.set(1);
        bitset1.set(3);
        bitset1.set(5);
        
        bitset2.set(3);
        bitset2.set(5);
        bitset2.set(7);
        
        let intersection = bitset1.intersection(&bitset2);
        assert!(intersection.is_set(3));
        assert!(intersection.is_set(5));
        assert!(!intersection.is_set(1));
        assert!(!intersection.is_set(7));
        
        let union = bitset1.union(&bitset2);
        assert!(union.is_set(1));
        assert!(union.is_set(3));
        assert!(union.is_set(5));
        assert!(union.is_set(7));
    }

    #[test]
    fn test_bounds_checking() {
        let mut bitset = BitSet64::new();
        
        assert!(bitset.set(0));
        assert!(bitset.set(63));
        
        assert!(!bitset.set(64));
        assert!(!bitset.set(100));
        assert!(!bitset.is_set(64));
    }
}