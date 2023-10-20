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
    if i == 0:
        out.append("        \".quad ")
    elif i % 32 == 0:
        out.append("        ")
    x = int.from_bytes(prestub[i:i+8], "little")
    qword1 = str(hex(x))
    qword2 = str(x)
    if len(qword1) <= len(qword2):
        out.append(qword1)
    else:
        out.append(qword2)
    if i + 8 == len(prestub):
        out.append("\",\n")
    elif i % 32 == 24:
        out.append(",\n")
    else:
        out.append(",")

# print the result
print("".join(out))