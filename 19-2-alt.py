repls = sorted([
    ('Al', 'ThF'),
    ('Al', 'ThRnFAr'),
    ('B', 'BCa'),
    ('B', 'TiB'),
    ('B', 'TiRnFAr'),
    ('Ca', 'CaCa'),
    ('Ca', 'PB'),
    ('Ca', 'PRnFAr'),
    ('Ca', 'SiRnFYFAr'),
    ('Ca', 'SiRnMgAr'),
    ('Ca', 'SiTh'),
    ('F', 'CaF'),
    ('F', 'PMg'),
    ('F', 'SiAl'),
    ('H', 'CRnAlAr'),
    ('H', 'CRnFYFYFAr'),
    ('H', 'CRnFYMgAr'),
    ('H', 'CRnMgYFAr'),
    ('H', 'HCa'),
    ('H', 'NRnFYFAr'),
    ('H', 'NRnMgAr'),
    ('H', 'NTh'),
    ('H', 'OB'),
    ('H', 'ORnFAr'),
    ('Mg', 'BF'),
    ('Mg', 'TiMg'),
    ('N', 'CRnFAr'),
    ('N', 'HSi'),
    ('O', 'CRnFYFAr'),
    ('O', 'CRnMgAr'),
    ('O', 'HP'),
    ('O', 'NRnFAr'),
    ('O', 'OTi'),
    ('P', 'CaP'),
    ('P', 'PTi'),
    ('P', 'SiRnFAr'),
    ('Si', 'CaSi'),
    #('Th', 'ThCa'),
    ('Ti', 'BP'),
    ('Ti', 'TiTi'),
    ('e', 'HF'),
    ('e', 'NAl'),
    ('e', 'OMg')
    ], key= lambda x:len(x[1]), reverse=True)

"""
    nogood = set()
    for r in repls:
        for c in r[1]:
            co = 0
            for r2 in repls:
                co += 1 if c in r2[0] else 0
            if co == 0:
                nogood.add(c)
    print(nogood)
"""
        
end = 'CRnCaSiRnBSiRnFArTiBPTiTiBFArPBCaSiThSiRnTiBPBPMgArCaSiRnTiMgArCaSiThCaSiRnFArRnSiRnFArTiTiBFArCaCaSiRnSiThCaCaSiRnMgArFYSiRnFYCaFArSiThCaSiThPBPTiMgArCaPRnSiAlArPBCaCaSiRnFYSiThCaRnFArArCaCaSiRnPBSiRnFArMgYCaCaCaCaSiThCaCaSiAlArCaCaSiRnPBSiAlArBCaCaCaCaSiThCaPBSiThPBPBCaSiRnFYFArSiThCaSiRnFArBCaCaSiRnFYFArSiThCaPBSiThCaSiRnPMgArRnFArPTiBCaPRnFArCaCaCaCaSiRnCaCaSiRnFYFArFArBCaSiThFArThSiThSiRnTiRnPMgArFArCaSiThCaPBCaSiRnBFArCaCaPRnCaCaPMgArSiRnFYFArCaSiThRnPBPMgAr'

#end = 'OBCRnAlAr'

#repls = [('H', 'HO'), ('H', 'OH'), ('O', 'HH'), ('e', 'H'), ('e', 'O')]
#end = 'HOHOHO'

unique_ones = set()
min_steps = 99999999 # len(end) - 10

def compare(s, st):
    for i in range(0, min(len(s), len(end))):
        if s[i] == end[i]:
            continue
        return i - 1
    if s == end:
        print('match', st)
        quit()
    return -1

def recur(curmol, steps):
    global min_steps

    amts = []
    #if steps < 50:
    #print(steps, curmol)
    if curmol in unique_ones:
        return 9999999
    unique_ones.add(curmol)
    if steps > min_steps:
        return 9999999

    if len(unique_ones) % 10000 == 0:
        print(len(unique_ones), curmol)

    samepos = compare(curmol, steps)

    possibles = []
    for r in repls:
        pos = curmol.find(r[0], 0)
        if pos == -1:
            continue
        if r[1].startswith('CRn') and pos != 0:
            continue
        if samepos > -1 and pos < samepos:
            continue
        replaced = curmol[:pos] + r[1] + curmol[pos+len(r[0]):]
        cm = compare(replaced, steps + 1)
        possibles.append((cm, pos, r, replaced))
        #print(steps, r[0], '->', r[1], '@', pos)

    if len(possibles) == 0:
        return 99999999

    #print(possibles)
    z = 9
    for p in sorted(possibles, key=lambda x:(x[0], x[1]), reverse=True):
        #print(steps, r, min_steps)
        pos = p[1]
        r = p[2]
        replaced = p[3]
        if replaced == end:
            print('matched!', steps + 1)
            quit()
            amts.append(steps + 1)
            if steps + 1 < min_steps:
                min_steps = steps + 1
        if len(replaced) < len(end):
            amts.append(recur(replaced, steps + 1))
        #else:
        #    print('too long', replaced)

    return 99999999 if len(amts) == 0 else min(amts)

steps = recur('e', 0)
print(steps)