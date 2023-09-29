use alloc::string::String;
use core::mem::MaybeUninit;
use crate::platform::services;

pub struct Reader<const N: usize = { super::DEFAULT_BUF_SIZE }> {
    buf: [MaybeUninit<u8>; N],
    len: usize,
    off: usize,
}

impl<const N: usize> Default for Reader<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Reader<N> {
    const _DUMMY: usize = {
        assert!(N >= super::MIN_BUF_SIZE, "Buffer size for Writer must be at least MIN_BUF_SIZE");
        0
    };
    pub fn new() -> Self {
        Self {
            buf: MaybeUninit::uninit_array(),
            len: 0,
            off: 0
        }
    }
    pub fn try_refill(&mut self, readahead: usize) -> usize {
        /* readahead cannot exceed the buffer size */
        assert!(readahead <= self.buf.len());
        if self.off + readahead <= self.len {
            /* data already available */
            return readahead;
        }
        /* secure space by discarding the already-consumed buffer contents at front */
        let mut end = self.off + readahead;
        if end > self.buf.len() {
            let rem = self.len - self.off;
            unsafe { core::ptr::copy(self.buf[self.off..].as_ptr(), self.buf.as_mut_ptr(), rem); }
            self.len = rem;
            self.off = 0;
            end = readahead;
        }
        unsafe {
            /* Although the buffer currently falls short of what has been requested,
             * it may still be possible that a full token (which is short)
             * is available within the remains. Thus, we check if we can return
             * without invoking read_stdio. This is crucial for cases where
             * the standard input is a pipe, which includes the local testing
             * console environment. */
            let white_pos = MaybeUninit::slice_assume_init_ref(&self.buf[self.off..self.len]).iter().position(|&b| b <= b' ');
            if white_pos.is_none() {
                /* No white space has been found. We have to read.
                 * We try to read as much as possible at once. */
                self.len += services::read_stdio(0, MaybeUninit::slice_assume_init_mut(&mut self.buf[self.len..]));
            }
            /* zero-fill unread portion for SIMD-accelerated unsafe integer read routines */
            if self.len < end {
                MaybeUninit::slice_assume_init_mut(&mut self.buf[self.len..end]).fill(0u8);
            }
        }
        core::cmp::min(readahead, self.len - self.off)
    }
    pub fn try_consume(&mut self, bytes: usize) -> usize {
        let mut consumed = 0;
        while consumed < bytes {
            if self.off == self.len {
                if self.try_refill(1) == 0 { break; }
            }
            let delta = core::cmp::min(self.len - self.off, bytes - consumed);
            self.off += delta;
            consumed -= delta;
        }
        consumed
    }
    pub fn skip_until_whitespace(&mut self) -> usize {
        let mut len = 0;
        #[target_feature(enable = "avx2")]
        unsafe fn nonwhite(s: &[u8]) -> Option<usize> {
            s.iter().position(|&c| c > b' ')
        }
        loop {
            let pos = unsafe { nonwhite(self.remain()) };
            if let Some(pos) = pos {
                len += pos;
                self.off += pos;
                break len;
            }
            len += self.len - self.off;
            self.off = self.len;
            self.try_refill(1);
        }
    }
    pub fn until(&mut self, delim: u8, buf: &mut String) -> usize {
        #[target_feature(enable = "avx2,sse4.2")]
        unsafe fn memchr(s: &[u8], delim: u8) -> Option<usize> {
            s.iter().position(|&b| b == delim)
        }
        let mut total = 0;
        loop {
            let len = self.len - self.off;
            let range = unsafe { MaybeUninit::slice_assume_init_ref(&self.buf[self.off..self.off + len]) };
            if let Some(i) = unsafe { memchr(range, delim) } {
                unsafe { buf.as_mut_vec() }.extend_from_slice(&range[..i]);
                self.off += i + 1;
                break total + i;
            } else {
                unsafe { buf.as_mut_vec() }.extend_from_slice(&range);
                self.off = self.len;
                self.try_refill(1);
                total += len;
            }
        }
    }
    pub fn remain(&self) -> &[u8] {
        unsafe { MaybeUninit::slice_assume_init_ref(&self.buf[self.off..self.len]) }
    }
    pub fn discard(&mut self, until: u8) -> usize {
        let mut len = 0;
        #[target_feature(enable = "avx2")]
        unsafe fn index(s: &[u8], b: u8) -> Option<usize> {
            s.iter().position(|&c| c == b)
        }
        loop {
            let pos = unsafe { index(self.remain(), until) };
            if let Some(pos) = pos {
                len += pos;
                self.off += pos + 1;
                break len;
            }
            len += self.len - self.off;
            self.off = self.len;
            self.try_refill(1);
        }
    }

