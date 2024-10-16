import math
N = int(input())


a=[ list(map(int,input().split(" "))) for i in range(N)]
sum = math.sqrt(a[0][0]**2 + a[0][1]**2)

for i in range(N-1):
    sum = sum + math.sqrt((a[i][0]-a[i+1][0])**2 + (a[i][1]-a[i+1][1])**2)

sum = sum + math.sqrt(a[N-1][0]**2 + a[N-1][1]**2)
print(sum)