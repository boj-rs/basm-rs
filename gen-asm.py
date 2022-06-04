import os
import array

text = "basm.bin"

code = bytearray((os.path.getsize(text) + 7) // 8 * 8)
with open(text, "rb") as f:
    f.readinto(code)

a = array.array('Q')
a.frombytes(code)
r = ",".join(min(str(i), hex(i), key=len) for i in a)
print('''section .text
    global main

main:
    dq %s''' % r)
