N, C = (int(x) for x in input().split())
T = []
T = [int(x) for x in input().split()]

count = 1
time = T[0]


for i in range(1,N):
    if T[i] - time >= C:
        count = count + 1
        time = T[i]

print(count)
