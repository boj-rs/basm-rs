use crate::platform::services;
use alloc::string::String;
use core::mem::MaybeUninit;
use core::str::FromStr;

pub trait Readable {
    fn read(reader: &mut impl ReaderTrait) -> Self;
}

mod position {
    #[cfg_attr(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature(enable = "avx2")
    )]
    pub unsafe fn white(s: &[u8]) -> Option<usize> {
        s.iter().position(|&c| c <= b' ')
    }
    #[cfg_attr(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature(enable = "avx2")
    )]
    pub unsafe fn newline(s: &[u8]) -> Option<usize> {
        s.iter().position(|&c| c == b'\n')
    }
    #[cfg_attr(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature(enable = "avx2,sse4.2")
    )]
    pub unsafe fn memchr(s: &[u8], delim: u8) -> Option<usize> {
        s.iter().position(|&b| b == delim)
    }
}

/// Note: _internal prefix solely for avoiding name clash with public method
trait ReaderBufferTrait: Sized {
    fn try_refill_internal(&mut self, readahead: usize) -> usize;
    fn remain_internal(&self) -> &[u8];
    fn advance(&mut self, bytes: usize); // raw functionality (cf. try_consume: has sanity checks)

    #[cfg(any(not(feature = "short"), feature = "fastio"))]
    fn noskip_u64(&mut self) -> u64 {
        const POW10: [u32; 9] = [
            1,
            10,
            100,
            1_000,
            10_000,
            100_000,
            1_000_000,
            10_000_000,
            100_000_000,
        ];
        // For speed optimization, we explicitly unroll the first step of the integer parsing.
        // By avoiding multiplication by POW10 in the first step, we gain speed.
        let mut c = unsafe {
            self.remain_internal()
                .as_ptr()
                .cast::<u64>()
                .read_unaligned()
        };
        let m = !c & 0x1010101010101010;
        let len = m.trailing_zeros() >> 3;
        self.advance(len as usize);
        c &= 0x0F0F0F0F0F0F0F0F;
        c <<= (8 - len) << 3;
        c = c.wrapping_mul(2561) >> 8;
        c = (c & 0x00FF00FF00FF00FF).wrapping_mul(6553601) >> 16;
        let mut out = (c & 0xFFFF) * 10000 + (c >> 32);
        loop {
            let mut c = unsafe {
                self.remain_internal()
                    .as_ptr()
                    .cast::<u64>()
                    .read_unaligned()
            };
            if c & 0x10 == 0 {
                /* len == 0 */
                break out;
            }
            let m = !c & 0x1010101010101010;
            let len = m.trailing_zeros() >> 3;
            self.advance(len as usize);
            out *= POW10[len as usize] as u64;
            c &= 0x0F0F0F0F0F0F0F0F;
            c <<= (8 - len) << 3;
            c = c.wrapping_mul(2561) >> 8;
            c = (c & 0x00FF00FF00FF00FF).wrapping_mul(6553601) >> 16;
            out += (c & 0xFFFF) * 10000 + (c >> 32);
        }
    }
    #[cfg(all(feature = "short", not(feature = "fastio")))]
    fn noskip_u64(&mut self) -> u64 {
        let mut n = 0;
        loop {
            let data = self.remain();
            if data.is_empty() || data[0] <= b' ' {
                // no more data available, or whitespace (delimiter) reached
                break n;
            } else {
                n *= 10;
                n += data[0] as u64 & 0x0F;
                self.advance(1);
            }
        }
    }
    fn noskip_u128(&mut self) -> u128 {
        let mut n = 0;
        'outer: loop {
            let data = self.remain();
            if data.is_empty() {
                // no more data available
                break n;
            }
            for (i, &b) in data.iter().enumerate() {
                if b > 32 {
                    n *= 10;
                    n += b as u128 & 0x0F;
                } else {
                    self.advance(i);
                    break 'outer n;
                }
            }
            self.advance(data.len());
        }
    }
}

