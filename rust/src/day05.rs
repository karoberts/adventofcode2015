use std::collections::HashMap;

use super::utils;

fn is_nice(chars:&Vec<char>, pairs:&mut HashMap<(char,char), usize>) -> (bool, bool) {

    let mut vowel = 0;
    let mut found_double = false;
    let mut part1_notnice = false;

    //let mut pairs : HashMap<(char, char), usize> = HashMap::new();
    let mut found_yny = false;
    let mut found_pair = false;

    for (i, &c) in chars.iter().enumerate() {
        if !part1_notnice {
            if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
                vowel += 1;
            }

            if i < chars.len() - 1 {
                if (c == 'a' && chars[i + 1] == 'b') ||
                (c == 'c' && chars[i + 1] == 'd') ||
                (c == 'p' && chars[i + 1] == 'q') ||
                (c == 'x' && chars[i + 1] == 'y') 
                {
                    part1_notnice = true;
                }

                if c == chars[i + 1] {
                    found_double = true;
                }
            }
        }

        if i < chars.len() - 1 {
            let next = chars[i + 1];
            let nextnext = if i + 2 == chars.len() {'-'} else {chars[i + 2]};
            match pairs.get(&(c, next)) {
                Some(idx) => found_pair |= i > *idx,
                _ => {
                    pairs.insert((c, next), i + 1);
                    ()
                }
            }

            if c == nextnext {
                found_yny = true;
            }
        }
    }

    if vowel < 3 {
        part1_notnice = true;
    }

    return (!part1_notnice && found_double, found_yny && found_pair);
}

pub fn _run() 
{
    let mut part1 = 0;
    let mut part2 = 0;

    let mut hmap: HashMap<(char, char), usize> = HashMap::with_capacity(20);

    let lines = utils::read_lines("../05.txt").expect("05.txt");
    for line in lines.map(|s| s.expect("fail")) {
        let chars : Vec<char> = line.chars().collect();

        hmap.clear();
        let ret = is_nice(&chars, &mut hmap);

        if ret.0 {
            part1 += 1;
        }
        if ret.1 {
            part2 += 1;
        }
    }

    println!("day05-1: {}", part1);
    println!("day05-2: {}", part2);
}