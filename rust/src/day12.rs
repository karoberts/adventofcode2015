use json;
use std::fs;

fn search(j:&json::JsonValue, mode:u8) -> i32
{
    let mut n = 0;
    if j.is_object() {
        if mode == 2 {
            for v in j.entries() {
                if v.1.is_string() && v.1.as_str().unwrap() == "red" {
                    return 0;
                }
            }
        }
        for v in j.entries() {
            n += search(v.1, mode);
        }
    }
    else if j.is_string() {
        return n;
    }
    else if j.is_array() {
        for v in j.members() {
            n += search(v, mode);
        }
    }
    else if j.is_number() {
        n += j.as_i32().unwrap();
    }
    return n
}

pub fn _run()
{
    let src = fs::read_to_string("../12.txt").expect("12.txt");

    let j = json::parse(src.as_str()).expect("parse");

    println!("day12-1: {}", search(&j, 1));
    println!("day12-2: {}", search(&j, 2));
}