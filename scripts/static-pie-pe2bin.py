import json
import pefile
import sys

if __name__ == '__main__':
    try:
        pe_path, binary_path = sys.argv[1:]
    except ValueError:
        print(f"Usage: {sys.argv[0]} pe_path binary_path", file=sys.stderr)
        sys.exit(1)

    pe = pefile.PE(pe_path)
    memory_bin = bytearray(pe.get_memory_mapped_image())
    needed = bytearray(len(memory_bin))
    pos_end = 0
    reloc_bin, reloc_off, reloc_sz = bytearray(), 0, 0
    for section in pe.sections:
        va, sz = section.VirtualAddress, section.Misc_VirtualSize
        section_name = section.Name.rstrip(b'\x00').decode()
        if section_name == '.pdata':
            continue
        elif section_name == '.reloc':
            reloc_bin = memory_bin[va:va+sz]
            reloc_sz = sz
        else:
            for i in range(va, va+sz):
                needed[i] = 1
            pos_end = max(pos_end, va+sz)
    for i in range(len(memory_bin)):
        if needed[i] == 0:
            memory_bin[i] = 0
    memory_bin = memory_bin[:pos_end]
    if reloc_sz > 0:
        reloc_off = len(memory_bin)
        memory_bin += reloc_bin
    with open(binary_path, "wb") as f:
        f.write(bytes(memory_bin))

    fdict = {}
    fdict['entrypoint_offset'] = pe.OPTIONAL_HEADER.AddressOfEntryPoint
    fdict['pe_image_base'] = pe.OPTIONAL_HEADER.ImageBase
    fdict['pe_off_reloc'] = reloc_off
    fdict['pe_size_reloc'] = reloc_sz
    print(json.dumps(fdict))    # callers of this script can capture stdout to get this value