N = int(input())
A = []
for a in range(N):
    A.append(list(input()))
B = A

for i in range(int(N/2)):
    for j in range(N+1-i):
        for k in range(N+1-i):
            B[k][N+1-j-1] = A[j][k]
    A = B

for i in range(N):
    for j in range(N):
        print(B[i][j], end="")
    