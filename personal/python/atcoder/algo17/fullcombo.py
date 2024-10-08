n = int(input())

count_perfect = 0
count_great = 0
count_failed = 0

for i in range(n):
    a = input()
    if a == "Perfect":
        count_perfect = count_perfect + 1
    if a == "Great":
        count_great = count_great + 1
    if (a == "Good") or (a == "Bad") or (a == "Miss"):
        count_failed = count_failed + 1

if count_failed == 0:
    if count_great == 0:
        print("All Perfect")
    else:
        print("Full Combo ")
else:
    print("Failed")