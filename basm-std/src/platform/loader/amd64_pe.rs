use core::ptr;

// Relocation types
const IMAGE_REL_BASED_ABSOLUTE: u16 = 0; // The base relocation is skipped. This type can be used to pad a block.
const IMAGE_REL_BASED_HIGH: u16 = 1; // The base relocation adds the high 16 bits of the difference to the 16-bit field at offset. The 16-bit field represents the high value of a 32-bit word.
const IMAGE_REL_BASED_LOW: u16 = 2; // The base relocation adds the low 16 bits of the difference to the 16-bit field at offset. The 16-bit field represents the low half of a 32-bit word.
const IMAGE_REL_BASED_HIGHLOW: u16 = 3; // The base relocation applies all 32 bits of the difference to the 32-bit field at offset.
const IMAGE_REL_BASED_HIGHADJ: u16 = 4; // The base relocation adds the high 16 bits of the difference to the 16-bit field at offset. The 16-bit field represents the high value of a 32-bit word. The low 16 bits of the 32-bit value are stored in the 16-bit word that follows this base relocation. This means that this base relocation occupies two slots.
const IMAGE_REL_BASED_DIR64: u16 = 10; // The base relocation applies the difference to the 64-bit field at offset.

// PE structs
//#[repr(packed)]
//struct ImageBaseRelocation {
//    virtual_address:    u32,
//    size_of_block:      u32,
//    type_offset:        u16,
//}

/* This function assumes the original ImageBase is 0x0,
 *   which is ensured by `static-pie-pe2bin.py`.
 * Note that when the executable runs natively,
 *   this assumption breaks; but in that case,
 *   the Windows PE loader handles relocation for us,
 *   and thus this function is not run; hence no problem.
 */
pub unsafe extern "sysv64" fn relocate(addr_image_base: u64, off_reloc: u64, size_reloc: u64) {
    let mut off = addr_image_base + off_reloc;
    let end = off + size_reloc;
    while off < end {
        let virtual_address: u32 = ptr::read(off as *const u32);
        let size_of_block: u32 = ptr::read((off + 4) as *const u32);
        let end_of_block: u64 = off + size_of_block as u64;
        off += 8;
        let reloc_delta: u64 = addr_image_base;
        while off < end_of_block {
            let w_val: u16 = ptr::read(off as *const u16);
            off += 2;
            let w_type: u16 = (w_val & 0xF000) >> 12;
            let w_offset: u16 = w_val & 0x0FFF;
            let patch_addr: u64 = addr_image_base + virtual_address as u64 + w_offset as u64;
            let mut tmp: i32;
            match w_type {
                IMAGE_REL_BASED_HIGH => {
                    ptr::write(
                        patch_addr as *mut u16,
                        ptr::read(patch_addr as *const u16) + (reloc_delta >> 16) as u16,
                    );
                }
                IMAGE_REL_BASED_LOW => {
                    ptr::write(
                        patch_addr as *mut u16,
                        ptr::read(patch_addr as *const u16) + reloc_delta as u16,
                    );
                }
                IMAGE_REL_BASED_HIGHLOW => {
                    ptr::write(
                        patch_addr as *mut u32,
                        ptr::read(patch_addr as *const u32) + reloc_delta as u32,
                    );
                }
                IMAGE_REL_BASED_HIGHADJ => {
                    if off + 2 >= end {
                        unreachable!()
                    }
                    tmp = (ptr::read(patch_addr as *const u16) as i32) << 16;
                    tmp += ptr::read(off as *const u16) as i32;
                    off += 2;
                    tmp += reloc_delta as i32;
                    tmp += 0x8000;
                    ptr::write(patch_addr as *mut u16, (tmp >> 16) as u16);
                }
                IMAGE_REL_BASED_DIR64 => {
                    ptr::write(
                        patch_addr as *mut u64,
                        ptr::read(patch_addr as *const u64) + reloc_delta,
                    );
                }
                IMAGE_REL_BASED_ABSOLUTE => (),
                _ => {
                    unreachable!()
                }
            }
        }
    }
}
