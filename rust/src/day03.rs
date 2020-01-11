use std::fs;
use std::collections::HashMap;

pub fn _run() 
{
    let (mut x, mut y) = (0, 0);
    let (mut rx, mut ry) = (0, 0);
    let (mut sx, mut sy) = (0, 0);
    let mut houses: HashMap<(i32, i32), i32> = HashMap::new();
    let mut houses2: HashMap<(i32, i32), i32> = HashMap::new();

    let contents = fs::read_to_string("../03.txt").expect("03.txt");

    let mut p_x : &mut i32;
    let mut p_y : &mut i32;

    for (i, c) in contents.trim().chars().enumerate()
    {
        *houses.entry((x,y)).or_insert(0) += 1;

        if i % 2 == 0 {
            *houses2.entry((sx, sy)).or_insert(0) += 1;
            p_x = &mut sx;
            p_y = &mut sy;
        }
        else {
            *houses2.entry((rx, ry)).or_insert(0) += 1;
            p_x = &mut rx;
            p_y = &mut ry;
        }

        match c {
            '>' => {
                x += 1;
                *p_x += 1;
            },
            '<' => {
                x -= 1;
                *p_x -= 1;
            },
            '^' => {
                y -= 1;
                *p_y -= 1;
            },
            'v' => {
                y += 1;
                *p_y += 1;
            },
            _ => panic!("{}", c)
        }
    }

    println!("day03-1: {}", houses.len());
    println!("day03-2: {}", houses2.len());
}