
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
    input = apply(input)

#print(input)
print(len(input))