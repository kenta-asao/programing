card = []
line = 3
for _ in range(0, line):
    row = list(map(int, input().split()))
    card.append(row)

M = []

for i in range(line):
    row = []
    for j in range(line):
        row.append(False)
    M.append(row)

N = int(input())

for _ in range(N):
    a = int(input())
    for i in range(line):
        for j in range(line):
            if card[i][j] == a:
                M[i][j] = True

bingo = False

for i in range(line):
    if M[i][0] == M[i][1] == M[i][2] == True:
        bingo = True
    elif M[0][i] == M[1][i] == M[2][i] == True:
        bingo = True
    
if M[0][0] == M[1][1] == M[2][2] == True:
    bingo = True
elif M[0][2] == M [1][1] == M[2][0] == True:
    bingo = True

if bingo == True:
    print("Yes")
else:
    print("No")