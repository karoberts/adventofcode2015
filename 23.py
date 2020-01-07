import re
from collections import namedtuple

pat_cmd = re.compile(r'^([a-z]+) (\d+|[+\-]\d+|[ab])(?:, ([+\-]\d+))?$')

OpType = namedtuple('Op', ['op', 'r', 'j', 'l'])

def run(regs):
    program = []
    with open('23.txt') as f:
        st = 0
        cur_op = None

        for line in (l.strip() for l in f):
            if line.startswith('#'):
                continue

            m = pat_cmd.match(line)
            r = None
            j = int(m.group(3)) if m.group(3) else None
            if m.group(1) == 'jmp':
                j = int(m.group(2))
            else:
                r = m.group(2)
            op = OpType(op = m.group(1), r = r, j = j, l = line)
            #print(repr(op))
            program.append(op)

    ip = 0
    stmts = 0
    while True:
        if ip >= len(program) or ip < 0:
            print('HALT')
            print(regs)
            break

        stmts += 1
        pline = program[ip]
        #printcodeline(ip, pline)
        #print('ip={} {} {}'.format(ip, regs, pline.l), end='')
        op = pline.op
        r = pline.r
        j = pline.j

        if op == 'jmp':
            ip += j
        elif op == 'jie':
            ip += (j if regs[r] % 2 == 0 else 1)
        elif op == 'jio':
            ip += (j if regs[r] == 1 else 1)
        else:
            if op == 'hlf':
                regs[r] //= 2
            elif op == 'tpl':
                regs[r] *= 3
            elif op == 'inc':
                regs[r] += 1

            ip += 1

        #print(' next ip={}, {}'.format(ip, regs))


regs = {'a': 0, 'b': 0} # part 1
run(regs)
regs = {'a': 1, 'b': 0} # part2
run(regs)
