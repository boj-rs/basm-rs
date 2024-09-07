/*
i686_elf.rs: Handles dynamic relocations at runtime in ELF32 (i686) binaries
Copyright (C) 2008 Seunghun Han (kkamagui)
Copyright (C) 2023 Byeongkeun Ahn (byeongkeunahn)

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 2 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

========

The ELF parsing and relocation routines in basm-rs were adapted
from the following implementation of MINT64OS, licensed under GPLv2+:
    https://github.com/kkamagui/mint64os/blob/master/02.Kernel64/Source/Loader.c

The original license statement:
    /**
     *  file    ApplicationLoader.c
     *  date    2009/12/26
     *  author  kkamagui
     *          Copyright(c)2008 All rights reserved by kkamagui
     *  brief   응용프로그램을 로드하여 실행하는 로더(Loader)에 관련된 함수를 정의한 소스 파일
     */
(brief in English:
    source file defining functions for loader that loads and runs applications)

Unlike all other parts of basm-rs, which are under the MIT license,
the files implementing ELF parsing and relocation are exceptionally
licensed under GPLv2+ since it is derived from an existing GPLv2+
implementation, "Loader.c" (see above). Although GPLv2+ mandates
licensing the project in its entirety as GPLv2+, the original author
has kindly granted us permission to confine the GPLv2+ license to
the parts explicitly derived from "Loader.c".

There are currently three files licensed under GPLv2+:
    scripts/static-pie-elf2bin.py
    basm-std/src/platform/loader/amd64_elf.rs
    basm-std/src/platform/loader/i686_elf.rs     (current file)
*/

#![allow(clippy::cmp_null)]

// Dynamic section entry types
const DT_REL: u32 = 17;
const DT_RELSZ: u32 = 18;
const DT_RELENT: u32 = 19;

// Relocation types
const R_386_NONE: u8 = 0; // none
const R_386_RELATIVE: u8 = 8; // word64   B + A

// ELF structs
#[repr(packed)]
struct Elf32Dyn {
    d_tag: u32,
    d_val_or_ptr: u32,
}
#[repr(packed)]
struct Elf32Rel {
    r_offset: u32,
    r_info: u32,
}

unsafe fn find_tag(mut ptr: *const Elf32Dyn, tag: u32) -> *const Elf32Dyn {
    unsafe {
        while (*ptr).d_tag != 0 {
            if (*ptr).d_tag == tag {
                return ptr;
            }
            ptr = ptr.add(1);
        }
        core::ptr::null()
    }
}

pub unsafe extern "C" fn relocate(addr_image_base: u32, addr_dynamic_section: u32) {
    unsafe {
        let ptr_dyn = addr_dynamic_section as *const Elf32Dyn;
        let ptr_rel = find_tag(ptr_dyn, DT_REL);
        let ptr_relsz = find_tag(ptr_dyn, DT_RELSZ);
        let ptr_relent = find_tag(ptr_dyn, DT_RELENT);

        /* do not use .is_null() since the method itself requires relocations, at least in debug mode */
        if ptr_rel == core::ptr::null()
            || ptr_relsz == core::ptr::null()
            || ptr_relent == core::ptr::null()
        {
            return;
        }

        let mut j = 0;
        while j < (*ptr_relsz).d_val_or_ptr {
            let pst_rel = (addr_image_base + (*ptr_rel).d_val_or_ptr + j) as *mut Elf32Rel;
            let ul_offset = (*pst_rel).r_offset;
            let ul_info = (*pst_rel).r_info;
            if ul_info as u8 == R_386_RELATIVE {
                let ptr_target = (addr_image_base + ul_offset) as *mut u32;
                *ptr_target += addr_image_base;
            } else if ul_info as u8 == R_386_NONE {
                /* do nothing */
            } else {
                /* not implemented */
                panic!();
            }
            j += (*ptr_relent).d_val_or_ptr;
        }
    }
}
