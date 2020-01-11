use std::fs;

pub fn _run() 
{
    let contents = fs::read_to_string("../01.txt").expect("01.txt");

    //let v = vec![];
    let mut floor : i32 = 0;
    let mut basement : usize = 0;
    for (i, c) in contents.trim().chars().enumerate()
    {
        /*
        match c 
        {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!(format!("don't know {}", c))
        }*/

        // more concise
        floor += if c == '(' {1} else {-1};

        if basement == 0 && floor == -1
        {
            basement = i + 1;
        }
    }

    println!("day01-1: {}", floor);
    println!("day01-2: {}", basement);
}