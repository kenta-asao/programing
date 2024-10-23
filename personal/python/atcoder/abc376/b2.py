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
            distance = min(abs(T[i]-R), abs(T[i]+N-R))
            if min(abs(L-R), abs(L+N-R)) < distance:
                count = count + distance
            else:
                count = count + N - distance
        R = T[i]
    else:
        if L !=T[i]:
            distance = min(abs(T[i]-L), abs(T[i]+N-L))
            if min(abs(R-L), abs(R+N-L)) > distance:
                count = count + distance
            else:
                count = count + N - distance
        L = T[i]

print(count)        
