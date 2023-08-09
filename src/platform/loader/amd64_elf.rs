// Code adapted from:
//     https://github.com/kkamagui/mint64os/blob/master/02.Kernel64/Source/Loader.c

// Dynamic section entry types
const DT_RELA:      u64 = 7;
const DT_RELASZ:    u64 = 8;
const DT_RELAENT:   u64 = 9;

// Relocation types
const R_X86_64_NONE:        u32 = 0;    // none
const R_X86_64_RELATIVE:    u32 = 8;    // word64   B + A

// ELF structs
#[repr(packed)]
struct Elf64Dyn {
    d_tag:          u64,
    d_val_or_ptr:   u64,
}
#[repr(packed)]
struct Elf64Rela {
    r_offset:       u64,
    r_info:         u64,
    r_addend:       u64,
}


unsafe fn find_tag(mut ptr: *const Elf64Dyn, tag: u64) -> *const Elf64Dyn {
    while (*ptr).d_tag != 0 {
        if (*ptr).d_tag == tag {
            return ptr;
        }
        ptr = ptr.add(1);
    }
    core::ptr::null()
}

pub unsafe extern "sysv64" fn relocate(
    addr_image_base: u64,
    addr_dynamic_section: u64
    ) {
    let ptr_dyn: *const Elf64Dyn = core::mem::transmute(addr_dynamic_section);
    let ptr_rela = find_tag(ptr_dyn, DT_RELA);
    let ptr_relasz = find_tag(ptr_dyn, DT_RELASZ);
    let ptr_relaent = find_tag(ptr_dyn, DT_RELAENT);

    if ptr_rela == core::ptr::null() ||
        ptr_relasz == core::ptr::null() ||
        ptr_relaent == core::ptr::null() {
        return;
    }

    let mut j = 0;
    while j < (*ptr_relasz).d_val_or_ptr {
        let pst_rela: *mut Elf64Rela = core::mem::transmute(
            addr_image_base + (*ptr_rela).d_val_or_ptr + j);
        let ul_offset = (*pst_rela).r_offset;
        let ul_info = (*pst_rela).r_info;
        let l_addend = (*pst_rela).r_addend;
        if ul_info as u32 == R_X86_64_RELATIVE {
            let l_result: u64 = addr_image_base + l_addend;
            let ptr_target: *mut u64 = core::mem::transmute(
                addr_image_base + ul_offset);
            *ptr_target = l_result;
        } else if ul_info as u32 == R_X86_64_NONE {
            /* do nothing */
        } else {
            /* not implemented */
            loop {}
        }
        j += (*ptr_relaent).d_val_or_ptr;
    }
}