import functools

# finds answer nearly immediately, but will take forever to complete the search

packages = [
    1,
    3,
    5,
    11,
    13,
    17,
    19,
    23,
    29,
    31,
    41,
    43,
    47,
    53,
    59,
    61,
    67,
    71,
    73,
    79,
    83,
    89,
    97,
    101,
    103,
    107,
    109,
    113
]

# part 1 sum = 1548  each = 516

#tgt = sum(packages) // 3 # part 1
tgt = sum(packages) // 4 # part 2
minlen = 999999
minent = 9999999999999999

def recur(curamt, sel, idx, grp1):
    global minlen
    global minent

    if curamt == tgt:
        if idx == 3:
            return True
        x = recur(0, sel, idx + 1, grp1)
        if idx == 1 and x:
            if len(grp1) <= minlen:
                ent = functools.reduce(lambda acc, x:x * acc, grp1, 1)
                minent = min(minent, ent)
                print('found!:', grp1, len(grp1), ent, minent)
                minlen = len(grp1)
        return x

    for i in reversed(packages):
        if i in sel: continue
        if curamt + i > tgt: continue

        sel_2 = set(sel)
        sel_2.add(i)
        grp_p = grp1
        if idx == 1:
            grp_p = list(grp1)
            grp_p.append(i)
        x = recur(curamt + i, sel_2, idx, grp_p)
        if x:
            if idx == 1:
                continue
            return True
    
recur(0, set(), 1, [])
