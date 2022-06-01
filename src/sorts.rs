use core::{cmp::Ordering, mem::MaybeUninit};

#[inline(always)]
pub fn sort_insertion<T: Ord>(arr: &mut [T]) {
    sort_insertion_by(arr, T::cmp);
}

#[inline(always)]
pub fn sort_insertion_by_key<T, F: FnMut(&T) -> K, K: Ord>(arr: &mut [T], mut key_f: F) {
    sort_insertion_by(arr, |a, b| key_f(a).cmp(&key_f(b)));
}

pub fn sort_insertion_by<T, F: FnMut(&T, &T) -> Ordering>(arr: &mut [T], mut by: F) {
    for mut i in 1..arr.len() {
        while i > 0 && matches!(by(&arr[i], &arr[i - 1]), Ordering::Less) {
            arr.swap(i, i - 1);
            i -= 1;
        }
    }
}

pub trait Binary: Copy + Ord {
    fn bits() -> u32;
    fn get_byte_at(&self, pos: u32) -> u8;
}

#[inline(always)]
pub fn sort_radix<T: Binary>(arr: &mut [T]) {
    sort_radix_by_key(arr, |&x| x);
}

#[inline(always)]
pub fn sort_radix_by_key<T, F: (Fn(&T) -> K) + Copy, K: Binary>(arr: &mut [T], key_f: F) {
    sort_radix_by_key_rec(arr, key_f, K::bits() - 8);
}

// reference: https://github.com/voutcn/kxsort/blob/master/kxsort.h

fn sort_radix_by_key_rec<T, F: (Fn(&T) -> K) + Copy, K: Binary>(
    arr: &mut [T],
    key_f: F,
    pos: u32,
) {
    let mut count: [u32; 256] = unsafe { MaybeUninit::uninit().assume_init() };
    count.iter_mut().for_each(|v| *v = 0);
    for v in arr.iter() {
        count[key_f(v).get_byte_at(pos) as usize] += 1;
    }
    let mut last: [u32; 257] = unsafe { MaybeUninit::uninit().assume_init() };
    last[0] = 0;
    last[1] = 0;
    for i in 2..=256 {
        last[i] = last[i - 1] + count[i - 2];
    }
    for i in 0..256 {
        let end = last[i] + count[i];
        if end == arr.len() as u32 {
            last[i + 1] = end;
            break;
        }
        while last[i + 1] < end {
            let byte = key_f(&arr[last[i + 1] as usize]).get_byte_at(pos) as usize;
            if byte == i {
                last[i + 1] += 1;
            } else {
                arr.swap(last[i + 1] as usize, last[byte + 1] as usize);
                last[byte + 1] += 1;
            }
        }
    }
    if pos > 0 {
        for i in 0..256 {
            let block = &mut arr[last[i] as usize..last[i + 1] as usize];
            if count[i] > 64 {
                sort_radix_by_key_rec(block, key_f, pos - 8);
            } else if count[i] > 1 {
                sort_insertion_by_key(block, key_f);
            }
        }
    }
}

impl Binary for u16 {
    fn bits() -> u32 {
        u16::BITS
    }
    fn get_byte_at(&self, pos: u32) -> u8 {
        (self >> pos) as u8
    }
}

impl Binary for u32 {
    fn bits() -> u32 {
        u32::BITS
    }
    fn get_byte_at(&self, pos: u32) -> u8 {
        (self >> pos) as u8
    }
}

impl Binary for u64 {
    fn bits() -> u32 {
        u64::BITS
    }
    fn get_byte_at(&self, pos: u32) -> u8 {
        (self >> pos) as u8
    }
}

impl Binary for usize {
    fn bits() -> u32 {
        usize::BITS
    }
    fn get_byte_at(&self, pos: u32) -> u8 {
        (self >> pos) as u8
    }
}