pub trait ReaderTrait: Sized {
    fn try_refill(&mut self, readahead: usize) -> usize;
    fn try_consume(&mut self, bytes: usize) -> usize;
    fn remain(&self) -> &[u8];
    fn skip_until_whitespace(&mut self) -> usize;
    fn until(&mut self, delim: u8, buf: &mut String) -> usize;
    fn discard(&mut self, until: u8) -> usize;
    fn word_buf(&mut self, buf: &mut [u8]) -> usize;
    fn word_to_string(&mut self, buf: &mut String);
    fn line_buf(&mut self, buf: &mut [u8]) -> usize;
    fn line_to_string(&mut self, buf: &mut String);
    fn is_eof(&mut self) -> bool;
    fn is_eof_skip_whitespace(&mut self) -> bool;
    fn i8(&mut self) -> i8;
    fn i16(&mut self) -> i16;
    fn i32(&mut self) -> i32;
    fn i64(&mut self) -> i64;
    fn i128(&mut self) -> i128;
    fn isize(&mut self) -> isize;
    fn u8(&mut self) -> u8;
    fn u16(&mut self) -> u16;
    fn u32(&mut self) -> u32;
    fn u64(&mut self) -> u64;
    fn u128(&mut self) -> u128;
    fn usize(&mut self) -> usize;
    fn f64(&mut self) -> f64;
    fn byte(&mut self) -> u8;
    fn word(&mut self) -> String;
    fn line(&mut self) -> String;
    fn skip_whitespace(&mut self) -> usize;
    fn next<T: Readable>(&mut self) -> T {
        T::read(self)
    }
    fn take<T: Readable>(&mut self, n: usize) -> impl Iterator<Item = T> {
        (0..n).map(|_| T::read(self))
    }
    /// Reads and collects `n` elements of type `T`.
    fn collect<Cn: FromIterator<T>, T: Readable>(&mut self, n: usize) -> Cn {
        Cn::from_iter((0..n).map(|_| T::read(self)))
    }
    /// Reads and collects an `n`-by-`m` matrix of type `T`.
    fn collect_2d<Cnm: FromIterator<Cm>, Cm: FromIterator<T>, T: Readable>(
        &mut self,
        n: usize,
        m: usize,
    ) -> Cnm {
        Cnm::from_iter((0..n).map(|_| Cm::from_iter((0..m).map(|_| T::read(self)))))
    }
    /// Reads and collects an `n`-by-`m`-by-`p` tensor of type `T`.
    fn collect_3d<
        Cnmp: FromIterator<Cmp>,
        Cmp: FromIterator<Cp>,
        Cp: FromIterator<T>,
        T: Readable,
    >(
        &mut self,
        n: usize,
        m: usize,
        p: usize,
    ) -> Cnmp {
        Cnmp::from_iter(
            (0..n).map(|_| {
                Cmp::from_iter((0..m).map(|_| Cp::from_iter((0..p).map(|_| T::read(self)))))
            }),
        )
    }
}

