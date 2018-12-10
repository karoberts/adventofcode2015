
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
                grid[k] = 0
            if t == 1:
                grid[k] += 1
            elif t == 2:
                grid[k] += 2
            elif t == 3:
                grid[k] = max(grid[k] - 1, 0)


grid = {}

with open('6.txt' ) as file:
    i = 0
    for line in (l.strip() for l in file):
        if i % 10 == 0:
            print(i)
        i += 1
        if line.startswith('turn on'):
            m = pat1.match(line)
            if m is not None:
                apply(1, int(m.group(1)),int(m.group(2)),int(m.group(3)),int(m.group(4)))
                continue

        if line.startswith('toggle'):
            m = pat2.match(line)
            if m is not None:
                apply(2, int(m.group(1)),int(m.group(2)),int(m.group(3)),int(m.group(4)))
                continue

        if line.startswith('turn off'):
            m = pat3.match(line)
            if m is not None:
                apply(3, int(m.group(1)),int(m.group(2)),int(m.group(3)),int(m.group(4)))
                continue

print(sum((grid[x] for x in grid)))
