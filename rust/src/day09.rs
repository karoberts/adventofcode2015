use regex::Regex;

use super::utils;

type MappingType = utils::HashMapFnv<String, Vec<(String, i32)>>;
type TosType = utils::HashSetFnv<String>;

fn tryit(fr:&String, tos:&mut TosType, mapping:&MappingType, totplaces:usize, p:i32) -> i32
{
    if tos.len() == totplaces || !mapping.contains_key(fr) {
        return 0;
    }

    let mut mincost = std::i32::MAX;
    let mut maxcost = std::i32::MIN;
    let mut found = false;

    for to in mapping.get(fr).unwrap() {
        found = true;
        if tos.contains(&to.0) {
            continue;
        }
        tos.insert(to.0.clone());
        let r = to.1 + tryit(&to.0, tos, mapping, totplaces, p);
        if p == 1 && r < mincost {
            mincost = r;
        }
        else if p == 2 && r > maxcost {
            maxcost = r;
        }
        tos.remove(&to.0);
    }

    if !found {
        return 0;
    }
    if p == 1 { mincost } else { maxcost } 
}

pub fn _run()
{
    let pat1 = Regex::new(r"^([a-zA-Z]+) to ([a-zA-Z]+) = (\d+)$").unwrap();

    let mut places : utils::HashSetFnv<String> = fastset!();
    let mut mapping : MappingType = fastmap!();

    let lines = utils::read_lines("../09.txt").expect("09.txt");
    for line in lines.map(|s| s.expect("fail")) {
        let m = pat1.captures(&line);
        if let Some(cap) = m {
            let fr = cap.get(1).unwrap().as_str().to_owned();
            let to = cap.get(2).unwrap().as_str().to_owned();
            let cost = utils::cap_to::<i32>(cap.get(3));

            places.insert(to.clone());
            places.insert(fr.clone());

            mapping.entry(fr.clone()).or_insert(Vec::new()).push((to.clone(), cost));
            mapping.entry(to.clone()).or_insert(Vec::new()).push((fr.clone(), cost));
        }
    }

    let totplaces = places.len();

    let mut mincost = std::i32::MAX;
    let mut maxcost = std::i32::MIN;

    for fr in mapping.keys() {
        let mut tos : TosType = fastset!();
        tos.insert(fr.clone());
        let r1 = tryit(&fr, &mut tos, &mapping, totplaces, 1);
        if r1 < mincost {
            mincost = r1;
        }
        let r2 = tryit(&fr, &mut tos, &mapping, totplaces, 2);
        if r2 > maxcost {
            maxcost = r2;
        }
    }

    println!("day09-1: {}", mincost);
    println!("day09-2: {}", maxcost);
}