impl<T: ReaderBufferTrait> ReaderTrait for T {
    fn try_refill(&mut self, readahead: usize) -> usize {
        self.try_refill_internal(readahead)
    }
    fn try_consume(&mut self, bytes: usize) -> usize {
        let mut consumed = 0;
        while consumed < bytes {
            let data = self.remain();
            let len = data.len();
            if data.is_empty() && self.try_refill(1) == 0 {
                break;
            }
            let delta = core::cmp::min(len, bytes - consumed);
            self.advance(delta);
            consumed += delta;
        }
        consumed
    }
    fn remain(&self) -> &[u8] {
        self.remain_internal()
    }
    fn skip_until_whitespace(&mut self) -> usize {
        let mut total = 0;
        'outer: loop {
            let data = self.remain();
            for (i, &b) in data.iter().enumerate() {
                if b <= b' ' {
                    total += i;
                    self.advance(i);
                    break 'outer total;
                }
            }
            self.advance(data.len());
            if self.try_refill(1) == 0 {
                break total;
            }
        }
    }
    fn until(&mut self, delim: u8, buf: &mut String) -> usize {
        let mut total = 0;
        loop {
            let range = self.remain();
            let len = range.len();
            if let Some(i) = unsafe { position::memchr(range, delim) } {
                unsafe { buf.as_mut_vec() }.extend_from_slice(&range[..i]);
                self.advance(i + 1);
                break total + i;
            } else {
                unsafe { buf.as_mut_vec() }.extend_from_slice(range);
                self.advance(len);
                total += len;
                if self.try_refill(1) == 0 {
                    break total;
                }
            }
        }
    }
    fn discard(&mut self, until: u8) -> usize {
        let mut total = 0;
        loop {
            let range = self.remain();
            let len = range.len();
            let pos = unsafe { position::memchr(range, until) };
            if let Some(pos) = pos {
                total += pos;
                self.advance(pos + 1);
                break total;
            }
            total += len;
            self.advance(len);
            if self.try_refill(1) == 0 {
                break total;
            }
        }
    }

    fn word_buf(&mut self, buf: &mut [u8]) -> usize {
        self.skip_whitespace();
        let mut total = 0;
        while total < buf.len() {
            let range = self.remain();
            let len = range.len();
            if len == 0 {
                // no more data available
                break;
            }

            let rem = core::cmp::min(len, buf.len() - total);
            let data = &range[..rem];
            if let Some(pos) = unsafe { position::white(data) } {
                buf[total..total + pos].copy_from_slice(&data[..pos]);
                total += pos;
                self.advance(pos);
                break;
            } else {
                buf[total..total + rem].copy_from_slice(data);
                total += rem;
                self.advance(rem);
                self.try_refill(1);
            }
        }
        total
    }
    fn word_to_string(&mut self, buf: &mut String) {
        self.skip_whitespace();
        loop {
            let data = self.remain();
            let len = data.len();
            if len == 0 {
                // no more data available
                break;
            }

            if let Some(pos) = unsafe { position::white(data) } {
                unsafe { buf.as_mut_vec() }.extend_from_slice(&data[..pos]);
                self.advance(pos);
                break;
            } else {
                unsafe { buf.as_mut_vec() }.extend_from_slice(data);
                self.advance(len);
                self.try_refill(1);
            }
        }
    }
    fn line_buf(&mut self, buf: &mut [u8]) -> usize {
        self.skip_whitespace();
        let mut total = 0;
        while total < buf.len() {
            let range = self.remain();
            let len = range.len();
            if len == 0 {
                // no more data available
                break;
            }

            let rem = core::cmp::min(len, buf.len() - total);
            let data = &range[..rem];
            if let Some(pos) = unsafe { position::newline(data) } {
                let pos_out = if pos > 0 && data[pos - 1] == b'\r' {
                    pos - 1
                } else {
                    pos
                };
                buf[total..total + pos_out].copy_from_slice(&data[..pos_out]);
                total += pos_out;
                self.advance(pos);
                break;
            } else {
                buf[total..total + rem].copy_from_slice(data);
                total += rem;
                self.advance(rem);
                self.try_refill(1);
            }
        }
        total
    }
    fn line_to_string(&mut self, buf: &mut String) {
        self.try_refill(1);
        loop {
            let data = self.remain();
            let len = data.len();
            if len == 0 {
                // no more data available
                break;
            }

            if let Some(pos) = unsafe { position::newline(data) } {
                let pos_out = if pos > 0 && data[pos - 1] == b'\r' {
                    pos - 1
                } else {
                    pos
                };
                unsafe { buf.as_mut_vec() }.extend_from_slice(&data[..pos_out]);
                self.advance(pos + 1);
                break;
            } else {
                unsafe { buf.as_mut_vec() }.extend_from_slice(data);
                self.advance(len);
                self.try_refill(1);
            }
        }
    }
    fn is_eof(&mut self) -> bool {
        let mut range = self.remain();
        if range.is_empty() {
            self.try_refill(1);
            range = self.remain();
        }
        range.is_empty()
    }
    fn is_eof_skip_whitespace(&mut self) -> bool {
        self.skip_whitespace();
        self.remain().is_empty()
    }
    fn word(&mut self) -> String {
        let mut buf = String::new();
        self.word_to_string(&mut buf);
        buf
    }
    fn line(&mut self) -> String {
        let mut buf = String::new();
        self.line_to_string(&mut buf);
        buf
    }
    fn i8(&mut self) -> i8 {
        self.i32() as i8
    }
    fn u8(&mut self) -> u8 {
        self.u32() as u8
    }
    fn i16(&mut self) -> i16 {
        self.i32() as i16
    }
    fn u16(&mut self) -> u16 {
        self.u32() as u16
    }
    fn i32(&mut self) -> i32 {
        self.skip_whitespace();
        self.try_refill(17);
        let sign = self.remain()[0] == b'-';
        (if sign {
            self.advance(1);
            self.noskip_u64().wrapping_neg()
        } else {
            self.noskip_u64()
        }) as i32
    }
    fn u32(&mut self) -> u32 {
        self.skip_whitespace();
        self.try_refill(16);
        self.noskip_u64() as u32
    }
    fn i64(&mut self) -> i64 {
        self.skip_whitespace();
        self.try_refill(25);
        let sign = if unsafe { self.remain_internal().as_ptr().read_unaligned() } == b'-' {
            self.advance(1);
            2
        } else {
            0
        };
        self.noskip_u64()
            .wrapping_mul(1u64.wrapping_sub(sign as u64)) as i64
    }
    fn u64(&mut self) -> u64 {
        self.skip_whitespace();
        self.try_refill(24);
        self.noskip_u64()
    }
    fn i128(&mut self) -> i128 {
        self.skip_whitespace();
        self.try_refill(41);
        let sign = self.remain()[0] == b'-';
        (if sign {
            self.advance(1);
            self.noskip_u128().wrapping_neg()
        } else {
            self.noskip_u128()
        }) as i128
    }
    fn u128(&mut self) -> u128 {
        self.skip_whitespace();
        self.try_refill(40);
        self.noskip_u128()
    }
    #[cfg(target_pointer_width = "32")]
    fn isize(&mut self) -> isize {
        self.i32() as isize
    }
    #[cfg(target_pointer_width = "32")]
    fn usize(&mut self) -> usize {
        self.u32() as usize
    }
    #[cfg(target_pointer_width = "64")]
    fn isize(&mut self) -> isize {
        self.i64() as isize
    }
    #[cfg(target_pointer_width = "64")]
    fn usize(&mut self) -> usize {
        self.u64() as usize
    }
    #[cfg(all(not(target_pointer_width = "32"), not(target_pointer_width = "64")))]
    fn isize(&mut self) -> isize {
        self.i128() as isize
    }
    #[cfg(all(not(target_pointer_width = "32"), not(target_pointer_width = "64")))]
    fn usize(&mut self) -> usize {
        self.u128() as usize
    }
    fn f64(&mut self) -> f64 {
        /* For simplicity, we assume the input string is at most 64 bytes.
         * Strings longer than this length are either incorrectly parsed
         * (scientific notations get their exponents truncated) or approximately parsed
         * (decimal notations get their tails truncated yielding approximately
         * correct outputs). */
        self.skip_whitespace();
        self.try_refill(64);
        let data = self.remain();
        let mut end = 0;
        while end < data.len() && data[end] > b' ' {
            end += 1;
        }
        if end == 0 {
            f64::NAN
        } else {
            let s = unsafe { core::str::from_utf8_unchecked(&data[..end]) };
            let out = f64::from_str(s);
            self.skip_until_whitespace();
            if let Ok(ans) = out { ans } else { f64::NAN }
        }
    }
    fn byte(&mut self) -> u8 {
        self.try_refill(1);
        let mut out = 0u8;
        let data = self.remain();
        if !data.is_empty() {
            out = data[0];
            self.advance(1);
        }
        out
    }
    // We do not use avx2 for this function since most of the time
    // we only skip a few whitespaces.
    fn skip_whitespace(&mut self) -> usize {
        let mut total = 0;
        'outer: loop {
            loop {
                let data = self.remain();
                if data.is_empty() {
                    break;
                } else if data[0] > b' ' {
                    break 'outer total;
                } else {
                    total += 1;
                    self.advance(1);
                }
            }
            if self.try_refill(1) == 0 {
                break total;
            }
        }
    }
}

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
    const BUF_LEN: usize = N - 8;
    const DUMMY: () = assert!(
        N >= super::MIN_BUF_SIZE,
        "Buffer size for Reader must be at least MIN_BUF_SIZE"
    );
    pub fn new() -> Self {
        #[allow(clippy::let_unit_value)]
        let _ = Self::DUMMY;
        Self {
            buf: [const { MaybeUninit::uninit() }; N],
            len: 0,
            off: 0,
        }
    }
}

