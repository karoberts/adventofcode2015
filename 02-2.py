import re

pat = re.compile(r'^(\d+)x(\d+)x(\d+)$')

tot = 0
with open('02.txt' ) as file:
    for line in (l.strip() for l in file):
        m = pat.match(line)
        l = int(m.group(1))
        w = int(m.group(2))
        h = int(m.group(3))

        dims = [2 * l + 2 * w, 2 * w + 2 * h, 2 * h + 2 * l]
        perim = min(dims)

        tot += perim + (l * w * h)

print(tot)
