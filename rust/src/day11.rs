use std::iter::FromIterator;
use arrayvec::ArrayVec;

fn isvalid(s:&[char; 8]) -> bool
{
    for c in s {
        match c {
            'i' | 'o' | 'l' => return false,
            _ => ()
        }
    }

    let mut pairs = [false; 26];
    let mut i : usize = 0;
    let mut npairs = 0;
    while i < s.len() - 1 {
        if s[i] == s[i+1] {
            i += 1;
            let idx = (s[i] as u8 - 'a' as u8) as usize;
            if !pairs[idx] {
                pairs[idx] = true;
                npairs += 1;
            }
        }
        i += 1
    }
    if npairs < 2 {
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

fn incpass(s:&[char; 8], r:&mut [char; 8])
{
    let mut chars = [0u8; 8];
    for i in 0..8 {
        chars[i] = (s[i] as u8) - ('a' as u8);
    }
    let mut i = s.len() - 1;
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

    for i in 0..8 {
        r[i] = (chars[i] + ('a' as u8)) as char;
    }
}

fn nextpass(s:&mut [char; 8], r:&mut [char; 8])
{
    let mut a = s;
    let mut b = &mut ['0'; 8];
    loop {
        incpass(a, &mut b);
        if isvalid(b) {
            for i in 0..8 {
                r[i] = b[i];
            }
            return;
        }
        std::mem::swap(&mut a, &mut b);
    }
}

pub fn _run()
{
    let input : ArrayVec<[char; 8]> = "vzbxkghb".to_owned().chars().collect();
    let mut input_ar : [char; 8] = input.into_inner().unwrap();
    let mut output_ar : [char; 8] = ['0'; 8];

    nextpass(&mut input_ar, &mut output_ar);

    println!("day11-1: {}", String::from_iter(output_ar.iter()));

    let mut output_ar2 : [char; 8] = ['0'; 8];
    nextpass(&mut output_ar, &mut output_ar2);

    println!("day11-2: {}", String::from_iter(output_ar2.iter()));
}