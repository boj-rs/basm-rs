const R_AARCH64_RELATIVE: u32 = 1027;

const DT_RELA: u64 = 7;
const DT_RELASZ: u64 = 8;
const DT_RELAENT: u64 = 9;

#[repr(C, packed)]
struct Elf64Dyn {
    d_tag: u64,
    d_val_or_ptr: u64
}

#[repr(C, packed)]
struct Elf64Rela {
    r_offset: u64,
    r_info: u64,
    r_addend: u64
}

unsafe fn locate_dynamic_hdr(addr_dynamic_section: u64, d_tag: u64) -> *const Elf64Dyn {
    unsafe {
        let mut ptr = addr_dynamic_section as *const Elf64Dyn;
        loop {
            if (*ptr).d_tag == 0 {
                break core::ptr::null();
            }
            if (*ptr).d_tag == d_tag {
                break ptr;
            }
            ptr = ptr.add(1);
        }
    }
}

pub unsafe extern "C" fn relocate(addr_image_base: u64, addr_dynamic_section: u64) {
    unsafe {
        let dyn_ptr_rela = locate_dynamic_hdr(addr_dynamic_section, DT_RELA);
        let dyn_ptr_relasz = locate_dynamic_hdr(addr_dynamic_section, DT_RELASZ);
        let dyn_ptr_relaent = locate_dynamic_hdr(addr_dynamic_section, DT_RELAENT);

        if dyn_ptr_rela == core::ptr::null() ||
            dyn_ptr_relasz == core::ptr::null() ||
            dyn_ptr_relaent == core::ptr::null() {
            return;
        }

        let rela_base = (addr_image_base + (*dyn_ptr_rela).d_val_or_ptr) as usize;
        let relasz = (*dyn_ptr_relasz).d_val_or_ptr as usize;
        let relaent = (*dyn_ptr_relaent).d_val_or_ptr as usize;

        let mut rela_addr = rela_base;
        while rela_addr < rela_base + relasz {
            let rela_ptr = rela_addr as *const Elf64Rela;
            let r_info_type = (*rela_ptr).r_info as u32;
            if r_info_type == R_AARCH64_RELATIVE {
                let addr = (addr_image_base + (*rela_ptr).r_offset) as usize;
                let value = addr_image_base.wrapping_add((*rela_ptr).r_addend);
                core::ptr::write(addr as *mut u64, value);
            } else {
                // Unimplemented or unknown relocation type
                panic!();
            }
            rela_addr += relaent;
        }
    }
}