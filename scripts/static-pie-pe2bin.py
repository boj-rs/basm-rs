import json
import sys
try:
    import pefile
except ModuleNotFoundError as e:
    print("basm-rs: \033[91mFailed\033[0m to load the required dependency 'pefile'. " + \
        "Please \033[92mrun\033[0m the following command to install it:\n" + \
        "  \033[93mpip install pefile\n\033[0m", file=sys.stderr)
    raise e

if __name__ == '__main__':
    try:
        pe_path, binary_path = sys.argv[1:]
    except ValueError:
        print(f"Usage: {sys.argv[0]} pe_path binary_path", file=sys.stderr)
        sys.exit(1)

    pe = pefile.PE(pe_path)
    memory_bin = bytearray(pe.get_memory_mapped_image())
    needed = bytearray(len(memory_bin))
    pos_begin = len(memory_bin)
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
            pos_begin = min(pos_begin, va)
            pos_end = max(pos_end, va+sz)
    if pos_begin == len(memory_bin):
        pos_begin = 0
    for i in range(len(memory_bin)):
        if needed[i] == 0:
            memory_bin[i] = 0
    memory_bin = memory_bin[pos_begin:pos_end]
    if reloc_sz > 0:
        reloc_off = len(memory_bin)
        memory_bin += reloc_bin
    with open(binary_path, "wb") as f:
        f.write(bytes(memory_bin))

    fdict = {}
    fdict['leading_unused_bytes'] = pos_begin
    fdict['entrypoint_offset'] = pe.OPTIONAL_HEADER.AddressOfEntryPoint - pos_begin
    fdict['pe_image_base'] = pe.OPTIONAL_HEADER.ImageBase
    fdict['pe_off_reloc'] = 0 if reloc_sz == 0 else pos_begin + reloc_off
    fdict['pe_size_reloc'] = reloc_sz
    print(json.dumps(fdict))    # callers of this script can capture stdout to get this value