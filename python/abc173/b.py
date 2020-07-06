n = int(input())

ac = 0
wa = 0
tle = 0
re = 0

for _ in range(n):
    i = input()

    if i == "AC":
        ac += 1
    if i == "WA":
        wa += 1
    if i == "TLE":
        tle += 1
    if i == "RE":
        re += 1

print("AC x {}".format(ac))
print("WA x {}".format(wa))
print("TLE x {}".format(tle))
print("RE x {}".format(re))
