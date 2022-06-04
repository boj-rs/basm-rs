import os
import array

text = "basm.bin"

code = bytearray((os.path.getsize(text) + 7) // 8 * 8)
with open(text, "rb") as f:
    f.readinto(code)

arr = array.array('Q')
arr.frombytes(code)
r = ",".join(min(str(i), hex(i), key=len) for i in arr)
print('__attribute__((section(".text")))unsigned long long a[]={%s};__libc_start_main(){((int(*)())a)();}main;' % r)
