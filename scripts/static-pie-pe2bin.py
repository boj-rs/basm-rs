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

    '''
    We relocate the PE image to the base address 0 (ImageBase=0),
      regardless of the original ImageBase (which is usually 0x1_4000_0000)
    This simplifies the loader code.
    '''
    pe = pefile.PE(pe_path)
    memory_bin = bytearray(pe.get_memory_mapped_image(ImageBase=0))
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
    entrypoint_offset = pe.OPTIONAL_HEADER.AddressOfEntryPoint - pos_begin
    if reloc_sz > 0:
        reloc_off = len(memory_bin)
        memory_bin += reloc_bin
    reloc_off = 0 if reloc_sz == 0 else pos_begin + reloc_off

    # Patch the entrypoint
    # We look for:
    #   0:  f8                      clc
    # and replace it with:
    #   0:  f9                      stc
    # This works for both i686 and amd64.
    assert memory_bin[entrypoint_offset:entrypoint_offset+1] == b"\xf8"
    memory_bin[entrypoint_offset:entrypoint_offset+1] = b"\xf9"

    # Patch the relocation offset and size (which is in _start)
    # We look for:
    #   0:  be 78 56 34 12          mov    esi,0x12345678  <- replaced with reloc_off
    #   5:  ba 78 56 34 12          mov    edx,0x12345678  <- replaced with reloc_sz
    template = b"\xbe\x78\x56\x34\x12\xba\x78\x56\x34\x12"
    reloc_patched = False
    for i in range(entrypoint_offset, len(memory_bin) - len(template)):
        if memory_bin[i:i+len(template)] == template:
            memory_bin[i+1:i+5] = reloc_off.to_bytes(4, byteorder='little')
            memory_bin[i+6:i+10] = reloc_sz.to_bytes(4, byteorder='little')
            reloc_patched = True
            break
    assert reloc_patched, "Failed to incorporate the relocation information into the binary. Please report this error."

    # Write to file
    with open(binary_path, "wb") as f:
        f.write(bytes(memory_bin))

    # Process exports
    exports = dict()
    for e in pe.DIRECTORY_ENTRY_EXPORT.symbols:
        e_name = None if e.name is None else e.name.decode('utf8')
        if e_name.startswith("_basm_export_"):
            assert e.address >= pos_begin
            exports[e_name] = e.address

    fdict = {}
    fdict['entrypoint_offset'] = entrypoint_offset
    fdict['exports'] = exports
    print(json.dumps(fdict))    # callers of this script can capture stdout to get this value