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
    memory_bin = pe.get_memory_mapped_image()
    with open(binary_path, "wb") as f:
        f.write(bytes(memory_bin))

    reloc = pe.OPTIONAL_HEADER.DATA_DIRECTORY[pefile.DIRECTORY_ENTRY['IMAGE_DIRECTORY_ENTRY_BASERELOC']]

    fdict = {}
    fdict['entrypoint_offset'] = pe.OPTIONAL_HEADER.AddressOfEntryPoint
    fdict['pe_image_base'] = pe.OPTIONAL_HEADER.ImageBase
    fdict['pe_off_reloc'] = reloc.VirtualAddress
    fdict['pe_size_reloc'] = reloc.Size
    print(json.dumps(fdict))    # callers of this script can capture stdout to get this value