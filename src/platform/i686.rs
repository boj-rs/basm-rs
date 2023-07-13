// Dynamic section entry types
const DT_REL:       u32 = 17;
const DT_RELSZ:     u32 = 18;
const DT_RELENT:    u32 = 19;

// Relocation types
const R_386_NONE:           u8 = 0;    // none
const R_386_RELATIVE:       u8 = 8;    // word64   B + A

// ELF structs
#[repr(packed)]
struct Elf32Dyn {
    d_tag:          u32,
    d_val_or_ptr:   u32,
}
#[repr(packed)]
struct Elf32Rel {
    r_offset:       u32,
    r_info:         u32,
}


unsafe fn find_tag(mut ptr: *const Elf32Dyn, tag: u32) -> *const Elf32Dyn {
    while (*ptr).d_tag != 0 {
        if (*ptr).d_tag == tag {
            return ptr;
        }
        ptr = ptr.add(1);
    }
    core::ptr::null()
}

pub unsafe extern "C" fn relocate(
    addr_image_base: u32,
    addr_dynamic_section: u32
    ) {
    let ptr_dyn: *const Elf32Dyn = core::mem::transmute(addr_dynamic_section);
    let ptr_rel = find_tag(ptr_dyn, DT_REL);
    let ptr_relsz = find_tag(ptr_dyn, DT_RELSZ);
    let ptr_relent = find_tag(ptr_dyn, DT_RELENT);

    if ptr_rel == core::ptr::null() ||
        ptr_relsz == core::ptr::null() ||
        ptr_relent == core::ptr::null() {
        return;
    }

    let mut j = 0;
    while j < (*ptr_relsz).d_val_or_ptr {
        let pst_rel: *mut Elf32Rel = core::mem::transmute(
            addr_image_base + (*ptr_rel).d_val_or_ptr + j);
        let ul_offset = (*pst_rel).r_offset;
        let ul_info = (*pst_rel).r_info;
        if ul_info as u8 == R_386_RELATIVE {
            let ptr_target: *mut u32 = core::mem::transmute(
                addr_image_base + ul_offset);
            *ptr_target += addr_image_base;
        } else if ul_info as u8 == R_386_NONE {
            /* do nothing */
        } else {
            /* not implemented */
            loop {}
        }
        j += (*ptr_relent).d_val_or_ptr;
    }
}