h, w, k = map(int, input().split())

c = [0] * h
for i in range(h):
    c[i] = input()

for cols in range(1 << w):
    for rows in range(1 << h):
        print("cols: {} i: {}".format(format(cols, "b"), i))
        print(cols >> i)
        # for i in range(cols):

