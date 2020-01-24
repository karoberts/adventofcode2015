use regex::Regex;

use super::utils;

struct OpType 
{
    op:String,
    r:usize,
    j:Option<i32>
}

fn run(regs:&mut [i32;2], program:&Vec<OpType>)
{
    let mut ip : i32 = 0;
    loop {
        if ip as usize >= program.len() || ip < 0 {
            //println!("HALT");
            //println!("{:?}", regs);
            break;
        }

        let pline = &program[ip as usize];
        let r = pline.r;
        let j = pline.j;

        match pline.op.as_str() {
            "jmp" => ip += j.unwrap(),
            "jie" => ip += if regs[r] % 2 == 0 {j.unwrap()} else {1},
            "jio" => ip += if regs[r] == 1 {j.unwrap()} else {1},
            _ => {
                match pline.op.as_str() {
                    "hlf" => regs[r] /= 2,
                    "tpl" => regs[r] *= 3,
                    "inc" => regs[r] += 1,
                    _ => panic!("!")
                };
                ip += 1;
            }
        };
    }
}


pub fn _run()
{
    let re = Regex::new(r"^([a-z]+) (\d+|[+\-]\d+|[ab])(?:, ([+\-]\d+))?$").unwrap();

    let lines = utils::read_lines("../23.txt").expect("23.txt");

    let mut program: Vec<OpType> = vec!();

    for line in lines.map(|s| s.expect("fail")) {
        let caps = re.captures(&line).unwrap();

        let mut o = OpType { 
            op: utils::cap_to_string(caps.get(1)), 
            r:0, 
            j:None 
        };
        if caps.get(3).is_some() {
            o.j = Some(utils::cap_to::<i32>(caps.get(3)));
        }
        if o.op == "jmp" {
            o.j = Some(utils::cap_to::<i32>(caps.get(2)));
        }
        else {
            o.r = match utils::cap_to_string(caps.get(2)).as_str() {
                "a" => 0,
                "b" => 1,
                _ => panic!("_")
            };
        }
        program.push(o);
    }

    let mut regs = [0, 0];
    run(&mut regs, &program);
    println!("day23-1: {}", regs[1]);
    regs = [1, 0];
    run(&mut regs, &program);
    println!("day23-2: {}", regs[1]);
}