use super::utils;

const D: usize = 1000;

type TGrid = [[bool; D]; D];

fn get_state(grid:&TGrid, x:i32, y:i32) -> bool {
    if x < 0 || y < 0 || x > 99 || y > 99 {
        return false;
    }
    return grid[x as usize][y as usize];
}

lazy_static! {
    static ref TESTS : Vec<(i32, i32)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
}

fn count_on(grid:&TGrid, x:usize,y:usize) -> i32
{
    let mut on = 0;
    for t in TESTS.iter() {
        if get_state(grid, (x as i32) + t.0, (y as i32) + t.1) {
            on += 1;
        }
    }

    return on;
}

fn cycle(grid:&TGrid, nextgrid:&mut TGrid, mode:i32)
{
    for y in 0usize..100 {
        for x in 0usize..100 {
            let on_count = count_on(grid, x,y);
            if grid[x][y] {
                nextgrid[x][y] = on_count == 2 || on_count == 3;
            }
            else {
                nextgrid[x][y] = on_count == 3
            }
        }
    }

    if mode == 2 {
        nextgrid[0][0] = true;
        nextgrid[0][99] = true;
        nextgrid[99][0] = true;
        nextgrid[99][99] = true;
    }
}

fn count_all(grid:&TGrid) -> i32
{
    let mut co = 0;
    for y in 0..100 {
        for x in 0..100 {
            if grid[x][y] {
                co += 1;
            }
        }
    }
    co
}

pub fn _run()
{
    let lines = utils::read_lines("../18.txt").expect("18.txt");

    let mut grid1 = [[false; D] ; D];
    let mut grid2 = [[false; D] ; D];

    for (y, line) in lines.map(|x| x.unwrap()).enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid1[x][y] = c == '#';
            grid2[x][y] = c == '#';
        }
    }

    let mut nextgrid1 = [[false; D] ; D];
    let mut nextgrid2 = [[false; D] ; D];

    for _ in 1..101 {
        cycle(&grid1, &mut nextgrid1, 1);
        std::mem::swap(&mut grid1, &mut nextgrid1);

        cycle(&grid2, &mut nextgrid2, 2);
        std::mem::swap(&mut grid2, &mut nextgrid2);
    }

    println!("day18-01: {}", count_all(&grid1));
    println!("day18-02: {}", count_all(&grid2));
}