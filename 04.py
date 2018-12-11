import hashlib

key = 'ckczppom'
n = 1

while True:
    input = key + str(n)
    md5 = hashlib.md5(bytearray(input, 'utf-8')).hexdigest()
    # if md5.startswith('00000'): # part 1
    if md5.startswith('000000'): # part 2
        print(n)
        break
    n += 1
