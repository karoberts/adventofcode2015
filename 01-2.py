
with open('01.txt' ) as file:
    line = file.readline().strip()

floor = 0
for i, c in enumerate(line):
    floor += 1 if c == '(' else -1
    if floor == -1:
        print('basement', i + 1)

print(floor)
