N, M = (int(x) for x in input().split())
s = []
c = []

number = []
guess_number = 0

for _ in range(N):
    number.append(0)

for _ in range(M):
    a, b = (int(x) for x in input().split())
    if b > number[a-1]:
        number[a-1] = b

for i in range(N):
    guess_number = guess_number + number[i] * (10**(N-1-i))

if number[0] == 0 or guess_number == 0:
    guess_number = -1

print(guess_number)