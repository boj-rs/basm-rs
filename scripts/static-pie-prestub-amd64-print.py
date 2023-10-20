# read prestub
with open("static-pie-prestub-amd64.bin", "rb") as f:
    prestub = f.read()

# pad to align at 8-byte boundary
prestub = bytearray(prestub)
while len(prestub) % 8 != 0:
    prestub.append(0)

# convert each 8-byte chunk
out = []
for i in range(0, len(prestub), 8):
    qword = str(hex(int.from_bytes(prestub[i:i+8], "little")))
    line = "        \".quad {0}\",\n".format(qword)
    out.append(line)

# print the result
print("".join(out))