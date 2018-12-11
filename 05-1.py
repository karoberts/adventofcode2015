import re

pat = re.compile(r'^(\d+)x(\d+)x(\d+)$')

def is_nice(s):
    for bad in ['ab', 'cd', 'pq', 'xy']:
        if bad in s:
            return False

    vowel = 0
    for c in s:
        if c in 'aeiou':
            vowel += 1

    if vowel < 3:
        return False

    for let in range(ord('a'), ord('z') + 1):
        if (chr(let) + chr(let)) in s:
            return True

    return False


with open('05.txt' ) as file:
    c = 0
    for line in (l.strip() for l in file):
        if is_nice(line):
            c += 1

print(c)

