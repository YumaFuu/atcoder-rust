n = int(input())

a, b = 1, 2
for i in range(n-1):
    a, b = a + b, a

print(a)