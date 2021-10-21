use regex::Regex;

use super::utils;

const D: usize = 1000;

type TGrid1 = Vec<[bool; D]>;
type TGrid2 = Vec<[i32; D]>;

fn apply(grid1:&mut TGrid1, grid2:&mut TGrid2, cap:&regex::Captures, mode:&str)
{
    let x1 = utils::cap_to::<usize>(cap.get(1));
    let y1 = utils::cap_to::<usize>(cap.get(2));
    let x2 = utils::cap_to::<usize>(cap.get(3));
    let y2 = utils::cap_to::<usize>(cap.get(4));

    for y in y1..y2+1 {
        for x in x1..x2+1 {
            let v1 = &mut grid1[x][y];
            let v2 = &mut grid2[x][y];
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

    // too big for stack
    //let mut map1 = [[false; D] ; D];
    //let mut map2 = [[0i32; D] ; D];

    let mut map1 : TGrid1 = vec!();
    for _ in 0..D {
        map1.push([false; D]);
    }
    let mut map2 : TGrid2 = vec!();
    for _ in 0..D {
        map2.push([0i32; D]);
    }

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

        panic!("didn't match {}", line);
    }

    let mut part1 = 0;
    let mut part2 = 0;

    for y in 0..D {
        for x in 0..D {
            if map1[x][y] {
                part1 += 1;
            }
            part2 += map2[x][y];
        }
    }

    println!("day06-1: {}", part1);
    println!("day06-2: {}", part2);
}
