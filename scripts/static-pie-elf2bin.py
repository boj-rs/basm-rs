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
    scripts/static-pie-elf2bin.py                (current file)
    basm-std/src/platform/loader/amd64_elf.rs
    basm-std/src/platform/loader/i686_elf.rs
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

# shn
SHN_UNDEF       = 0

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
    e_shstrndx = b2i(elf[62:64])
    for i in range(e_shnum):
        sh_offset = e_shoff + i*e_shentsize
        pstSectionHeader = elf[sh_offset:sh_offset+e_shentsize]
        sh_dict = {
            'sh_name'   : b2i(pstSectionHeader[ 0: 4]),
            'sh_type'   : b2i(pstSectionHeader[ 4: 8]),
            'sh_flags'  : b2i(pstSectionHeader[ 8:16]),
            'sh_addr'   : b2i(pstSectionHeader[16:24]),
            'sh_offset' : b2i(pstSectionHeader[24:32]),
            'sh_size'   : b2i(pstSectionHeader[32:40]),
        }
        sh.append(sh_dict)

    shstrtab = b''
    if e_shstrndx != SHN_UNDEF:
        sh_dict = sh[e_shstrndx]
        src_off, cnt = sh_dict['sh_offset'], sh_dict['sh_size']
        shstrtab = bytes(elf[src_off:src_off+cnt])
    def resolve_sh_name(sh_name):
        if sh_name >= len(shstrtab):
            return b''
        i = sh_name
        while i < len(shstrtab) and shstrtab[i] != 0:
            i += 1
        return shstrtab[sh_name:i]

    pos_begin, pos_end = len(elf), 0
    for sh_dict in sh:
        if (sh_dict['sh_flags'] & SHF_ALLOC) != 0:
            pos_begin = min(pos_begin, sh_dict['sh_addr'])
            pos_end = max(pos_end, sh_dict['sh_addr'] + sh_dict['sh_size'])

    memory_bin = bytearray(pos_end)
    dynsym = []
    dynstr = b''
    for sh_dict in sh:
        dst_off, src_off, cnt = sh_dict['sh_addr'], sh_dict['sh_offset'], sh_dict['sh_size']
        blob = elf[src_off:src_off+cnt]

        if sh_dict['sh_type'] == SHT_DYNAMIC:
            # Trim the DYNAMIC section, leaving only relocation-related entries
            # 16 == sizeof(Elf64_Dyn)
            dst = 0 
            for src in range(0, len(blob), 16):
                # Included entries:
                #   DT_PLTRELSZ = 2, DT_RELA = 7, DT_RELASZ = 8, DT_RELAENT = 9,
                #   DT_REL = 17, DT_RELSZ = 18, DT_RELENT = 19, DT_PLTREL = 20,
                #   DT_TEXT_REL = 22, DT_JMPREL = 23.
                #
                # Note: DT_RELACOUNT = 0x6fff_fff9 and DT_RELCOUNT = 0x6fff_fffa
                #   are not included since they are redundant since
                #   DT_RELACOUNT = DT_RELASZ/DT_RELAENT and
                #   DT_RELCOUNT = DT_RELSZ/DT_RELENT.
                if b2i(blob[src:src+8]) in [2, 7, 8, 9, 17, 18, 19, 20, 22, 23]:
                    blob[dst:dst+16] = blob[src:src+16]
                    dst += 16
            blob[dst:] = bytearray(len(blob[dst:])) # fill remaining part with zeros
        elif sh_dict['sh_type'] == SHT_DYNSYM:
            for i in range(0, sh_dict['sh_size'], 24):
                st_entry = blob[i:][:24]
                st_dict = {
                    'st_name'   : b2i(st_entry[ 0: 4]),
                    'st_info'   : b2i(st_entry[ 4: 5]),
                    'st_other'  : b2i(st_entry[ 5: 6]),
                    'st_shndx'  : b2i(st_entry[ 6: 8]),
                    'st_value'  : b2i(st_entry[ 8:16]),
                    'st_size'   : b2i(st_entry[16:24]),
                }
                dynsym.append(st_dict)
        elif sh_dict['sh_type'] == SHT_STRTAB and resolve_sh_name(sh_dict['sh_name']) == b'.dynstr':
            dynstr = bytes(blob)

        if (sh_dict['sh_flags'] & SHF_ALLOC) == 0 or sh_dict['sh_size'] == 0:
            continue
        if sh_dict['sh_type'] == SHT_NOBITS:
            continue        # since bytearray is zero-initialized
        memory_bin[dst_off:dst_off+cnt] = blob

    def resolve_st_name(st_name):
        if st_name >= len(dynstr):
            return b''
        i = st_name
        while i < len(dynstr) and dynstr[i] != 0:
            i += 1
        return dynstr[st_name:i]

    exports = dict()
    for st_dict in dynsym:
        st_name_str = resolve_st_name(st_dict['st_name']).decode('utf8')
        if st_name_str.startswith("_basm_export_") or st_name_str.startswith("_basm_import_"):
            exports[st_name_str] = st_dict['st_value']

    entrypoint_offset = b2i(elf[24:32])
    return memory_bin, pos_begin, entrypoint_offset, exports

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
    exports = dict()        # TBD
    return memory_bin, pos_begin, entrypoint_offset, exports


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
        memory_bin, pos_begin, entrypoint_offset, exports = load_elf64(elf)
    elif elf[EI_CLASS] == ELFCLASS32:
        memory_bin, pos_begin, entrypoint_offset, exports = load_elf32(elf)
    else:
        print(f"Unsupported EI_CLASS value: {elf[EI_CLASS]}", file=sys.stderr)
        sys.exit(1)

    if pos_begin == len(elf):
        pos_begin = 0
    pos_begin -= pos_begin % 128
    assert entrypoint_offset >= pos_begin
    memory_bin = memory_bin[pos_begin:]
    entrypoint_offset -= pos_begin

    # Patch the entrypoint
    # We look for:
    #   0:  f8                      clc
    # and replace it with:
    #   0:  f9                      stc
    # This works for both i686 and amd64.
    assert memory_bin[entrypoint_offset:entrypoint_offset+1] == b"\xf8"
    memory_bin[entrypoint_offset:entrypoint_offset+1] = b"\xf9"

    # Write to file
    with open(binary_path, "wb") as f:
        f.write(bytes(memory_bin))

    # Process exports
    for k, v in exports.items():
        assert v >= pos_begin

    fdict = {}
    fdict['entrypoint_offset'] = entrypoint_offset
    fdict['exports'] = exports
    print(json.dumps(fdict))    # callers of this script can capture stdout to get this value