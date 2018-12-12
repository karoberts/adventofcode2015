
inputs = sorted([
43,
3,
4,
10,
21,
44,
4,
6,
47,
41,
34,
17,
17,
44,
36,
31,
46,
9,
27,
38
], reverse=True)

inputs_t = sorted([20, 15, 10, 5, 5], reverse=True)

amt = 150
amt_t = 25

conts = []

def rec(mini, s, avl):

    num_combs = 0
    for i, c in enumerate(avl):
        if c < 0 or i < mini:
            continue
        if s - c < 0:
            continue
        if s - c == 0:
            avlP = list(avl)
            avlP[i] = -1 * c
            #print(avlP)
            conts.append([-x for x in avlP if x < 0])
            num_combs += 1
            continue

        avlP = list(avl)
        avlP[i] = c * -1
        num_combs += rec(i, s - c, avlP)

    return num_combs

n = 0
for i, c in enumerate(inputs):
    print(i, c, len(inputs))
    avail = list(inputs)
    avail[i] = -1 * c
    n += rec(i, amt - c, avail)

print('part1', n)

#print(conts)
minconts = min(conts, key=len)
counts = len([x for x in conts if len(x) == len(minconts)])
print('part2', counts)