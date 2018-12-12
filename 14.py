import re
import collections

pat1 = re.compile(r'^([A-Za-z]+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds\.$')

mapping = {}

with open('14.txt' ) as file:
    for line in (l.strip() for l in file):
        m = pat1.match(line)

        mapping[m.group(1)] = {'speed': int(m.group(2)), 'flytime': int(m.group(3)), 'resttime': int(m.group(4)), 'dist': 0, 'cur': 'fly', 'fstart': 0, 'rstart': 0, 'flight': 0, 'points': 0}

print(mapping)
maxsec = 2503

#maxsec = 1000

for now in range(1, maxsec):
    for r, d in mapping.items():
        if d['cur'] == 'fly':
            d['dist'] += d['speed']
            d['flight'] += 1
            if now - d['fstart'] == d['flytime']:
                #print(now, r, 'resting', d['resttime'], d['dist'])
                d['cur'] = 'rest'
                d['rstart'] = now
        elif d['cur'] == 'rest':
            if now - d['rstart'] == d['resttime']:
                #print(now, r, 'flying', d['flytime'], d['dist'])
                d['cur'] = 'fly'
                d['fstart'] = now
    maxdist = max(mapping, key=lambda x:mapping[x]['dist'])
    for r, d in mapping.items():
        if d['dist'] == mapping[maxdist]['dist']:
            d['points'] += 1

print(mapping)
maxr = max(mapping, key=lambda x:mapping[x]['dist'])
print('part1', maxr)
print('part1', mapping[maxr]['dist'])

maxr = max(mapping, key=lambda x:mapping[x]['points'])
print('part2', maxr)
print('part2', mapping[maxr]['points'])
