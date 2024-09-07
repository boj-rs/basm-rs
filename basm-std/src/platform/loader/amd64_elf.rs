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
    basm-std/src/platform/loader/amd64_elf.rs    (current file)
    basm-std/src/platform/loader/i686_elf.rs
*/

#![allow(clippy::cmp_null)]

use core::mem::MaybeUninit;

// Dynamic section entry types
const DT_RELA: u64 = 7;
const DT_RELASZ: u64 = 8;
const DT_RELAENT: u64 = 9;

// Relocation types
const R_X86_64_NONE: u32 = 0; // none
const R_X86_64_RELATIVE: u32 = 8; // word64   B + A

// ELF structs
#[repr(packed)]
struct Elf64Dyn {
    d_tag: u64,
    d_val_or_ptr: u64,
}
#[repr(packed)]
struct Elf64Rela {
    r_offset: u64,
    r_info: u64,
    r_addend: u64,
}

pub unsafe extern "sysv64" fn relocate(addr_image_base: u64, addr_dynamic_section: u64) {
    unsafe {
        let mut ptr_dyn: *const Elf64Dyn = addr_dynamic_section as *const Elf64Dyn;
        let mut ptr_rela = 0;
        let mut relasz = MaybeUninit::<u64>::uninit();
        let mut relaent = MaybeUninit::<u64>::uninit();
        loop {
            match (*ptr_dyn).d_tag {
                0 => {
                    break;
                }
                DT_RELA => {
                    ptr_rela = addr_image_base + (*ptr_dyn).d_val_or_ptr;
                }
                DT_RELASZ => {
                    relasz.write((*ptr_dyn).d_val_or_ptr);
                }
                DT_RELAENT => {
                    relaent.write((*ptr_dyn).d_val_or_ptr);
                }
                _ => (),
            }
            ptr_dyn = ptr_dyn.add(1);
        }

        /* 1) Do not use .is_null() since the method itself requires relocations, at least in debug mode.
         * 2) When DT_RELA is present, the other entries DT_RELASZ and DT_RELAENT must exist.
         *    Source: https://docs.oracle.com/cd/E19683-01/817-3677/chapter6-42444/index.html
         *    ("This element requires the DT_RELASZ and DT_RELAENT elements also be present.")
         */
        if ptr_rela == 0 {
            return;
        }
        relasz.write(relasz.assume_init() + ptr_rela);

        while ptr_rela < relasz.assume_init() {
            let pst_rela = ptr_rela as *mut Elf64Rela;
            let ul_offset = (*pst_rela).r_offset;
            let ul_info = (*pst_rela).r_info;
            let l_addend = (*pst_rela).r_addend;
            if ul_info as u32 == R_X86_64_RELATIVE {
                let l_result: u64 = addr_image_base + l_addend;
                let ptr_target = (addr_image_base + ul_offset) as *mut u64;
                *ptr_target = l_result;
            } else if ul_info as u32 == R_X86_64_NONE {
                /* do nothing */
            } else {
                /* not implemented */
                panic!();
            }
            ptr_rela += relaent.assume_init();
        }
    }
}
