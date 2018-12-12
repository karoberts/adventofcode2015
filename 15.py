import re

input = ['Sprinkles: capacity 5, durability -1, flavor 0, texture 0, calories 5',
'PeanutButter: capacity -1, durability 3, flavor 0, texture 0, calories 1',
'Frosting: capacity 0, durability -1, flavor 4, texture 0, calories 6',
'Sugar: capacity -1, durability 0, flavor 0, texture 2, calories 8']

pat1 = re.compile(r'^([A-Za-z]+): ([a-z]+) ([\-\d]+), ([a-z]+) ([\-\d]+), ([a-z]+) ([\-\d]+), ([a-z]+) ([\-\d]+), ([a-z]+) ([\-\d]+)$')

mapping = {}

for line in input:
    m = pat1.match(line)
    mapping[m.group(1)] = {m.group(2): int(m.group(3)),
        m.group(4): int(m.group(5)),
        m.group(6): int(m.group(7)),
        m.group(8): int(m.group(9)),
        m.group(10): int(m.group(11))}

print(mapping)

scores = []
calscores = []
for f1 in range(0, 101):
    for f2 in range(0, 100 - f1 + 1):
        for f3 in range(0, 100 - (f1 + f2) + 1):
            f4 = 100 - (f1 + f2 + f3)
            tot = 1

            for f in mapping['Sprinkles'].keys():
                if f == 'calories':
                    continue
                val = f1 * mapping['Sprinkles'][f] + f2 * mapping['PeanutButter'][f] + f3 * mapping['Frosting'][f] + f4 * mapping['Sugar'][f]
                val = max(val, 0)
                tot *= val
            
            #print(f1,f2,f3,f4, tot)

            f = 'calories'
            cals = f1 * mapping['Sprinkles'][f] + f2 * mapping['PeanutButter'][f] + f3 * mapping['Frosting'][f] + f4 * mapping['Sugar'][f]
            if cals == 500:
                calscores.append(tot)

            scores.append(tot)

#print(scores)
print('part1', max(scores))
print('part2', max(calscores))