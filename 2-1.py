import re

pat = re.compile(r'^(\d+)x(\d+)x(\d+)$')

tot = 0
with open('2.txt' ) as file:
    for line in (l.strip() for l in file):
        m = pat.match(line)
        l = int(m.group(1))
        w = int(m.group(2))
        h = int(m.group(3))

        amt = (2 * l * w) + (2 * w * h) + (2 * h * l) + min([l*w, w*h, h*l])
        tot += amt

print(tot)
