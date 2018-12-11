import re

pat = re.compile(r'^([a-zA-Z]+) to ([a-zA-Z]+) = (\d+)$')

mapping = {}
places = set()

with open('9.txt') as file:
    for line in (l.strip() for l in file):
        m = pat.match(line)
        fr = m.group(1)
        to = m.group(2)
        cost = int(m.group(3))
        places.add(to)
        places.add(fr)
        if fr not in mapping:
            mapping[fr] = []
        if to not in mapping:
            mapping[to] = []
        mapping[fr].append((to, cost))
        mapping[to].append((fr, cost))

totplaces = len(places)

def tryit(fr, tos):
    c = 0
    if fr not in mapping or len(tos) == totplaces:
        return c
    cs = []
    for to in mapping[fr]:
        if to[0] in tos:
            continue
        tosp = set(tos)
        tosp.add(to[0])
        cs.append(to[1] + tryit(to[0], tosp))
    if len(cs) == 0:
        return 0
    # return min(cs) # part 1
    return max(cs)

costs = {}

for fr in mapping.keys():
    tos = set()
    tos.add(fr)
    cost = tryit(fr, tos)
    costs[fr] = cost
    #print(fr, cost, tos)

print(costs)
# minkey = min(costs, key=lambda x:costs[x]) # part 1
# print(minkey)
# print(costs[minkey])

maxkey = max(costs, key=lambda x:costs[x])
print(maxkey)
print(costs[maxkey])