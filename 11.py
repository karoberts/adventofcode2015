def isvalid(s):
    chars = set(s)
    if 'i' in chars or 'o' in chars or 'l' in chars:
        return False

    pairs = set()
    i = 0
    while i < len(s) - 1:
        if s[i] == s[i+1]:
            i += 1
            pairs.add(s[i])
        i += 1
    if len(pairs) < 2:
        return False

    i = 1
    cur_s = s[0]
    s_len = 1
    while i < len(s):
        if ord(cur_s) == ord(s[i]) - 1:
            s_len += 1
            if s_len == 3:
                return True
        else:
            s_len = 1

        cur_s = s[i]
        i += 1

    return False

def incpass(s):
    i = len(s) - 1
    chars = [ord(c) - ord('a') for c in s]
    carry = 0
    while i >= 0:
        chars[i] = (chars[i] + 1)
        if chars[i] == 26:
            chars[i] = 0
            carry = 1
        elif chars[i] == 27:
            chars[i] = 1
            carry = 1
        else: 
            break
        i -= 1
    return ''.join((chr(c + ord('a')) for c in chars))

def nextpass(s):
    n = s
    while True:
        n = incpass(n)
        if isvalid(n):
            return n

#print(incpass('abcdfezz'))

#print(isvalid('abcdffaa'))
#print(nextpass('abcdefgh'))

input = 'vzbxkghb'

n = nextpass(input)
print('part1', n)
print('part2', nextpass(n))