N, Q = (int(x) for x in input().split())
H = []
T = []

R = 2
L = 1

count = 0

for i in range(Q):
    h,t = input().split()
    H.append(h)
    T.append(int(t))

for i in range(Q):
    if H[i] == "R":
        if R !=T[i]:
            if T[i] > L:
                count = count + abs(T[i]-R)
            elif T[i] < L:
                count = count + abs(N+R-T[i])
        R = T[i]
    else:
        if L !=T[i]:
            if T[i] > R:
                count = count + abs(T[i]-L)
            
            elif T[i] < R:
                count = count + abs(N+L-T[i])
        L = T[i]

print(count)        
