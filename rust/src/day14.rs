use regex::Regex;

use super::utils;

enum Current
{
    Fly,
    Rest
}

struct Deer 
{
    speed: i32,
    flytime: i32,
    resttime: i32,
    dist: i32,
    cur: Current,
    fstart: i32,
    rstart: i32,
    flight: i32,
    points: i32
}

pub fn _run()
{
    let re = Regex::new(r"^([A-Za-z]+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds\.$").unwrap();

    let lines = utils::read_lines("../14.txt").expect("14.txt");

    let mut mapping : utils::HashMapFnv<String, Deer> = fastmap!();

    for line in lines.map(|s| s.expect("fail")) {
        let caps = re.captures(&line).unwrap();

        let d = Deer {
            speed: utils::cap_to(caps.get(2)),
            flytime: utils::cap_to(caps.get(3)),
            resttime: utils::cap_to(caps.get(4)),
            dist: 0,
            cur: Current::Fly,
            fstart: 0,
            rstart: 0,
            flight: 0,
            points: 0
        };

        mapping.insert(caps.get(1).unwrap().as_str().to_owned(), d);
    }

    let maxsec = 2503;

    for now in 1..maxsec {
        for (_, d) in mapping.iter_mut() {
            match d.cur {
                Current::Fly => {
                    d.dist += d.speed;
                    d.flight += 1;
                    if now - d.fstart == d.flytime {
                        d.cur = Current::Rest;
                        d.rstart = now;
                    }
                },
                Current::Rest => {
                    if now - d.rstart == d.resttime {
                        d.cur = Current::Fly;
                        d.fstart = now;
                    }
                }
            };
        }

        let maxdist = mapping.iter().max_by_key(|x| x.1.dist).unwrap().1.dist;

        for (_, d) in mapping.iter_mut() {
            if d.dist == maxdist {
                d.points += 1;
            }
        }
    }

    let maxdist = mapping.iter().max_by_key(|x| x.1.dist).unwrap();
    println!("day14-01: {}", maxdist.1.dist);

    let maxpts = mapping.iter().max_by_key(|x| x.1.points).unwrap();
    println!("day14-02: {}", maxpts.1.points);
}