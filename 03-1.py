import re

with open('03.txt' ) as file:
    x = 0
    y = 0
    houses = {}
    for c in file.readline().strip():
        k = (x,y)
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

print(len(houses))
