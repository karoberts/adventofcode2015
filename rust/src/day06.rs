use regex::Regex;
use std::collections::HashMap;

use super::utils;

fn apply(grid1:&mut HashMap<(i32,i32), bool>, grid2:&mut HashMap<(i32,i32), i32>, cap:&regex::Captures, mode:&str)
{
    let x1 = utils::cap_to::<i32>(cap.get(1));
    let y1 = utils::cap_to::<i32>(cap.get(2));
    let x2 = utils::cap_to::<i32>(cap.get(3));
    let y2 = utils::cap_to::<i32>(cap.get(4));

    for y in y1..y2+1 {
        for x in x1..x2+1 {
            let k = (x,y);
            let v1 = grid1.entry(k).or_insert(false);
            let v2 = grid2.entry(k).or_insert(0);
            match mode {
                "on" => {
                    *v1 = true;
                    *v2 += 1;
                },
                "off" => {
                    *v1 = false;
                    *v2 = std::cmp::max(0, *v2 - 1);
                }
                "toggle" => {
                    *v1 = !*v1;
                    *v2 += 2;
                },
                _ => panic!("bad mode")
            };
        }
    }
}

pub fn _run() 
{
    let pat1 = Regex::new(r"^turn on (\d+),(\d+) through (\d+),(\d+)$").unwrap();
    let pat2 = Regex::new(r"^toggle (\d+),(\d+) through (\d+),(\d+)$").unwrap();
    let pat3 = Regex::new(r"^turn off (\d+),(\d+) through (\d+),(\d+)$").unwrap();

    let mut map1 : HashMap<(i32,i32), bool> = HashMap::new();
    let mut map2 : HashMap<(i32,i32), i32> = HashMap::new();

    let lines = utils::read_lines("../06.txt").expect("06.txt");
    for line in lines.map(|s| s.expect("fail")) {
        let mut m = pat1.captures(&line);
        if let Some(cap) = m {
            apply(&mut map1, &mut map2, &cap, "on");
            continue;
        }

        m = pat2.captures(&line);
        if let Some(cap) = m {
            apply(&mut map1, &mut map2, &cap, "toggle");
            continue;
        }

        m = pat3.captures(&line);
        if let Some(cap) = m {
            apply(&mut map1, &mut map2, &cap, "off");
            continue;
        }

        panic!(format!("didn't match {}", line));
    }

    let part1 = map1.iter().filter(|&(_,v)| *v).count();
    let part2 = map2.iter().map(|(_,v)| *v).sum::<i32>();

    println!("day05-1: {}", part1);
    println!("day05-2: {}", part2);
}