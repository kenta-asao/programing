x = int(input("平方根を求めたい値を入力してください。\n"))
r = x

for i in range(100):
    r = (r + x/r)/2

print(r)