impl<const N: usize> ReaderBufferTrait for Reader<N> {
    fn try_refill_internal(&mut self, readahead: usize) -> usize {
        /* readahead cannot exceed the buffer size */
        assert!(readahead <= Self::BUF_LEN);
        unsafe {
            let mut rem = self.len - self.off;
            if rem < readahead {
                #[cfg(all(feature = "short", not(feature = "fastio")))]
                {
                    /* 1) For short and not(fastio), we skip non-essential checks that enhance usability on console.
                     * 2) We explicitly use `rep movsb` on x64 to ensure generation of short code.
                     */
                    #[cfg(target_arch = "x86_64")]
                    core::arch::asm!(
                        "rep movsb",
                        in("rsi") self.buf.as_ptr().wrapping_add(self.off),
                        in("rdi") self.buf.as_mut_ptr(),
                        in("rcx") rem,
                        lateout("rsi") _,
                        lateout("rdi") _,
                        lateout("rcx") _,
                    );
                    #[cfg(not(target_arch = "x86_64"))]
                    for i in 0..rem {
                        *self.buf[i].assume_init_mut() = self.buf[self.off + i].assume_init();
                    }
                    rem += services::read_stdio(0, self.buf[rem..Self::BUF_LEN].assume_init_mut());
                }
                #[cfg(any(not(feature = "short"), feature = "fastio"))]
                {
                    /* Secure space by discarding the already-consumed buffer contents at front.
                     * Note that we expect `readahead` to be small (<100 bytes), so we unconditionally
                     * copy the contents to the front to reduce code size. When the default buffer size
                     * is used (which is >100K), this will not happen often and hence shouldn't affect
                     * performance by a noticeable amount. */
                    let mut white_cnt = 0u32;
                    let mut j = self.off;
                    for i in 0..rem {
                        let c = self.buf[j].assume_init();
                        if c <= b' ' {
                            white_cnt += 1;
                        }
                        *self.buf[i].assume_init_mut() = c;
                        j += 1;
                    }

                    /* Although the buffer currently falls short of what has been requested,
                     * it may still be possible that a full token (which is short)
                     * is available within the remains. Thus, we check if we can return
                     * without invoking read_stdio. This is crucial for cases where
                     * the standard input is a pipe, which includes the local testing
                     * console environment. */
                    if white_cnt == 0 {
                        /* No whitespace has been found. We have to read.
                         * We try to read as much as possible at once. */
                        rem +=
                            services::read_stdio(0, self.buf[rem..Self::BUF_LEN].assume_init_mut());
                    }
                }

                /* Add a null-terminator, whether or not the read was nonsaturating (for SIMD-accelerated unsafe integer read routines).
                 * This is safe since we spare 8 bytes at the end of the buffer. */
                *self.buf[rem].assume_init_mut() = 0u8;

                /* Save the new data length */
                self.len = rem;
                self.off = 0;
            } else {
                /* data already available */
            }
            rem
        }
    }
    fn remain_internal(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self.buf.assume_init_ref().as_ptr().wrapping_add(self.off),
                self.len - self.off,
            )
        }
    }
    fn advance(&mut self, bytes: usize) {
        self.off += bytes;
    }
}

