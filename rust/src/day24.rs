use super::utils;

fn recur(done:&mut bool, packages: &[i32;28], tgt:i32, minlen:&mut usize, minent:&mut i64, curamt:i32, sel: &utils::HashSetFnv<i32>, idx:usize, grp1:&Vec<i32>, nmins:&mut i32, howmany:i32) -> bool 
{
    if grp1.len() > *minlen {
        return false;
    }

    if curamt == tgt {
        if idx == 3 {
            return true;
        }
        let x = recur(done, packages, tgt, minlen, minent, 0, sel, idx + 1, grp1, nmins, howmany);
        if *done { return true; }
        if idx == 1 && x {
            if grp1.len() <= *minlen {
                let mut ent : i64 = 1;
                for g in grp1.iter() {
                    ent *= *g as i64;
                }
                *minent = std::cmp::min(*minent, ent);
                //println!("found!: {:?} {} {} {}", grp1, grp1.len(), ent, minent);
                *minlen = grp1.len();
                *nmins += 1;
                if *nmins >= howmany {
                    *done = true;
                }
                return true;
            }
        }
        return x;
    }

    for i in packages.iter().rev() {
        if sel.contains(i) { continue; }
        if curamt + i > tgt { continue; }

        let mut sel_2 : utils::HashSetFnv<i32> = fastset!();
        sel_2.extend(sel);
        sel_2.insert(*i);
        let x : bool;
        if idx == 1 {
            let mut grp_p : Vec<i32> = vec!();
            grp_p.extend(grp1);
            grp_p.push(*i);
            x = recur(done, packages, tgt, minlen, minent, curamt + i, &sel_2, idx, &grp_p, nmins, howmany);
        }
        else {
            x = recur(done, packages, tgt, minlen, minent, curamt + i, &sel_2, idx, grp1, nmins, howmany);
        }
        if *done { return true; }
        if x {
            if idx == 1 {
                continue;
            }
            return true;
        }
    }

    return false;
}

pub fn _run()
{
    let packages = [
        1,
        3,
        5,
        11,
        13,
        17,
        19,
        23,
        29,
        31,
        41,
        43,
        47,
        53,
        59,
        61,
        67,
        71,
        73,
        79,
        83,
        89,
        97,
        101,
        103,
        107,
        109,
        113
    ];

    let mut tgt : i32 = packages.iter().map(|x| *x).sum::<i32>() / 3; // part 1
    let mut minlen = 999999;
    let mut minent = 9999999999999999;
    let mut nmins = 0;

    let mut sel : utils::HashSetFnv<i32> = fastset!();
    let mut grp1 : Vec<i32> = vec!();
    let mut done : bool = false;
    // part 1, the very first minimum is the answer
    recur(&mut done, &packages, tgt, &mut minlen, &mut minent, 0, &sel, 1, &grp1, &mut nmins, 1);
    println!("day24-1: {}", minent);

    tgt = packages.iter().map(|x| *x).sum::<i32>() / 4; // part 2
    minlen = 999999;
    minent = 9999999999999999;

    sel = fastset!();
    grp1 = vec!();
    done = false;
    nmins = 0;
    // part 2, takes about 10 minimums to find the answer
    recur(&mut done, &packages, tgt, &mut minlen, &mut minent, 0, &sel, 1, &grp1, &mut nmins, 10);
    println!("day24-2: {}", minent);
}