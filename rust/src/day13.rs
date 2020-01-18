use regex::Regex;

use super::utils;

type MappingType = utils::HashMapFnv<String, utils::HashMapFnv<String, i32>>;
type TosType = utils::HashSetFnv<String>;

fn tryit(fr:&String, tos:&mut TosType, mapping:&MappingType, order:&mut Vec<String>, cost:i32) -> i32
{
    let mut max_cs : Option<i32> = None;

    for (to, m) in mapping.get(fr).unwrap() {
        if tos.contains(to) {
            continue;
        }
        tos.insert(to.clone());
        order.push(to.clone());

        let mapc = *mapping.get(to).unwrap().get(fr).unwrap_or(&0);
        let r = tryit(&to, tos, mapping, order, m + mapc);
        match max_cs {
            None => max_cs = Some(r),
            Some(v) => if r > v { max_cs = Some(r) }
        };
        tos.remove(to);
        order.pop();
    }

    match max_cs {
        None => {
            let mapc1 = *mapping.get(order.first().unwrap()).unwrap().get(order.last().unwrap()).unwrap_or(&0);
            let mapc2 = *mapping.get(order.last().unwrap()).unwrap().get(order.first().unwrap()).unwrap_or(&0);
            cost + mapc1 + mapc2
        },
        Some(v) => v + cost
    }
}

pub fn _run()
{
    let pat1 = Regex::new(r"^([A-Za-z]+) would (gain|lose) (\d+) happiness units by sitting next to ([a-zA-Z]+)\.$").unwrap();

    let mut attendees : utils::HashSetFnv<String> = fastset!();
    let mut mapping : MappingType = fastmap!();

    let lines = utils::read_lines("../13.txt").expect("13.txt");
    for line in lines.map(|s| s.expect("fail")) {
        let m = pat1.captures(&line);
        if let Some(cap) = m {
            attendees.insert(cap.get(1).unwrap().as_str().to_owned());
            attendees.insert(cap.get(4).unwrap().as_str().to_owned());

            let e = mapping.entry(cap.get(1).unwrap().as_str().to_owned()).or_insert( utils::HashMapFnv::default() );
            let h = utils::cap_to::<i32>(cap.get(3));
            e.insert(cap.get(4).unwrap().as_str().to_owned(), if cap.get(2).unwrap().as_str() == "gain" {1} else {-1} * h);
        }
    }

    let mut highest : i32 = std::i32::MIN;
    let mut tos : TosType = fastset!();
    let mut order : Vec<String> = Vec::new();

    for fr in mapping.keys() {
        tos.clear();
        tos.insert(fr.clone());
        order.clear();
        order.push(fr.clone());
        let c = tryit(&fr, &mut tos, &mapping, &mut order, 0);
        if c > highest {
            highest = c;
        }
    }

    println!("day13-1: {}", highest);

    let mut me : utils::HashMapFnv<String, i32> = fastmap!();
    for a in attendees.iter() {
        mapping.get_mut(a).unwrap().insert("me".to_owned(), 0);
        me.insert(a.clone(), 0);
    }
    mapping.insert("me".to_owned(), me);
    attendees.insert("me".to_owned());

    highest = std::i32::MIN;

    for fr in mapping.keys() {
        tos.clear();
        tos.insert(fr.clone());
        order.clear();
        order.push(fr.clone());
        let c = tryit(&fr, &mut tos, &mapping, &mut order, 0);
        if c > highest {
            highest = c;
        }
    }
    
    println!("day13-2: {}", highest);
}