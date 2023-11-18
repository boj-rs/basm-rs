"""
static-pie-elf2bin.py: Python 3 script that converts an ELF binary into its in-memory layout.
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
    scripts/static-pie-elf2bin.py       (current file)
    src/platform/loader/amd64_elf.rs
    src/platform/loader/i686_elf.rs
"""

import json
import sys

# e_ident[]의 index 의미
EI_MAG0         = 0
EI_MAG1         = 1
EI_MAG2         = 2
EI_MAG3         = 3
EI_CLASS        = 4
EI_DATA         = 5
EI_VERSION      = 6
EI_OSABI        = 7
EI_ABIVERSION   = 8
EI_PAD          = 9
EI_NIDENT       = 16

# e_ident[EI_MAGX]
ELFMAG0         = 0x7F
ELFMAG1         = ord('E')
ELFMAG2         = ord('L')
ELFMAG3         = ord('F')

# e_ident[EI_CLASS]
ELFCLASSNONE    = 0
ELFCLASS32      = 1
ELFCLASS64      = 2

# e_ident[EI_DATA]
ELFDATANONE     = 0
ELFDATA2LSB     = 1
ELFDATA2MSB     = 2

# e_type
ET_NONE         = 0
ET_REL          = 1
ET_EXEC         = 2
ET_DYN          = 3
ET_CORE         = 4

# sh_type
SHT_NULL        = 0
SHT_PROGBITS    = 1
SHT_SYMTAB      = 2
SHT_STRTAB      = 3
SHT_RELA        = 4
SHT_HASH        = 5
SHT_DYNAMIC     = 6
SHT_NOTE        = 7
SHT_NOBITS      = 8
SHT_REL         = 9
SHT_SHLIB       = 10
SHT_DYNSYM      = 11
SHT_LOOS        = 0x60000000
SHT_HIOS        = 0x6FFFFFFF
SHT_LOPROC      = 0x70000000
SHT_HIPROC      = 0x7FFFFFFF
SHT_LOUSER      = 0x80000000
SHT_HIUSER      = 0xFFFFFFFF

# sh_flags
SHF_WRITE       = 1
SHF_ALLOC       = 2
SHF_EXECINSTR   = 4
SHF_MASKOS      = 0x0FF00000
SHF_MASKPROC    = 0xF0000000


def b2i(x):
    return int.from_bytes(x, byteorder='little')

def check_header(elf):
    if len(elf) < 18:
        return False
    return (elf[EI_MAG0] == ELFMAG0) and \
        (elf[EI_MAG1] == ELFMAG1) and \
        (elf[EI_MAG2] == ELFMAG2) and \
        (elf[EI_MAG3] == ELFMAG3) and \
        (elf[EI_CLASS] == ELFCLASS32 or elf[EI_CLASS] == ELFCLASS64) and \
        (elf[EI_DATA] == ELFDATA2LSB) and \
        (b2i(elf[16:18]) == ET_DYN)

def load_elf64(elf):
    sh = []

    e_shoff = b2i(elf[40:48])
    e_shentsize = b2i(elf[58:60])
    e_shnum = b2i(elf[60:62])
    for i in range(e_shnum):
        sh_offset = e_shoff + i*e_shentsize
        pstSectionHeader = elf[sh_offset:sh_offset+e_shentsize]
        sh_dict = {
            'sh_type'   : b2i(pstSectionHeader[ 4: 8]),
            'sh_flags'  : b2i(pstSectionHeader[ 8:16]),
            'sh_addr'   : b2i(pstSectionHeader[16:24]),
            'sh_offset' : b2i(pstSectionHeader[24:32]),
            'sh_size'   : b2i(pstSectionHeader[32:40]),
        }
        sh.append(sh_dict)

    pos_begin, pos_end = len(elf), 0
    for sh_dict in sh:
        if (sh_dict['sh_flags'] & SHF_ALLOC) != 0:
            pos_begin = min(pos_begin, sh_dict['sh_addr'])
            pos_end = max(pos_end, sh_dict['sh_addr'] + sh_dict['sh_size'])

    memory_bin = bytearray(pos_end)
    for sh_dict in sh:
        if (sh_dict['sh_flags'] & SHF_ALLOC) == 0 or sh_dict['sh_size'] == 0:
            continue
        if sh_dict['sh_type'] == SHT_NOBITS:
            continue        # since bytearray is zero-initialized

        dst_off, src_off, cnt = sh_dict['sh_addr'], sh_dict['sh_offset'], sh_dict['sh_size']
        memory_bin[dst_off:dst_off+cnt] = elf[src_off:src_off+cnt]

    entrypoint_offset = b2i(elf[24:32])
    return memory_bin, pos_begin, entrypoint_offset

