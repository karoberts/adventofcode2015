
tgt = 36_000_000
#tgt = 1430000

# https://stackoverflow.com/questions/6800193/what-is-the-most-efficient-way-of-finding-all-the-factors-of-a-number-in-python
from math import sqrt
from functools import reduce
def factors(n):
    step = 2 if n%2 else 1
    return sum(set(reduce(list.__add__,
                ([i, n//i] for i in range(1, int(sqrt(n))+1, step) if n % i == 0))))*10

house = 1
while True:
    presents = factors(house)
    """
    for i in range(1, house+1):
        if house % i == 0:
            presents += 10 * i
            """

    #print('house', house, 'gets', factors(house))
    if presents > tgt:
        print('house', house)
        break

    if house % 10000 == 0:
        print(house, presents)

    house += 1