    fn noskip_u32(&mut self) -> u32 {
        let mut c = unsafe { self.buf[self.off..].as_ptr().cast::<u64>().read_unaligned() };
        let m = !c & 0x1010101010101010;
        let len = m.trailing_zeros() >> 3;
        c <<= (8 - len) << 3;
        c = (c & 0x0F0F0F0F0F0F0F0F).wrapping_mul(2561) >> 8;
        c = (c & 0x00FF00FF00FF00FF).wrapping_mul(6553601) >> 16;
        c = (c & 0x0000FFFF0000FFFF).wrapping_mul(42949672960001) >> 32;
        self.off += len as usize;
        if len == 8 {
            if unsafe { self.buf[self.off].assume_init() } & 0x10 != 0 {
                c *= 10;
                c += (unsafe { self.buf[self.off].assume_init() } - b'0') as u64;
                self.off += 1;
            }
            if unsafe { self.buf[self.off].assume_init() } & 0x10 != 0 {
                c *= 10;
                c += (unsafe { self.buf[self.off].assume_init() } - b'0') as u64;
                self.off += 1;
            }
        }
        self.off += 1;
        c as u32
    }
    fn noskip_u64(&mut self) -> u64 {
        let mut c = unsafe { self.buf[self.off..].as_ptr().cast::<u64>().read_unaligned() };
        let m = !c & 0x1010101010101010;
        let len = m.trailing_zeros() >> 3;
        c <<= (8 - len) << 3;
        c = (c & 0x0F0F0F0F0F0F0F0F).wrapping_mul(2561) >> 8;
        c = (c & 0x00FF00FF00FF00FF).wrapping_mul(6553601) >> 16;
        c = (c & 0x0000FFFF0000FFFF).wrapping_mul(42949672960001) >> 32;
        self.off += len as usize;
        if len == 8 && unsafe { self.buf[self.off].assume_init() } & 16 != 0 {
            let mut d = unsafe { self.buf[self.off..].as_ptr().cast::<u64>().read_unaligned() };
            let m = !d & 0x1010101010101010;
            let len = m.trailing_zeros() >> 3;
            for _ in 0..len {
                c *= 10;
            }
            d <<= (8 - len) << 3;
            d = (d & 0x0F0F0F0F0F0F0F0F).wrapping_mul(2561) >> 8;
            d = (d & 0x00FF00FF00FF00FF).wrapping_mul(6553601) >> 16;
            d = (d & 0x0000FFFF0000FFFF).wrapping_mul(42949672960001) >> 32;
            c += d;
            self.off += len as usize;
            if len == 8 {
                if unsafe { self.buf[self.off].assume_init() } & 0x10 != 0 {
                    c *= 10;
                    c += (unsafe { self.buf[self.off].assume_init() } - b'0') as u64;
                    self.off += 1;
                }
                if unsafe { self.buf[self.off].assume_init() } & 0x10 != 0 {
                    c *= 10;
                    c += (unsafe { self.buf[self.off].assume_init() } - b'0') as u64;
                    self.off += 1;
                }
                if unsafe { self.buf[self.off].assume_init() } & 0x10 != 0 {
                    c *= 10;
                    c += (unsafe { self.buf[self.off].assume_init() } - b'0') as u64;
                    self.off += 1;
                }
            }
        }
        self.off += 1;
        c
    }
    fn noskip_u128(&mut self) -> u128 {
        let mut n = 0;
        loop {
            let b = unsafe { self.buf[self.off].assume_init() };
            if b > 32 {
                n *= 10;
                n += b as u128 & 0x0F;
                self.off += 1;
            } else {
                break n;
            }
        }
    }

    pub fn i8(&mut self) -> i8 {
        self.i32() as i8
    }
    pub fn u8(&mut self) -> u8 {
        self.u32() as u8
    }
    pub fn i16(&mut self) -> i16 {
        self.i32() as i16
    }
    pub fn u16(&mut self) -> u16 {
        self.u32() as u16
    }
    pub fn i32(&mut self) -> i32 {
        self.skip_until_whitespace();
        self.try_refill(12);
        let sign = unsafe { self.buf[self.off].assume_init() } == b'-';
        (if sign {
            self.off += 1;
            self.noskip_u32().wrapping_neg()
        } else {
            self.noskip_u32()
        }) as i32
    }
    pub fn u32(&mut self) -> u32 {
        self.skip_until_whitespace();
        self.try_refill(11);
        self.noskip_u32()
    }
    pub fn i64(&mut self) -> i64 {
        self.skip_until_whitespace();
        self.try_refill(22);
        let sign = unsafe { self.buf[self.off].assume_init() } == b'-';
        (if sign {
            self.off += 1;
            self.noskip_u64().wrapping_neg()
        } else {
            self.noskip_u64()
        }) as i64
    }
    pub fn u64(&mut self) -> u64 {
        self.skip_until_whitespace();
        self.try_refill(21);
        self.noskip_u64()
    }
    pub fn i128(&mut self) -> i128 {
        self.skip_until_whitespace();
        self.try_refill(41);
        let sign = unsafe { self.buf[self.off].assume_init() } == b'-';
        (if sign {
            self.off += 1;
            self.noskip_u128().wrapping_neg()
        } else {
            self.noskip_u128()
        }) as i128
    }
    pub fn u128(&mut self) -> u128 {
        self.skip_until_whitespace();
        self.try_refill(40);
        self.noskip_u128()
    }
    #[cfg(target_pointer_width = "32")]
    pub fn isize(&mut self) -> isize {
        self.i32() as isize
    }
    #[cfg(target_pointer_width = "32")]
    pub fn usize(&mut self) -> usize {
        self.u32() as usize
    }
    #[cfg(target_pointer_width = "64")]
    pub fn isize(&mut self) -> isize {
        self.i64() as isize
    }
    #[cfg(target_pointer_width = "64")]
    pub fn usize(&mut self) -> usize {
        self.u64() as usize
    }
    #[cfg(all(not(target_pointer_width = "32"), not(target_pointer_width = "64")))]
    pub fn isize(&mut self) -> isize {
        self.i128() as isize;
    }
    #[cfg(all(not(target_pointer_width = "32"), not(target_pointer_width = "64")))]
    pub fn usize(&mut self) -> usize {
        self.u128() as usize;
    }
}