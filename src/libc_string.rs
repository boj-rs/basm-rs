use core::cmp::Ordering;

#[no_mangle]
unsafe extern "C" fn memcpy(dest: *mut u8, mut src: *const u8, n: usize) -> *mut u8 {
    let mut p = dest;
    for _ in 0..n {
        *p = *src;
        p = p.offset(1);
        src = src.offset(1);
    }
    dest
}

#[no_mangle]
unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if src == dest || n == 0 {
        return dest;
    }

    if (dest as *const u8) > src && (dest as *const u8) < src.add(n) {
        // [src ........]
        //     [dest .......]
        for i in (0..n).rev() {
            *dest.add(i) = *src.add(i);
        }
    } else if (src > dest as *const u8) && src < dest.add(n) {
        //     [src ........]
        // [dest .......]
        for i in 0..n {
            *dest.add(i) = *src.add(i);
        }
    } else {
        memcpy(dest, src, n);
    }
    dest
}

#[no_mangle]
unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut p = s;
    for _ in 0..n {
        *p = c as u8;
        p = p.offset(1);
    }
    s
}

#[no_mangle]
unsafe extern "C" fn memcmp(mut s1: *const u8, mut s2: *const u8, n: usize) -> i32 {
    for _ in 0..n {
        match (*s1).cmp(&*s2) {
            Ordering::Less => return -1,
            Ordering::Greater => return 1,
            _ => {
                s1 = s1.offset(1);
                s2 = s2.offset(1);
            }
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn memmove_no_overlap() {
        let src = b"Hello";
        let mut dest = [b'!'; 7];
        unsafe {
            memmove(dest.as_mut_ptr().offset(1), src.as_ptr(), 5);
        }
        assert_eq!(&dest, b"!Hello!");
    }

    #[test]
    fn memmove_overlapping_area_src_comes_first() {
        let mut region = [b'H', b'e', b'l', b'l', b'o', 0, 0, b'!'];
        //            src ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        //                            ^^^^^^^^^^^^^^^^^^^^^^ dest
        unsafe {
            memmove(region.as_mut_ptr().offset(2), region.as_ptr(), 5);
        }
        assert_eq!(&region, b"HeHello!");
    }

    #[test]
    fn memmove_overlapping_area_dest_comes_first() {
        let mut region = [0, 0, b'H', b'e', b'l', b'l', b'o', b'!'];
        //                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ src
        //           dest ^^^^^^^^^^^^^^^^^^^^^^^
        unsafe {
            memmove(region.as_mut_ptr(), region.as_ptr().offset(2), 5);
        }
        assert_eq!(&region, b"Hellolo!");
    }
}
