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

start = 'CRnCaSiRnBSiRnFArTiBPTiTiBFArPBCaSiThSiRnTiBPBPMgArCaSiRnTiMgArCaSiThCaSiRnFArRnSiRnFArTiTiBFArCaCaSiRnSiThCaCaSiRnMgArFYSiRnFYCaFArSiThCaSiThPBPTiMgArCaPRnSiAlArPBCaCaSiRnFYSiThCaRnFArArCaCaSiRnPBSiRnFArMgYCaCaCaCaSiThCaCaSiAlArCaCaSiRnPBSiAlArBCaCaCaCaSiThCaPBSiThPBPBCaSiRnFYFArSiThCaSiRnFArBCaCaSiRnFYFArSiThCaPBSiThCaSiRnPMgArRnFArPTiBCaPRnFArCaCaCaCaSiRnCaCaSiRnFYFArFArBCaSiThFArThSiThSiRnTiRnPMgArFArCaSiThCaPBCaSiRnBFArCaCaPRnCaCaPMgArSiRnFYFArCaSiThRnPBPMgAr'

#repls = [('H', 'HO'), ('H', 'OH'), ('O', 'HH')]
#start = 'HOHOHO'

unique_results = set()

for r in repls:
    print(r)
    pos = start.find(r[0], 0)
    while pos != -1 and pos < len(start):
        replaced = start[:pos] + r[1] + start[pos+len(r[0]):]
        unique_results.add(replaced)

        pos = start.find(r[0], pos + 1)

print(len(unique_results))