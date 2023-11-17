# read prestub
with open("static-pie-prestub-amd64.bin", "rb") as f:
    prestub = f.read()
prestub = bytearray(prestub)
if len(prestub) > 0 and prestub[-1] == 0:
    prestub = prestub[:-1]
    asciz = True
else:
    asciz = False

# special handling for trailing ASCII characters
j = len(prestub)
b85_table = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+-;<=>?@^_`{|}~"
while j > 0 and (prestub[j-1] in b85_table or prestub[j-1] == 0):
    j -= 1
while j < len(prestub) and j % 8 != 0:
    j += 1
table_part = prestub[j:]
prestub = prestub[:j]

# settings
SPECIFIER = ".quad"
CHUNK_SIZE = 8
ENTRIES_PER_LINE = 4

# pad to align at `CHUNK_SIZE`-byte boundary
while len(prestub) % CHUNK_SIZE != 0:
    prestub.append(0)

# convert each `CHUNK_SIZE`-byte chunk
out = []
for i in range(0, len(prestub), CHUNK_SIZE):
    if i == 0:
        out.append("        \"{0} ".format(SPECIFIER))
    elif i % (CHUNK_SIZE * ENTRIES_PER_LINE) == 0:
        out.append("        ")
    x = int.from_bytes(prestub[i:i+CHUNK_SIZE], "little")
    def to_hex_short(y):
        out = str(hex(y))[2:]
        nonzero_idx = len(out)
        while nonzero_idx > 1 and out[nonzero_idx-1] == '0':
            nonzero_idx -= 1
        out2 = out[:nonzero_idx] + "h<<" + str((len(out) - nonzero_idx) * 4)
        out = out + "h"
        if len(out2) < len(out):
            out = out2
        if ord(out[0]) >= ord('a'):
            out = "0" + out
        return out
    qword1 = to_hex_short(x)
    qword2 = str(x)
    if len(qword1) <= len(qword2):
        out.append(qword1)
    else:
        out.append(qword2)
    if i + CHUNK_SIZE == len(prestub):
        out.append("\",\n")
    elif i % (CHUNK_SIZE * ENTRIES_PER_LINE) == CHUNK_SIZE * (ENTRIES_PER_LINE - 1):
        out.append(",\\\n")
    else:
        out.append(",")

# convert the table part
table_part = table_part.decode('ascii')
table_part = table_part.replace('{', '{{').replace('}', '}}').replace('$', '\\\\x24').replace('\0','\\\\0')
out.append("        \"{0}\\\"{1}\\\"\",\n".format(".asciz" if asciz else ".ascii", table_part))

# print the result
print("".join(out))