import json
import collections

def search(o):
    n = 0
    if isinstance(o, collections.Mapping):
        for v in o.values():
            n += search(v)
    elif isinstance(o, str):
        return n
    elif isinstance(o, collections.Iterable):
        for v in o:
            n += search(v)
    elif isinstance(o, int):
        n += o
    return n

with open('12.txt') as f:
    obj = json.load(f)

    print(search(obj))
