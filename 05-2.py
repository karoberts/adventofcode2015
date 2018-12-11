import re

pat1 = re.compile(r'([a-z])([a-z]).*\1\2')
pat2 = re.compile(r'([a-z]).\1')

def is_nice(s):
    m1 = pat1.search(s)
    if m1 is None:
        return 0
    print(m1.groups(), end='')
    m2 = pat2.search(s)
    if m2 is None:
        return 1

    return 2

print(is_nice('qjhvhtzxzqqjkmpb'))
print(is_nice('xxyxx'))
print(is_nice('uurcxstgmygtbstg'))
print(is_nice('ieodomkazucvgmuy'))

with open('05.txt' ) as file:
    c = 0
    for line in (l.strip() for l in file):
        print(line, end='')
        n = is_nice(line)
        if n == 2:
            print(' nice')
            c += 1
        else:
            print('', n)

print(c)

