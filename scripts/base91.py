def encode(x_in, use_rle=False):
    sharp_insertion_points = []
    if use_rle:
        current_bits, current_bytes, i = 0, 0, 0
        x = bytearray()
        while i < len(x_in):
            current_bits += 13
            while current_bytes < current_bits // 8:
                if i >= len(x_in):
                    break
                x.append(x_in[i])
                current_bytes += 1
                i += 1
            if len(x) > 0 and x[-1] == 0:
                zeros_cnt = 1
                while i - 1 + zeros_cnt < len(x_in) and zeros_cnt < 256 and x_in[i - 1 + zeros_cnt] == 0:
                    zeros_cnt += 1
                if zeros_cnt >= 2:
                    x.pop()
                    x.append(zeros_cnt - 1)
                    sharp_insertion_points.append((current_bits // 13 * 2) + len(sharp_insertion_points))
                    i += zeros_cnt - 1
        sharp_insertion_points = list(reversed(sharp_insertion_points))
    else:
        x = x_in

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
    ret = bytearray()
    for pack in out:
        ret.append(0x24 + (pack % 91))
        ret.append(0x24 + (pack // 91))
        if len(sharp_insertion_points) > 0 and len(ret) == sharp_insertion_points[-1]:
            ret.append(ord(b'#'))
            sharp_insertion_points.pop()
    ret.append(ord(b'!'))
    return bytes(ret)

if __name__ == '__main__':
    input_data = b"ASFGUT"
    ans = b"C*l5I*6]!"
    enc = encode(input_data)
    print("Expected: {0}".format(ans))
    print("Actual:   {0} ({1})".format(enc, "Agrees" if enc == ans else "Disagrees"))