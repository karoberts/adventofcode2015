import re

analysis = {
'children': 3,
'cats': 7,
'samoyeds': 2,
'pomeranians': 3,
'akitas': 0,
'vizslas': 0,
'goldfish': 5,
'trees': 3,
'cars': 2,
'perfumes': 1}

pat = re.compile(r'^Sue (\d+): ((?:[a-z]+: \d+[, ]{0,2})+)$')

sues = []
for i in range(0, 501):
    sues.append({})

with open('16.txt') as file:
    for line in (l.strip() for l in file):
        m = pat.match(line)
        sueid = int(m.group(1))
        items = [x.strip() for x in m.group(2).split(',')]
        for i in items:
            subitems = i.split(':')
            sues[sueid][subitems[0].strip()] = int(subitems[1].strip())

matched = []
for i, sue in enumerate(sues):
    if i == 0:
        continue
    match = True
    for a, v in analysis.items():
        if a not in sue:
            continue
        if a == 'cats' or a == 'trees':
            if sue[a] > v:
                continue
            else:
                match = False
                break
        if a == 'pomeranians' or a == 'goldfish':
            if sue[a] < v:
                continue
            else:
                match = False
                break
        if sue[a] == v:
            continue
        match = False
        break
    if match:
        matched.append(i)

print(matched)