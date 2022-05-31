with open("basm.bin", "rb") as f:
    code = f.read()

n = len(code)
code += bytes([0]) * (4 - n % 4)

import array
s = array.array('i')
s.frombytes(code)
r = ",".join(map(str, s))
print(f"__attribute__((section(\".text\")))a[]={{{r}}};__libc_start_main(){{((int(*)())a)();}}main;")
