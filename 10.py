
# from reddit, since mine takes a long time
import re
re_d = re.compile(r'((\d)\2*)')

def replace(match_obj):
    s = match_obj.group(1)
    return str(len(s)) + s[0]

s = '1321131112'
for i in range(50):
    if i == 40:
        print('part1', len(s))
    s = re_d.sub(replace,s)
print('part2', len(s))

exit()


def apply(s):
    i = 0
    next = ''
    while i < len(s):
        c = s[i]
        i += 1
        if i == len(s) or s[i] != c:
            next += '1' + c
        else:
            li = i
            while i < len(s) and s[i] == c:
                i += 1
            next += chr(i - li + 1 + ord('0')) + c
    return next
        

input = '1321131112'

part1 = 40
part2 = 50

for i in range(0, part2):
    print(i)
    if i == part1:
        print('part1', len(input))
    input = apply(input)

#print(input)
print('part2', len(input))
