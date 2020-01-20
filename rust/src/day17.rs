fn rec(mini:usize, s:i32, avl:&Vec<i32>, conts:&mut Vec<i32>) -> i32
{
    let mut num_combs = 0;
    for (i, c) in avl.iter().enumerate() {
        if *c < 0 || i < mini || s - *c < 0 {
        }
        else if s - *c == 0 {
            let mut ct = 0;
            for (j, x) in avl.iter().enumerate() {
                if j == i || *x < 0 {
                    ct += 1;
                }
            }
            conts.push(ct);
            num_combs += 1;
        }
        else {
            let mut avl_p = avl.clone();
            avl_p[i] = c * -1;
            num_combs += rec(i, s - c, &avl_p, conts);
        }
    }

    return num_combs
}

pub fn _run()
{
    let mut inputs = [
        43,
        3,
        4,
        10,
        21,
        44,
        4,
        6,
        47,
        41,
        34,
        17,
        17,
        44,
        36,
        31,
        46,
        9,
        27,
        38];
    inputs.sort_unstable_by_key(|&x| -x);

    let amt = 150;

    let mut conts : Vec<i32> = vec!();

    let mut n = 0;
    for (i, c) in inputs.iter().enumerate() {
        //print(i, c, len(inputs))
        let mut avail = inputs.to_vec();
        avail[i] = -1 * c;
        n += rec(i, amt - c, &avail, &mut conts)
    }

    println!("day17-1: {}", n);

    let minconts = conts.iter().min().unwrap();
    let counts = conts.iter().filter(|x| *x == minconts).count();
    println!("day17-2: {}", counts);
}