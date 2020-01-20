use regex::Regex;

use super::utils;

pub fn _run()
{
    let mut analysis : utils::HashMapFnv<String, i32> = fastmap!();

    analysis.insert("children".to_owned(), 3);
    analysis.insert("cats".to_owned(), 7);
    analysis.insert("samoyeds".to_owned(), 2);
    analysis.insert("pomeranians".to_owned(), 3);
    analysis.insert("akitas".to_owned(), 0);
    analysis.insert("vizslas".to_owned(), 0);
    analysis.insert("goldfish".to_owned(), 5);
    analysis.insert("trees".to_owned(), 3);
    analysis.insert("cars".to_owned(), 2);
    analysis.insert("perfumes".to_owned(), 1);

    let pat = Regex::new(r"^Sue (\d+): ((?:[a-z]+: \d+[, ]{0,2})+)$").unwrap();

    let mut sues : Vec<utils::HashMapFnv<String,i32>> = Vec::with_capacity(501);
    for _ in 0..501 {
        sues.push(fastmap!());
    }

    let lines = utils::read_lines("../16.txt").expect("16.txt");
    for line in lines.map(|x| x.unwrap()) {
        let caps = pat.captures(&line).unwrap();
        let sueid = utils::cap_to::<usize>(caps.get(1));
        let items = utils::cap_to_string(caps.get(2)).split(',').map(|x| x.to_owned()).collect::<Vec<_>>();
        for i in items.iter() {
            let subitems = i.split(':').map(|x| x.trim().to_owned()).collect::<Vec<_>>();
            sues[sueid].insert(subitems[0].clone(), subitems[1].parse::<i32>().unwrap());
        }
    }

    for i in 1..501 {
        let mut _match = true;
        let sue = &sues[i];
        for (a, v) in analysis.iter() {
            match sue.get(a) {
                None => continue,
                Some(x) => if x == v { continue; }
            }
            _match = false;
            break;
        }
        if _match {
            println!("day16-01: {}", i);
            break;
        }
    }

}