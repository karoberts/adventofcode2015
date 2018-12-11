import re

pat1 = re.compile(r'^(\d+|[a-z]+) (AND|RSHIFT|LSHIFT|OR) (\d+|[a-z]+) -> ([a-z]+)$')
pat2 = re.compile(r'^NOT ([a-z]+) -> ([a-z]+)$')
pat3 = re.compile(r'^(\d+) -> ([a-z]+)$')
pat4 = re.compile(r'^([a-z]+) -> ([a-z]+)$')

wires = {}
inputs = {}
ops = {}

with open('07.txt' ) as file:
    for line in (l.strip() for l in file):

        m = pat1.match(line)
        if m is not None:
            if m.group(1) not in wires:
                wires[m.group(1)] = None
            if m.group(4) not in wires:
                wires[m.group(4)] = None
            if m.group(4) not in inputs:
                inputs[m.group(4)] = set()
            if not m.group(1).isdigit():
                inputs[m.group(4)].add(m.group(1))
            if not m.group(3).isdigit():
                inputs[m.group(4)].add(m.group(3))
            if m.group(4) not in ops:
                ops[m.group(4)] = []
            ops[m.group(4)].append({'op': m.group(2), 'left': m.group(1), 'right': m.group(3)})
            continue

        m = pat2.match(line)
        if m is not None:
            if m.group(1) not in wires:
                wires[m.group(1)] = None
            if m.group(2) not in wires:
                wires[m.group(2)] = None
            if m.group(2) not in inputs:
                inputs[m.group(2)] = set()
            inputs[m.group(2)].add(m.group(1))
            if m.group(2) not in ops:
                ops[m.group(2)] = []
            ops[m.group(2)].append({'op': 'NOT', 'left': m.group(1), 'right': None})
            continue

        m = pat3.match(line)
        if m is not None:
            wires[m.group(2)] = int(m.group(1))
            continue

        m = pat4.match(line)
        if m is not None:
            if m.group(1) not in wires:
                wires[m.group(1)] = None
            if m.group(2) not in wires:
                wires[m.group(2)] = None
            if m.group(2) not in inputs:
                inputs[m.group(2)] = set()
            inputs[m.group(2)].add(m.group(1))
            if m.group(2) not in ops:
                ops[m.group(2)] = []
            ops[m.group(2)].append({'op': 'SEND', 'left': m.group(1), 'right': None})
            continue

        pass

# uncomment for part 2 
# wires['b'] = 16076

def exec(w, op):
    l = op['left']
    r = op['right']
    ldig = l.isdigit()
    rdig = r.isdigit() if r is not None else False
    li = None
    if ldig:
        li = int(l)
    ri = None
    if rdig:
        ri = int(r)

    if op['op'] == 'SEND':
        wires[w] = li if ldig else wires[l]
    elif op['op'] == 'NOT':
        wires[w] = ~ wires[l]
    elif op['op'] == 'AND':
        wires[w] = (li if ldig else wires[l]) & (ri if rdig else wires[r])
    elif op['op'] == 'OR':
        wires[w] = (li if ldig else wires[l]) | (ri if rdig else wires[r])
    elif op['op'] == 'LSHIFT':
        wires[w] = wires[l] << ri
    elif op['op'] == 'RSHIFT':
        wires[w] = wires[l] >> ri
    pass

for clock in range(0, 100):
    opdone = False
    wiresat = False
    for w in wires:
        if w not in inputs.keys() and w not in ops:
            wiresat = True
            for i in inputs:
                if w in inputs[i]:
                    print(clock, 'remove', w, 'from', i)
                    inputs[i].remove(w)

    for w in wires:
        if w in ops:
            done = set()
            for i, op in enumerate(ops[w]):
                left = op['left']
                right = op['right']
                if left not in ops and (right is None or right not in ops):
                    print(clock, 'exec', w, op)
                    opdone = True
                    exec(w, op)
                    done.add(i)
            leftover = []
            for i, op in enumerate(ops[w]):
                if i in done:
                    continue
                leftover.append(op)
            if len(leftover) > 0:
                ops[w] = leftover
            else:
                del ops[w]

    if not opdone:
        break


#print(wires)
#print(inputs)
print(ops)
print(wires['a'])