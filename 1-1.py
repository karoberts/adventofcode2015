
with open('1.txt' ) as file:
    line = file.readline().strip()

floor = 0
for c in line:
    floor += 1 if c == '(' else -1

print(floor)
