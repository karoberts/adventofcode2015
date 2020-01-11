
// To continue, please consult the code grid in the manual.  Enter the code at row 2981, column 3075.

fn calc_code(v: u32) -> u32
{
    (((v as u64) * 252533) % 33554393) as u32
}

pub fn _run()
{
    let mut row : Vec<Vec<u32>> = vec![];
    row.push( vec![0; 6055] );
    row[0][0] = 20151125;

    let mut r = 0;
    let mut c = 0;
    let mut cn = 1;
    let mut size = 6055;
    
    'main: loop 
    {
        let mut cur = row[r][c];
        r += cn;
        c = 0;
        cn += 1;
        if row.len() <= r {
            row.push( vec![0; size] );
            size -= 1;
        }
        for x in 0..cn {
            cur = calc_code(cur);
            if r == 2980 && c == 3074 {
                println!("day25: {}", cur);
                break 'main;
            }

            row[r][c] = cur;

            if x < cn - 1 {
                r -= 1;
                c += 1;
            }
        }
    }
}