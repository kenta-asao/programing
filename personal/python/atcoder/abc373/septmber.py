s = []
count = 0

for i in range(12):
    s.append(input())

for i in range(12):
    if len(s[i]) == (i+1):
        count = count + 1

print(count)