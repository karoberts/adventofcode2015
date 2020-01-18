use regex::Regex;

use super::utils;

type MappingType = utils::HashMapFnv<String, utils::HashMapFnv<String, i32>>;
type TosType = utils::HashSetFnv<String>;

fn mapval(mapping:&MappingType, to:&String, fr:&String) -> i32 {
    *mapping.get(to).unwrap().get(fr).unwrap_or(&0)
}

fn tryit(fr:&String, tos:&TosType, mapping:&MappingType, order:&Vec<String>, cost:i32) -> i32
{
    let mut max_cs : Option<i32> = None;
    let mut tosp : TosType = fastset!();
    let mut orderp : Vec<String>;

    for (to, m) in mapping.get(fr).unwrap() {
        if tos.contains(to) {
            continue;
        }
        tosp.clear();
        for v in tos.iter() {
            tosp.insert(v.clone());
        }
        tosp.insert(to.clone());
        orderp = order.clone();
        orderp.push(to.clone());

        let r = tryit(&to, &tosp, mapping, &orderp, m + mapval(mapping, to, fr));
        match max_cs {
            None => max_cs = Some(r),
            Some(v) => if r > v { max_cs = Some(r) }
        };
    }

    match max_cs {
        None => cost + 
            mapval(mapping, order.first().unwrap(), order.last().unwrap()) + 
            mapval(mapping, order.last().unwrap(), order.first().unwrap()),
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

    for fr in mapping.keys() {
        let mut tos : TosType = fastset!();
        tos.insert(fr.clone());
        let order = vec![fr.clone()];
        let c = tryit(&fr, &tos, &mapping, &order, 0);
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
        let mut tos : TosType = fastset!();
        tos.insert(fr.clone());
        let order = vec![fr.clone()];
        let c = tryit(&fr, &tos, &mapping, &order, 0);
        if c > highest {
            highest = c;
        }
    }
    
    println!("day13-2: {}", highest);
}