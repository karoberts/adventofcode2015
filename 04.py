import hashlib

key = 'ckczppom'
n = 1
p1 = False

while True:
    input = key + str(n)
    md5 = hashlib.md5(bytearray(input, 'utf-8')).hexdigest()
    if not p1 and md5.startswith('00000'): # part 1
        print('part1', n)
        p1 = True
    if md5.startswith('000000'): # part 2
        print('part2', n)
        break
    n += 1
