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
        let mut out = Self {
            buf: MaybeUninit::uninit_array(),
            len: 0,
            off: 0
        };
        out.try_refill(N);
        out
    }
    pub fn try_refill(&mut self, readahead: usize) {
        /* readahead cannot exceed the buffer size */
        assert!(readahead <= self.buf.len());
        if self.off + readahead <= self.len {
            /* data already available */
            return;
        }
        if self.off + readahead > self.buf.len() {
            /* secure space by discarding the already-consumed buffer contents at front */
            let rem = self.len - self.off;
            unsafe { core::ptr::copy(self.buf[self.off].as_ptr(), MaybeUninit::array_assume_init(self.buf).as_mut_ptr(), rem); }
            self.len = rem;
            self.off = 0;
        }
        /* try to read as much as possible at once */
        self.len += services::read_stdio(0, unsafe {
            MaybeUninit::slice_assume_init_mut(&mut self.buf[self.len..])
        });
    }
    pub fn consume(&mut self, bytes: usize) {
        self.off += bytes;
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

    pub fn i32(&mut self) -> i32 {
        let sign = unsafe { self.buf[self.off].assume_init() } == b'-';
        (if sign {
            self.off += 1;
            self.u32().wrapping_neg()
        } else {
            self.u32()
        }) as i32
    }
    pub fn u32(&mut self) -> u32 {
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
    pub fn i64(&mut self) -> i64 {
        let sign = unsafe { self.buf[self.off].assume_init() } == b'-';
        (if sign {
            self.off += 1;
            self.u64().wrapping_neg()
        } else {
            self.u64()
        }) as i64
    }
    pub fn u64(&mut self) -> u64 {
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
}