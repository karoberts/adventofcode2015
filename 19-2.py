import time 
from datetime import timedelta

repls = [
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
    ('Th', 'ThCa'),
    ('Ti', 'BP'),
    ('Ti', 'TiTi'),
    ('e', 'HF'),
    ('e', 'NAl'),
    ('e', 'OMg')
    ]

repls = sorted(repls, key=lambda x:len(x[1]), reverse=False)

end = 'CRnCaSiRnBSiRnFArTiBPTiTiBFArPBCaSiThSiRnTiBPBPMgArCaSiRnTiMgArCaSiThCaSiRnFArRnSiRnFArTiTiBFArCaCaSiRnSiThCaCaSiRnMgArFYSiRnFYCaFArSiThCaSiThPBPTiMgArCaPRnSiAlArPBCaCaSiRnFYSiThCaRnFArArCaCaSiRnPBSiRnFArMgYCaCaCaCaSiThCaCaSiAlArCaCaSiRnPBSiAlArBCaCaCaCaSiThCaPBSiThPBPBCaSiRnFYFArSiThCaSiRnFArBCaCaSiRnFYFArSiThCaPBSiThCaSiRnPMgArRnFArPTiBCaPRnFArCaCaCaCaSiRnCaCaSiRnFYFArFArBCaSiThFArThSiThSiRnTiRnPMgArFArCaSiThCaPBCaSiRnBFArCaCaPRnCaCaPMgArSiRnFYFArCaSiThRnPBPMgAr'

#end = 'CRnCaSiRnBSiRnFArTiBPTiTiBFArPBCaSiThSiRnTiBPBPMgArCaSiRnTiMgAr'

#repls = [('H', 'HO'), ('H', 'OH'), ('O', 'HH'), ('e', 'H'), ('e', 'O')]
#end = 'HOHOHO'

unique_ones = set()
min_steps = len(end)
min_mol = len(end)

def recur(curmol, steps):
    global min_steps
    global min_mol

    amts = []

    if 'e' in curmol:
        print('found e', steps, curmol)
        return 99999999

    if curmol in unique_ones:
        return 99999999
    unique_ones.add(curmol)
    if steps > min_steps:
        return 99999999

    if len(unique_ones) % 10000 == 0:
        print(len(unique_ones))

    nfound = 0
    for r in repls:
        #print(steps, r, min_steps, curmol)
        #print(len(curmol))
        pos = curmol.find(r[1], 0)
        if pos > -1:
            nfound += 1
        while pos != -1 and pos < len(end):
            if r[0].startswith('CRn') and pos != 0:
                break
            replaced = curmol[:pos] + r[0] + curmol[pos+len(r[1]):]

            if len(replaced) < min_mol:
                min_mol = len(replaced)
                print('new min', min_mol, replaced)

            if replaced == 'e':
                print('matched!', steps + 1)
                quit()
                amts.append(steps + 1)
                if steps + 1 < min_steps:
                    min_steps = steps + 1
            elif len(replaced) > len(curmol):
                print('longer', replaced, curmol)
            else:
                amts.append(recur(replaced, steps + 1))

            pos = curmol.find(r[1], pos + 1)
            break

    #if nfound == 0:
    #print('eol', len(curmol), steps, len(unique_ones), curmol)

    return 99999999 if len(amts) == 0 else min(amts)

start_time = time.monotonic()
steps = recur(end, 0)
print(steps)
end_time = time.monotonic()
print(timedelta(seconds=end_time - start_time))