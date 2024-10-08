s = input()
s = list(s)

c = []
l = []
c_num = 1

for i in range(1,len(s)):
    if s[i-1] == s[i]:
        c_num = c_num + 1
    else:
        c.append(c_num)
        l.append(s[i-1])
        c_num = 1

c.append(c_num)
l.append(s[len(s)-1])

for i in range(len(c)):
    print(l[i], c[i], end=' ')

print("")