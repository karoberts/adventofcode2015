import re

pat = re.compile(r'^(\d+)x(\d+)x(\d+)$')

def key(x, y):
    return str(x) + ',' + str(y)

with open('3.txt' ) as file:
    x = 0
    y = 0
    houses = {}
    for c in file.readline().strip():
        k = key(x,y)
        if k not in houses:
            houses[k] = 0
        houses[k] += 1

        if c == '>':
            x += 1
        elif c == '<':
            x -= 1
        elif c == '^':
            y -= 1
        elif c == 'v':
            y += 1

k = key(x,y)
if k not in houses:
    houses[k] = 0
houses[k] += 1

print(len(houses))
