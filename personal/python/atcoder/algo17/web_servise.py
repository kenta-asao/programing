fee = [int(x) for x in input().split()]
month = [int(x) for x in input().split()]
a = 0
b = fee[1]
c = fee[2]

for i in range(12):
    if month[i] > 3:
        a = a + fee[0] * (month[i] - 3)

for i in range(12):
    if month[i] > 50:
        b = b + fee[0] * (month[i] - 50)

compare = [a, b, c]

print(min(compare))