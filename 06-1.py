
import re

pat1 = re.compile(r'^turn on (\d+),(\d+) through (\d+),(\d+)$')
pat2 = re.compile(r'^toggle (\d+),(\d+) through (\d+),(\d+)$')
pat3 = re.compile(r'^turn off (\d+),(\d+) through (\d+),(\d+)$')

def key(x,y):
    return str(x) + ',' + str(y)

def apply(t,x1,y1,x2,y2):
    for x in range(x1, x2+1):
        for y in range(y1, y2+1):
            k = key(x,y)
            if k not in grid:
                grid[k] = False
            if t == 1:
                grid[k] = True
            elif t == 2:
                grid[k] = not grid[k]
            elif t == 3:
                grid[k] = False


grid = {}

with open('06.txt' ) as file:
    for line in (l.strip() for l in file):
        #print(line)
        m = pat1.match(line)
        if m is not None:
            apply(1, int(m.group(1)),int(m.group(2)),int(m.group(3)),int(m.group(4)))
            continue

        m = pat2.match(line)
        if m is not None:
            apply(2, int(m.group(1)),int(m.group(2)),int(m.group(3)),int(m.group(4)))
            continue

        m = pat3.match(line)
        if m is not None:
            apply(3, int(m.group(1)),int(m.group(2)),int(m.group(3)),int(m.group(4)))
            continue

print(sum((1 for x in grid if grid[x])))
