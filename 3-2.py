import re

pat = re.compile(r'^(\d+)x(\d+)x(\d+)$')

def key(x, y):
    return str(x) + ',' + str(y)

def go(x, y, c):
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
    return (x,y)

with open('3.txt' ) as file:
    rx = 0
    ry = 0
    sx = 0
    sy = 0
    houses = {}
    w = 0
    for c in file.readline().strip():
        if w % 2 == 0:
            (sx, sy) = go(sx, sy, c)
        else:
            (rx, ry) = go(rx, ry, c)
        w += 1

if w % 2 == 0:
    (sx, sy) = go(sx, sy, c)
else:
    (rx, ry) = go(rx, ry, c)

print(len(houses))
