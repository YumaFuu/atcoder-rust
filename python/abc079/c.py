a, b, c, d = map(lambda x: int(x), list(input()))


def get_str(i):
    if i >= 0:
        s = "+{}".format(i)
    else:
        s = "{}".format(i)
    return s


for i in range(1 << 3):
    ai, bi, ci, di = a, b, c, d
    if (i >> 2) % 2:
        bi = -b
    if (i >> 1) % 2:
        ci = -c
    if (i >> 0) % 2:
        di = -d

    if ai + bi + ci + di == 7:
        s = ""
        s += get_str(ai)
        s += get_str(bi)
        s += get_str(ci)
        s += get_str(di)
        s += "=7"
        print(s[1::])
        break
