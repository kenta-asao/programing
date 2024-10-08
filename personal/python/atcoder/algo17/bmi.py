height, weight = (float(x) for x in input().split())

height = height / 100

bmi = weight / (height * height)

if bmi < 20:
  print("A")
elif bmi >= 25:
  print("C")
else:
  print("B")