import re
import collections

pat1 = re.compile(r'^([A-Za-z]+) would (gain|lose) (\d+) happiness units by sitting next to ([a-zA-Z]+)\.$')

attendees = set()
mapping = collections.defaultdict(dict)

with open('13.txt' ) as file:
    for line in (l.strip() for l in file):
        m = pat1.match(line)
        attendees.add(m.group(1))
        attendees.add(m.group(4))

        mapping[m.group(1)][m.group(4)] = (1 if m.group(2) == 'gain' else -1) * int(m.group(3))

# part 2
for a in attendees:
    mapping[a]['me'] = 0
    mapping['me'][a] = 0
attendees.add('me')
# part 2

def tryit(fr, tos, order, cost):
    cs = []
    for to, m in mapping[fr].items():
        if to in tos:
            continue
        tosp = set(tos)
        tosp.add(to)
        orderp = list(order)
        orderp.append(to)
        cs.append(tryit(to, tosp, orderp, m + mapping[to][fr]))

    if len(cs) == 0:
        #print(order, cost + mapping[order[0]][order[-1]] + mapping[order[-1]][order[0]])
        return cost + mapping[order[0]][order[-1]] + mapping[order[-1]][order[0]]
    return max(cs) + cost

costs = {}

for fr in mapping.keys():
    tos = set()
    tos.add(fr)
    order = [fr]
    cost = tryit(fr, tos, order, 0)
    costs[fr] = cost
    #print(fr, cost, tos)

print(costs)

maxkey = max(costs, key=lambda x:costs[x])
print(maxkey)
print(costs[maxkey])