#[cfg(all(target_arch = "x86_64", not(test)))]
pub struct MmapReader {
    buf: *const u8,
    end: *const u8,
}

#[cfg(all(target_arch = "x86_64", not(test)))]
impl Default for MmapReader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(all(target_arch = "x86_64", not(test)))]
impl MmapReader {
    pub fn new() -> Self {
        #[cfg(not(feature = "submit"))]
        {
            let pd = crate::platform::services::platform_data();
            assert!(
                pd.env_id == crate::platform::services::ENV_ID_LINUX,
                "MmapReader is only supported on Linux x64"
            );
        }

        use crate::platform::os::linux::syscall;
        let mut st = syscall::Stat::default();
        unsafe {
            syscall::fstat(0, &mut st);
            let page_boundary = st.st_size as usize & 0xfff;
            let file_size = ((st.st_size as usize + 0xfff) >> 12) << 12;
            let mut extra_page = 0;
            if page_boundary > 0xff8 {
                // Ensure we have at least 8 bytes at the end of the buffer.
                // If (stdin file size mod 4096) > 4088, we allocate one extra page at the end of it.
                extra_page = 0x1000;
            }
            // We first reserve the required address space.
            let buf = syscall::mmap(
                core::ptr::null(),
                file_size + extra_page,
                syscall::PROT_NONE,
                syscall::MAP_ANON | syscall::MAP_PRIVATE,
                -1,
                0,
            );
            // Then, we allocate the memory for stdin, ...
            syscall::mmap(
                buf,
                st.st_size as usize,
                syscall::PROT_READ,
                syscall::MAP_SHARED | syscall::MAP_FIXED,
                0,
                0,
            );
            if extra_page > 0 {
                // ...and the extra page at the end if needed.
                syscall::mmap(
                    buf.wrapping_add(file_size),
                    extra_page,
                    syscall::PROT_WRITE | syscall::PROT_READ,
                    syscall::MAP_ANON | syscall::MAP_PRIVATE | syscall::MAP_FIXED,
                    -1,
                    0,
                );
            }
            Self {
                buf,
                end: buf.wrapping_add(st.st_size as usize),
            }
        }
    }
}

