use std::iter::FromIterator;

use super::utils;

fn isvalid(s:&Vec<char>) -> bool
{
    for c in s {
        match c {
            'i' | 'o' | 'l' => return false,
            _ => ()
        }
    }

    let mut pairs : utils::HashSetFnv<char> = fastset!();
    let mut i : usize = 0;
    while i < s.len() - 1 {
        if s[i] == s[i+1] {
            i += 1;
            pairs.insert(s[i]);
        }
        i += 1
    }
    if pairs.len() < 2 {
        return false;
    }

    i = 1;
    let mut cur_s = s[0];
    let mut s_len = 1;
    while i < s.len() {
        if (cur_s as u8) == (s[i] as u8) - 1 {
            s_len += 1;
            if s_len == 3 {
                return true;
            }
        }
        else {
            s_len = 1;
        }
        cur_s = s[i];
        i += 1;
    }

    return false;
}

fn incpass(s:&Vec<char>) -> Vec<char> 
{
    let mut i = s.len() - 1;
    let mut chars : Vec<u8> = s.iter().clone().map(|c| (*c as u8) - ('a' as u8)).collect();
    loop {
        chars[i] += 1;
        match chars[i] {
            26 => chars[i] = 0,
            27 => chars[i] = 1,
            _ => break
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }

    return chars.iter().clone().map(|c| (c + ('a' as u8)) as char).collect();
}

fn nextpass(s:&Vec<char>) -> Vec<char>
{
    let mut n : Vec<char> = s.clone();
    let mut x : Vec<char>;
    loop {
        x = incpass(&n);
        if isvalid(&x) {
            return x.clone();
        }
        n = x;
    }
}

pub fn _run()
{
    let input : Vec<char> = "vzbxkghb".to_owned().chars().collect();

    let mut n = nextpass(&input);

    println!("day11-1: {}", String::from_iter(n.iter()));

    n = nextpass(&n);

    println!("day11-2: {}", String::from_iter(n.iter()));
}