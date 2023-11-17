def encode(x):
    out = []
    i = 0
    cnt5, stack5 = 0, []
    while i < len(x):
        out.append(x[i])
        i += 1
        cnt5 += 5
        stack5.append((len(out) - 1, 8))
        while cnt5 >= 8:
            v = x[i] if i < len(x) else 0
            i += 1
            bit_rem = 8
            while bit_rem > 0:
                pos, bits = stack5.pop()
                drain = min(bit_rem, 13 - bits)
                out[pos] |= (v & ((1 << drain) - 1)) << bits
                v >>= drain
                bit_rem -= drain
                bits += drain
                if bits < 13:
                    stack5.append((pos, bits))
            cnt5 -= 8
    ret = bytearray(2 * len(out) + 1)
    for i in range(len(out)):
        ret[2 * i + 0] = 0x24 + (out[i] % 91)
        ret[2 * i + 1] = 0x24 + (out[i] // 91)
    ret[-1] = ord('!')
    return bytes(ret)

if __name__ == '__main__':
    input_data = b"ASFGUT"
    ans = b"C*l5I*6]!"
    enc = encode(input_data)
    print("Expected: {0}".format(ans))
    print("Actual:   {0} ({1})".format(enc, "Agrees" if enc == ans else "Disagrees"))