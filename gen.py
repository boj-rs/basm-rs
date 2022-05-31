import os
import struct

text = "basm.bin"

code = bytearray((os.path.getsize(text) + 7) // 8 * 8)
with open(text, "rb") as f:
    f.readinto(code)

r = ",".join(min(str(i), hex(i), key=len) for i, in struct.iter_unpack("<q", code))
print('__attribute__((section(".text")))long long a[]={%s};__libc_start_main(){((int(*)())a)();}main;' % r)
