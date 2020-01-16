use super::utils;

fn calc(chars:&Vec<char>) -> (i32, i32)
{
    let mut i = 0;
    let mut code = 0;
    let mut string = 0;
    while i < chars.len()
    {
        let c = chars[i];
        match c {
            '"' => {
                code += 1;
                i += 1;
            },
            '\\' => {
                match chars[i+1] {
                    '\\' | '"' => {
                        code += 2;
                        string += 1;
                        i += 2;
                    },
                    'x' => {
                        code += 4;
                        string += 1;
                        i += 4;
                    }
                    _ => panic!("bad char {}", chars[i+1])
                }
            },
            _ => {
                code += 1;
                string += 1;
                i += 1;
            }
        }
    }

    return (code, string)
}

fn encode(chars:&Vec<char>, e:&mut Vec<char>)
{
    e.push('"');
    for c in chars {
        match c {
            '"' => {
                e.push('\\');
                e.push('"');
            },
            '\\' => {
                e.push('\\');
                e.push('\\');
            },
            _ => {
                e.push(*c);
            }
        }
    }
    e.push('"');
}

pub fn _run()
{
    let lines = utils::read_lines("../08.txt").expect("08.txt");

    let mut code1 = 0;
    let mut string1 = 0;
    let mut code2 = 0;
    let mut string2 = 0;
    for line in lines.map(|s| s.expect("fail")) {
        let chars : Vec<char> = line.chars().collect();

        let r = calc(&chars);
        //println!("{:?} {}", r, line);
        code1 += r.0;
        string1 += r.1;

        let mut e : Vec<char> = Vec::with_capacity(chars.len() * 2);
        encode(&chars, &mut e);
        let r2 = calc(&e);
        //println!("{:?} {}", r, line);
        code2 += r2.0;
        string2 += r2.1;
    }

    println!("day08-1: {}", code1 - string1);
    println!("day08-2: {}", code2 - string2);
}