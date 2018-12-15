
from collections import defaultdict

tgt = 36_000_000
#tgt = 1_000_000

# https://stackoverflow.com/questions/6800193/what-is-the-most-efficient-way-of-finding-all-the-factors-of-a-number-in-python
from math import sqrt
from functools import reduce
def factor2(n):
    step = 2 if n%2 else 1
    return sorted(filter(lambda x:x>=n//50, set(reduce(list.__add__,
                ([i, n//i] for i in range(1, int(sqrt(n))+1, step) if n % i == 0)))))
def factors(n):
    step = 2 if n%2 else 1
    return list(set(reduce(list.__add__,
                ([i, n//i] for i in range(1, int(sqrt(n))+1, step) if n % i == 0))))

def get_ps(h):
    ps = 0
    for i in factors(h):
        #print('adding', i)
        if i * 50 > h:
            ps += int(h/ i) * 11
        else:
            ps += i * 50 * 11
    return ps

def get_ps_slow(h):
    elves = [None, min(h, 50)]
    ps = min(h, 50) * 11
    for i in range(2, h + 1):
        if i >= len(elves):
            elves.append(0)
        for j in range(2, i + 1):
            if elves[j] < 50 and i % j == 0:
                #print('2dd', j)
                ps += 11
                elves[j] += 1
    print(elves)
    return ps

def get_ps_slow2(h):
    ps = 0
    for i in range(1, int(sqrt(h))+h):
        amt = h // i
        ps += min(amt, 50) * 11
    return ps

def get_ps_slow3(h):
    ps = 0
    for i in range(h//50, h + 1):
        if h % i == 0:
            if h // i <= 50:
                print(i)
                ps += i * 11
    return ps

def get_ps_slow4(h):
    ps = 0
    for i in factor2(h):
        ps += i * 11
    return ps


#n = 1000
#print(part1(n))
#print(get_ps(n))
#print(get_ps_slow3(727407))
#print(get_ps_slow2(727407))
#print(get_ps_slow2(n))
#tgt = 1000

#print(get_ps_slow(52))
#print(get_ps_slow(52))
#print(get_ps_slow3(700000))
#print(get_ps_slow4(700000))
#exit()


house = 700_000
#lastp = get_ps_slow3(house - 1)
while True:
    ps = get_ps_slow4(house)

    #print('house', house, 'gets', ps)
    if ps > tgt:
        print('! house', house, ps)
        break

    if house % 1000 == 0:
        print(house, ps)

    house += 1