#[cfg(all(target_arch = "x86_64", not(test)))]
impl ReaderBufferTrait for MmapReader {
    fn try_refill_internal(&mut self, _readahead: usize) -> usize {
        unsafe { self.end.offset_from_unsigned(self.buf) }
    }
    fn remain_internal(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.buf, self.end.offset_from_unsigned(self.buf)) }
    }
    fn advance(&mut self, bytes: usize) {
        self.buf = self.buf.wrapping_add(bytes);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    pub struct MockReader {
        buf: alloc::vec::Vec<u8>,
        off: usize,
    }

    impl Default for MockReader {
        fn default() -> Self {
            Self::new(&[])
        }
    }

    impl MockReader {
        fn new(data: &[u8]) -> Self {
            let mut buf = data.to_vec();
            buf.extend_from_slice(&[0u8; 8]); // ensure padding for unsafe I/O acceleration
            Self { buf, off: 0 }
        }
    }

    impl ReaderBufferTrait for MockReader {
        fn try_refill_internal(&mut self, _readahead: usize) -> usize {
            self.buf.len() - 8 - self.off
        }
        fn remain_internal(&self) -> &[u8] {
            &self.buf[self.off..self.buf.len() - 8]
        }
        fn advance(&mut self, bytes: usize) {
            self.off += bytes;
            assert!(self.off <= self.buf.len() - 8);
        }
    }

    #[test]
    fn read_numbers() {
        let mut reader = MockReader::new(
            b"1234 -56 1234567890 -9223372036854775808 18446744073709551615\n-9999.9999\n",
        );

        assert_eq!(reader.usize(), 1234);
        assert_eq!(reader.i32(), -56);
        assert_eq!(reader.i64(), 1234567890);
        assert_eq!(reader.i64(), -9223372036854775808);
        assert_eq!(reader.u64(), 18446744073709551615);
        assert_eq!(reader.f64(), -9999.9999);
    }

    #[test]
    fn read_scientifi_notation() {
        let mut reader = MockReader::new(b"1e1\n1e-1\n");

        assert_eq!(reader.f64(), 10.0);
        assert_eq!(reader.f64(), 1e-1);
    }

    #[test]
    fn read_word() {
        let mut reader = MockReader::new(b"Hello World\r\nBye\n");
        let mut buf = [0; 100];

        let n = reader.word_buf(&mut buf);
        assert_eq!(n, 5);
        assert_eq!(&buf[..n], b"Hello");

        let n = reader.word_buf(&mut buf);
        assert_eq!(n, 5);
        assert_eq!(&buf[..n], b"World");

        let n = reader.word_buf(&mut buf);
        assert_eq!(n, 3);
        assert_eq!(&buf[..n], b"Bye");
    }

    #[test]
    fn next_until() {
        let mut reader = MockReader::new(b"Hello World\r\nBye\n");
        let mut buf = [0; 100];

        let n = reader.line_buf(&mut buf);
        assert_eq!(n, 11);
        assert_eq!(&buf[..n], b"Hello World");

        let n = reader.word_buf(&mut buf);
        assert_eq!(n, 3);
        assert_eq!(&buf[..n], b"Bye");
    }

    #[test]
    fn read_word_without_terminator() {
        let mut reader = MockReader::new(b"no-terminator");
        let mut buf = [0; 100];

        let n = reader.word_buf(&mut buf);
        assert_eq!(n, 13);
        assert_eq!(&buf[..n], b"no-terminator");
    }

    #[test]
    fn read_word_multiple_space_in_between() {
        // This also affects number reading.
        let mut reader = MockReader::new(b"1 \nb"); // Trailing space in first line
        let mut buf = [0; 100];

        let n = reader.word_buf(&mut buf);
        assert_eq!(n, 1);

        let n = reader.word_buf(&mut buf);
        assert_eq!(n, 1);
        assert_eq!(&buf[..n], b"b");
    }

    #[test]
    fn skip_white() {
        let mut reader = MockReader::new(b" \t\x0b\n5\n");
        assert_eq!(reader.skip_whitespace(), 4);
        assert_eq!(reader.usize(), 5);
    }

    #[test]
    fn skip_until() {
        let mut reader = MockReader::new(b"garbage,5\n");
        assert_eq!(reader.discard(b','), b"garbage".len());
        assert_eq!(reader.usize(), 5);
    }
}
