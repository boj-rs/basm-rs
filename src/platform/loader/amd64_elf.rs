/*
amd64_elf.rs: Handles dynamic relocations at runtime in ELF64 (amd64) binaries
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
    src/platform/loader/amd64_elf.rs    (current file)
    src/platform/loader/i686_elf.rs
*/

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