def load_elf32(elf):
    sh = []

    e_shoff = b2i(elf[32:36])
    e_shentsize = b2i(elf[46:48])
    e_shnum = b2i(elf[48:50])
    for i in range(e_shnum):
        sh_offset = e_shoff + i*e_shentsize
        pstSectionHeader = elf[sh_offset:sh_offset+e_shentsize]
        sh_dict = {
            'sh_type'   : b2i(pstSectionHeader[ 4: 8]),
            'sh_flags'  : b2i(pstSectionHeader[ 8:12]),
            'sh_addr'   : b2i(pstSectionHeader[12:16]),
            'sh_offset' : b2i(pstSectionHeader[16:20]),
            'sh_size'   : b2i(pstSectionHeader[20:24]),
        }
        sh.append(sh_dict)

    pos_begin, pos_end = len(elf), 0
    for sh_dict in sh:
        if (sh_dict['sh_flags'] & SHF_ALLOC) != 0:
            pos_begin = min(pos_begin, sh_dict['sh_addr'])
            pos_end = max(pos_end, sh_dict['sh_addr'] + sh_dict['sh_size'])

    memory_bin = bytearray(pos_end)
    for sh_dict in sh:
        if (sh_dict['sh_flags'] & SHF_ALLOC) == 0 or sh_dict['sh_size'] == 0:
            continue
        if sh_dict['sh_type'] == SHT_NOBITS:
            continue        # since bytearray is zero-initialized

        dst_off, src_off, cnt = sh_dict['sh_addr'], sh_dict['sh_offset'], sh_dict['sh_size']
        memory_bin[dst_off:dst_off+cnt] = elf[src_off:src_off+cnt]

    entrypoint_offset = b2i(elf[24:28])
    return memory_bin, pos_begin, entrypoint_offset


if __name__ == '__main__':
    try:
        elf_path, binary_path = sys.argv[1:]
    except ValueError:
        print(f"Usage: {sys.argv[0]} elf_path binary_path", file=sys.stderr)
        sys.exit(1)

    with open(elf_path, "rb") as f:
        elf = bytearray(f.read())

    if not check_header(elf):
        print(f"Invalid ELF header", file=sys.stderr)
        sys.exit(1)

    if elf[EI_CLASS] == ELFCLASS64:
        memory_bin, pos_begin, entrypoint_offset = load_elf64(elf)
    elif elf[EI_CLASS] == ELFCLASS32:
        memory_bin, pos_begin, entrypoint_offset = load_elf32(elf)
    else:
        print(f"Unsupported EI_CLASS value: {elf[EI_CLASS]}", file=sys.stderr)
        sys.exit(1)

    if pos_begin == len(elf):
        pos_begin = 0
    pos_begin -= pos_begin % 128
    assert entrypoint_offset >= pos_begin
    memory_bin = memory_bin[pos_begin:]
    entrypoint_offset -= pos_begin

    with open(binary_path, "wb") as f:
        f.write(bytes(memory_bin))

    fdict = {}
    fdict['entrypoint_offset'] = entrypoint_offset
    print(json.dumps(fdict))    # callers of this script can capture stdout to get this value