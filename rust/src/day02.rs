use regex::Regex;
use std::cmp;

use super::utils;

pub fn _run() 
{
    let re = Regex::new(r"^(\d+)x(\d+)x(\d+)$").unwrap();

    let mut tot = 0;
    let mut tot2 = 0;
    let lines = utils::read_lines("../02.txt").expect("02.text");

    for line in lines.map(|s| s.expect("fail")) {
        let caps = re.captures(&line).unwrap_or_else(|| panic!("regex fail {}", line));
        let l: i32 = utils::cap_to(caps.get(1));
        let w: i32 = utils::cap_to(caps.get(2));
        let h: i32 = utils::cap_to(caps.get(3));

        let wh = w * h;
        let hl = h * l;
        let lw = l * w;
        let amt = (2 * lw) + (2 * wh) + (2 * hl) + cmp::min(cmp::min(lw, wh), hl);
        tot += amt;

        let perim = cmp::min(cmp::min(2 * l + 2 * w, 2 * w + 2 * h), 2 * h + 2 * l);

        tot2 += perim + (lw * h);
    }

    println!("day02-1: {}", tot);
    println!("day02-2: {}", tot2);
}