
def calc(line):
    i = 0
    _code = 0
    _string = 0
    while i < len(line):
        c = line[i]
        if c == '"':
            _code += 1
            i += 1
        elif c == '\\':
            if line[i + 1] == '\\' or line[i + 1] == '"':
                _code += 2
                _string += 1
                i += 2
            elif line[i + 1] == 'x':
                _code += 4
                _string += 1
                i += 4
        else:
            _code += 1
            _string += 1
            i += 1

    #print(line, _code, _string)
    return (_code, _string)

def encode(line):
    e = []
    for c in line:
        if c == '"':
            e.append('\\')
            e.append('"')
        elif c == '\\':
            e.append('\\')
            e.append('\\')
        else:
            e.append(c)

    return '"' + ''.join(e) + '"'

#print(encode('"\\x27"'))
#exit()

with open('08.txt' ) as file:
    code = 0
    string = 0
    for line in (l.strip() for l in file):
        r = calc(line)
        code += r[0]
        string += r[1]
        #print(code, string, line)

    print('part1', code - string)
    
    orig = code

with open('08.txt' ) as file:
    code = 0
    string = 0
    for line in (l.strip() for l in file):
        r = calc(encode(line))
        code += r[0]
        string += r[1]
        #print('cs', code, string)

    n = code

    #print(n, orig)
    print('part2', n - orig)