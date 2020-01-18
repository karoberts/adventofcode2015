use regex::Regex;

use super::utils;

type MapType = utils::HashMapFnv<String, utils::HashMapFnv<String, i32>>;

pub fn _run()
{
    let re = Regex::new(r"^([A-Za-z]+): ([a-z]+) ([\-\d]+), ([a-z]+) ([\-\d]+), ([a-z]+) ([\-\d]+), ([a-z]+) ([\-\d]+), ([a-z]+) ([\-\d]+)$").unwrap();

    let input = ["Sprinkles: capacity 5, durability -1, flavor 0, texture 0, calories 5",
        "PeanutButter: capacity -1, durability 3, flavor 0, texture 0, calories 1",
        "Frosting: capacity 0, durability -1, flavor 4, texture 0, calories 6",
        "Sugar: capacity -1, durability 0, flavor 0, texture 2, calories 8"];

    let mut mapping : MapType = fastmap!();

    for line in input.iter() {
        let caps = re.captures(&line).unwrap();

        let m = mapping.entry(utils::cap_to_string(caps.get(1))).or_insert(fastmap!());

        m.insert(utils::cap_to_string(caps.get(2)), utils::cap_to(caps.get(3)));
        m.insert(utils::cap_to_string(caps.get(4)), utils::cap_to(caps.get(5)));
        m.insert(utils::cap_to_string(caps.get(6)), utils::cap_to(caps.get(7)));
        m.insert(utils::cap_to_string(caps.get(8)), utils::cap_to(caps.get(9)));
        m.insert(utils::cap_to_string(caps.get(10)), utils::cap_to(caps.get(11)));
    }

    let mut scores : Vec<i32> = vec!();
    let mut calscores : Vec<i32> = vec!();

    for f1 in 0..101 {
        for f2 in 0..(100 - f1 + 1) {
            for f3 in 0..(100 - (f1 + f2) + 1) {
                let f4 = 100 - (f1 + f2 + f3);
                let mut tot = 1;

                let spr = mapping.get("Sprinkles").unwrap();
                let mut cals = 0;

                for f in spr.keys() {
                    let mut val = f1 * spr.get(f).or(Some(&0)).unwrap() + 
                        f2 * mapping.get("PeanutButter").unwrap().get(f).or(Some(&0)).unwrap() +
                        f3 * mapping.get("Frosting").unwrap().get(f).or(Some(&0)).unwrap() +
                        f4 * mapping.get("Sugar").unwrap().get(f).or(Some(&0)).unwrap();

                    if f == "calories" {
                        cals = val;
                        continue;
                    }
                    val = std::cmp::max(val, 0);
                    tot *= val;
                }

                if cals == 500 {
                    calscores.push(tot);
                }

                scores.push(tot);
            }
        }
    }

    println!("day15-01: {}", scores.iter().max().unwrap());
    println!("day15-02: {}", calscores.iter().max().unwrap());
}