n = int(input())
a_input = []
a = [0,0,0,0,0,0,0,0,0,0]
sum = 0

a_input = [int(x) for x in input().split()]
for j in range(n):
    match a_input:
        case a_input if a_input[j] == 1:
            a[0] = a[0] + 1
        case a_input if a_input[j] == 2:
            a[1] = a[1] + 1
        case a_input if a_input[j] == 3:
            a[2] = a[2] + 1
        case a_input if a_input[j] == 4:
            a[3] = a[3] + 1
        case a_input if a_input[j] == 5:
            a[4] = a[4] + 1
        case a_input if a_input[j] == 6:
            a[5] = a[5] + 1
        case a_input if a_input[j] == 7:
            a[6] = a[6] + 1
        case a_input if a_input[j] == 8:
            a[7] = a[7] + 1
        case a_input if a_input[j] == 9:
            a[8] = a[8] + 1
        case a_input if a_input[j] == 10:
            a[9] = a[9] + 1

p = [int(x) for x in input().split()]

for k in range(10):
    sum = sum + a[k]*p[k]

print(sum)