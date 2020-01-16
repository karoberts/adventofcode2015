
fn apply(s:&Vec<char>) -> Vec<char> {
    let mut i : usize = 0;
    let mut next : Vec<char> = Vec::with_capacity(s.len() * 2);

    while i < s.len() {
        let c = s[i];
        i += 1;
        if i == s.len() || s[i] != c {
            next.push('1');
        }
        else {
            let li = i;
            while i < s.len() && s[i] == c {
                i += 1;
            }
            next.push( ((i - li + 1) as u8 + ('0' as u8)) as char );
        }
        next.push(c);
    }

    return next;
}

pub fn _run()
{
    let mut input : Vec<char> = "1321131112".to_owned().chars().collect();

    //part1 = 40
    //part2 = 50

    for i in 0..50 {
        if i == 40 {
            println!("day10-1: {}", input.len());
        }
        input = apply(&input);
    }

    println!("day10-2: {}", input.len());
}