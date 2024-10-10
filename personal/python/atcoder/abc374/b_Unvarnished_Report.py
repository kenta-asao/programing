S = input()
T = input()

if S == T:
    print(0)
else:
    for i in range(len(S)):
        if S[i] != T[i]:
            print(i+1)
            break
    else:
        print(i+2)
