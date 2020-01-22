use super::utils;

// https://stackoverflow.com/questions/6800193/what-is-the-most-efficient-way-of-finding-all-the-factors-of-a-number-in-python
/*
from math import sqrt
from functools import reduce
def factors(n):
    step = 2 if n%2 else 1
    return sum(set(reduce(list.__add__,
                ([i, n//i] for i in range(1, int(sqrt(n))+1, step) if n % i == 0) )))*10
                */

fn factors(n:i32) -> i32 {
    let step = if n % 2 == 1 {2} else {1};
    let start = 1;
    let end = ((n as f64).sqrt() as i32) + 1;

    let mut factor_set : utils::HashSetFnv<i32> = fastset!();
    let mut sum = 0;

    for i in (start..end).step_by(step) {
        if n % i != 0 {
            continue;
        }

        factor_set.insert(i);
        sum += i;

        let ndivi = n / i;
        if factor_set.insert(ndivi) {
            sum += ndivi;
        }
    }

    return sum * 10;
}

/*
def factor2(n):
    step = 2 if n%2 else 1
    return sorted(filter(lambda x:x>=n//50, set(reduce(list.__add__,
                ([i, n//i] for i in range(1, int(sqrt(n))+1, step) if n % i == 0)))))
*/

fn factors2(n:i32) -> i32
{
    let step = if n % 2 == 1 {2} else {1};
    let start = 1;
    let end = ((n as f64).sqrt() as i32) + 1;

    let mut factor_set : utils::HashSetFnv<i32> = fastset!();
    let mut sum = 0;
    let ndiv50 = n / 50;

    for i in (start..end).step_by(step) {
        if n % i != 0 {
            continue;
        }

        if i >= ndiv50 {
            if factor_set.insert(i) {
                sum += i * 11;
            }
        }
        let ndivi = n / i;
        if ndivi >= ndiv50 {
            if factor_set.insert(ndivi) {
                sum += ndivi * 11;
            }
        }
        else
        {
            break;
        }

    }

    return sum;
}

pub fn _run()
{
    let tgt = 36_000_000;

    // original python went by 1, but anything * 10 has the most factors.  For my input, 100 actually works too
    let mut house = 100;
    loop {
        let presents = factors(house);

        if presents > tgt {
            println!("day20-1: {}", house);
            break
        }

        house += 100;
    }

    house = 700_000;

    loop {
        let ps = factors2(house);

        if ps > tgt {
            println!("day20-2: {}", house);
            break
        }

        house += 1;
    }
}