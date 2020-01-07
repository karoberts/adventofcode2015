
grid = {}
nextgrid = {}

def key(x,y):
    return str(x) + "," + str(y)

def printit():
    co = 0
    for y in range(0, my):
        for x in range(0, mx):
            co += 1 if grid[key(x,y)] else 0
            #print('#' if grid[key(x,y)] else '.', end='')
        #print()
    return co

def get_state(x,y):
    k = key(x,y)
    if k not in grid:
        return False
    return grid[k]

def count_on(x,y):
    return sum(1 for p in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] if get_state(x + p[0], y + p[1]))

def cycle():
    for y in range(0, my):
        for x in range(0, mx):
            k = key(x,y)
            if k in fixed_on:
                nextgrid[k] = True
                continue
            on_count = count_on(x,y)
            if grid[k]:
                nextgrid[k] = on_count in [2,3]
            else:
                nextgrid[k] = on_count == 3


my = 100
mx = 100

# part 1
fixed_on = set()

with open('18.txt' ) as file:
    for y, line in enumerate(l.strip() for l in file):
        for x, c in enumerate(line):
            grid[key(x,y)] = True if c == '#' else False
    for f in fixed_on:
        grid[f] = True

for step in range(1, 101):
    #print(step)
    cycle()
    grid = nextgrid
    nextgrid = {}

print('part1', printit())

#part 2
fixed_on = set(['0,0', '99,0', '0,99', '99,99'])

with open('18.txt' ) as file:
    for y, line in enumerate(l.strip() for l in file):
        for x, c in enumerate(line):
            grid[key(x,y)] = True if c == '#' else False
    for f in fixed_on:
        grid[f] = True

for step in range(1, 101):
    #print(step)
    cycle()
    grid = nextgrid
    nextgrid = {}

print('part2', printit())
