#!/usr/bin/env python3
# from https://stackoverflow.com/questions/57664235/why-does-a-fully-static-rust-elf-binary-have-a-global-offset-table-got-section/57733804#57733804
# modified slightly to match basm-rs configuration

import re
import sys
import argparse
import subprocess

def read_u64(binary):
    return sum(binary[i] * 256 ** i for i in range(8))

def distance_u32(start, end):
    assert abs(end - start) < 2 ** 31
    diff = end - start
    if diff < 0:
        return 2 ** 32 + diff
    else:
        return diff

def to_u32(x):
    assert 0 <= x < 2 ** 32
    return bytes((x // (256 ** i)) % 256 for i in range(4))

class GotInstruction:
    def __init__(self, lines, symbol_address, symbol_offset):
        self.address = int(lines[0].split(":")[0].strip(), 16)
        self.offset = symbol_offset + (self.address - symbol_address)
        self.got_offset = int(lines[0].split("(File Offset: ")[1].strip().strip(")"), 16)
        self.got_offset = self.got_offset % 0x200000  # No idea why the offset is actually wrong
        self.bytes = []
        for line in lines:
            self.bytes += [int(x, 16) for x in line.split("\t")[1].split()]

class TextDump:
    symbol_regex = re.compile(r"^([0-9,a-f]{16}) <(.*)> \(File Offset: 0x([0-9,a-f]*)\):")

    def __init__(self, binary_path):
        self.got_instructions = []
        objdump_output = subprocess.check_output(["objdump", "-Fdj", ".text", "-j", ".init", "-M", "intel",
                                                  binary_path])
        lines = objdump_output.decode("utf-8").split("\n")
        current_symbol_address = 0
        current_symbol_offset = 0
        for line_group in self.group_lines(lines):
            match = self.symbol_regex.match(line_group[0])
            if match is not None:
                current_symbol_address = int(match.group(1), 16)
                current_symbol_offset = int(match.group(3), 16)
            elif ".got" in line_group[0]:
                instruction = GotInstruction(line_group, current_symbol_address,
                                             current_symbol_offset)
                self.got_instructions.append(instruction)

    @staticmethod
    def group_lines(lines):
        if not lines:
            return
        line_group = [lines[0]]
        for line in lines[1:]:
            if line.count("\t") == 1:  # this line continues the previous one
                line_group.append(line)
            else:
                yield line_group
                line_group = [line]
        yield line_group

    def __iter__(self):
        return iter(self.got_instructions)

def read_binary_file(path):
    try:
        with open(path, "rb") as f:
            return f.read()
    except (IOError, OSError) as exc:
        print(f"Failed to open {path}: {exc.strerror}")
        sys.exit(1)

def write_binary_file(path, content):
    try:
        with open(path, "wb") as f:
            f.write(content)
    except (IOError, OSError) as exc:
        print(f"Failed to open {path}: {exc.strerror}")
        sys.exit(1)

def patch_got_reference(instruction, binary_content):
    got_data = read_u64(binary_content[instruction.got_offset:])
    code = instruction.bytes
    if code[0] == 0xff:
        assert len(code) == 6
        relative_address = distance_u32(instruction.address, got_data)
        if code[1] == 0x15:  # call QWORD PTR [rip+...]
            patch = b"\xe8" + to_u32(relative_address - 5) + b"\x90"
        elif code[1] == 0x25:  # jmp QWORD PTR [rip+...]
            patch = b"\xe9" + to_u32(relative_address - 5) + b"\x90"
        else:
            raise ValueError(f"unknown machine code: {code}")
    elif code[:3] == [0x48, 0x83, 0x3d]:  # cmp QWORD PTR [rip+...],<BYTE>
        assert len(code) == 8
        if got_data == code[7]:
            patch = b"\x48\x39\xc0" + b"\x90" * 5  # cmp rax,rax
        elif got_data > code[7]:
            patch = b"\x48\x83\xfc\x00" + b"\x90" * 3  # cmp rsp,0x0
        else:
            patch = b"\x50\x31\xc0\x83\xf8\x01\x90"  # push rax
                                                     # xor eax,eax
                                                     # cmp eax,0x1
                                                     # pop rax
    elif code[:3] == [0x48, 0x3b, 0x1d]:  # cmp rbx,QWORD PTR [rip+...]
        assert len(code) == 7
        patch = b"\x81\xfb" + to_u32(got_data) + b"\x90"  # cmp ebx,<DWORD>
    else:
        raise ValueError(f"unknown machine code: {code}")
    return dict(offset=instruction.offset, data=patch)

def make_got_patches(binary_path, binary_content):
    patches = []
    text_dump = TextDump(binary_path)
    for instruction in text_dump.got_instructions:
        patches.append(patch_got_reference(instruction, binary_content))
    return patches

def apply_patches(binary_content, patches):
    for patch in patches:
        offset = patch["offset"]
        data = patch["data"]
        binary_content = binary_content[:offset] + data + binary_content[offset + len(data):]
    return binary_content

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("binary_path", help="Path to ELF binary")
    parser.add_argument("-o", "--output", help="Output file path", required=True)
    args = parser.parse_args()

    binary_content = read_binary_file(args.binary_path)
    patches = make_got_patches(args.binary_path, binary_content)
    patched_content = apply_patches(binary_content, patches)
    write_binary_file(args.output, patched_content)

if __name__ == "__main__":
    main()
