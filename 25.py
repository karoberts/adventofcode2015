# To continue, please consult the code grid in the manual.  Enter the code at row 2981, column 3075.

def calc_code(x):
    return (x * 252533) % 33554393


row = [None] * 6
row[0] = [20151125, 18749137, 17289845, 30943339, 10071777, 33511524] + [None] * 3070
row[1] = [31916031, 21629792, 16929656,  7726640, 15514188,  4041754] + [None] * 3070
row[2] = [16080970,  8057251,  1601130,  7981243, 11661866, 16474243] + [None] * 3070
row[3] = [24592653, 32451966, 21345942,  9380097, 10600672, 31527494] + [None] * 3070
row[4] = [   77061, 17552253, 28094349,  6899651,  9250759, 31663883] + [None] * 3070
row[5] = [33071741,  6796745, 25397450, 24659492,  1534922, 27995004] + [None] * 3070


r = 0 
c = 0
cn = 1
while True:
    cur = row[r][c]
    r += cn
    c = 0
    cn += 1
    if len(row) <= r:
        row.append([None] * 3075)
    for x in range(0, cn):
        cur = calc_code(cur)
        if r == 2981-1 and c == 3075-1:
            print(cur)
            exit()
        if len(row[r]) <= c:
            row[r].append(cur)
        else:
            row[r][c] = cur
        if x == cn - 1:
            break
        r -= 1
        c += 1


    