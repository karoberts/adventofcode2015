use regex::Regex;

use super::utils;

type MappingType = utils::HashMapFnv<String, Vec<(String, i32)>>;
type TosType = utils::HashSetFnv<String>;

fn tryit(fr:&String, tos:&mut TosType, mapping:&MappingType, totplaces:usize, p:i32) -> i32
{
    if tos.len() == totplaces || !mapping.contains_key(fr) {
        return 0;
    }

    let mut cs : Vec<i32> = vec!();
    let mut tosp : TosType = fastset!();
    for to in mapping.get(fr).unwrap() {
        if tos.contains(&to.0) {
            continue;
        }
        tosp.clear();
        for v in tos.iter() {
            tosp.insert(v.clone());
        }
        tosp.insert(to.0.clone());
        let r = to.1 + tryit(&to.0, &mut tosp, mapping, totplaces, p);
        cs.push(r);
    }

    if cs.len() == 0 {
        return 0;
    }

    return if p == 1 { *cs.iter().min().unwrap() } else { *cs.iter().max().unwrap() }
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

    let mut costs1 : utils::HashMapFnv<String, i32> = fastmap!();
    let mut costs2 : utils::HashMapFnv<String, i32> = fastmap!();

    for fr in mapping.keys() {
        let mut tos : TosType = fastset!();
        tos.insert(fr.clone());
        costs1.insert(fr.clone(), tryit(&fr, &mut tos, &mapping, totplaces, 1));
        costs2.insert(fr.clone(), tryit(&fr, &mut tos, &mapping, totplaces, 2));
    }

    println!("day09-1: {}", costs1.values().min().unwrap());
    println!("day09-2: {}", costs2.values().max().unwrap());
}