use regex::Regex;

use super::utils;

#[derive(Clone,Debug)]
struct Op
{
    op : String,
    left : Result<String, i32>,
    right : Option<Result<String, i32>>
}

type WireMap = utils::HashMapFnv<String, Option<i32>>;
type InputMap = utils::HashMapFnv<String, utils::HashSetFnv<String>>;
type OpMap = utils::HashMapFnv<String, Vec<Op>>;

fn exec(wires: &WireMap, op: &Op) -> Option<i32>
{
    let wl: i32 = match &op.left {
        Ok(left) => wires.get(left).unwrap().unwrap(),
        Err(left) => *left
    };

    let mut wr: i32 = 0;
    if let Some(rv) = &op.right {
        wr = match &rv {
            Ok(right) => wires.get(right).unwrap().unwrap(),
            Err(right) => *right
        };
    }

    match op.op.as_str() {
        "SEND" => return Some(wl),
        "NOT" => return Some(! wl),
        "AND" => return Some(wl & wr),
        "OR" => return Some(wl | wr),
        "LSHIFT" => return Some(wl << wr),
        "RSHIFT" => return Some(wl >> wr),
        _ => panic!("bad op: {}", op.op)
    }
}

fn runit(wires: &mut WireMap, inputs: &mut InputMap, ops: &mut OpMap) 
{
    let mut done : utils::HashSetFnv<usize> = utils::HashSetFnv::default();

    for w in wires.keys() {
        if ! inputs.contains_key(w) && ! ops.contains_key(w) {
            for (_, s) in inputs.iter_mut() {
                if s.contains(w) {
                    //println!("{} remove {} from {}", clock, w, i);
                    s.remove(w);
                }
            }
        }
    }

    while ops.len() > 0
    {
        let keys: Vec<String> = wires.keys().map(|s| s.clone()).collect();
        for w in keys {
            if ! ops.contains_key(&w) {
                continue;
            }

            done.clear();

            let olist = ops.get(&w).unwrap();
            for (i, op) in olist.iter().enumerate() {
                if let Ok(left) = &op.left {
                    if ops.contains_key(left) {
                        continue;
                    }
                }

                let apply = match &op.right {
                    Some(rv) => {
                        match rv {
                            Ok(right) => ! ops.contains_key(right),
                            _ => true
                        }
                    },
                    _ => true
                };
                
                if apply {
                    *wires.get_mut(&w).unwrap() = exec(&wires, &op);
                    done.insert(i);
                }
            }

            //println!("done = {:?}", done);

            if done.len() == 0 {
                continue;
            }
            else if done.len() == olist.len() {
                ops.remove(&w);
            }
            else
            {
                let mut leftover : Vec<Op> = Vec::with_capacity(olist.len() - done.len());
                for (i, op) in olist.iter().enumerate() {
                    if done.contains(&i) {
                        continue;
                    }
                    leftover.push(op.clone());
                }

                *ops.get_mut(&w).unwrap() = leftover;
            }
        }
    }
}

fn read_input(wires: &mut WireMap, inputs: &mut InputMap, ops: &mut OpMap) 
{
    let pat1 = Regex::new(r"^(\d+|[a-z]+) (AND|RSHIFT|LSHIFT|OR) (\d+|[a-z]+) -> ([a-z]+)$").unwrap();
    let pat2 = Regex::new(r"^NOT ([a-z]+) -> ([a-z]+)$").unwrap();
    let pat3 = Regex::new(r"^(\d+) -> ([a-z]+)$").unwrap();
    let pat4 = Regex::new(r"^([a-z]+) -> ([a-z]+)$").unwrap();

    let lines = utils::read_lines("../07.txt").expect("07.txt");
    for line in lines.map(|s| s.expect("fail")) {
        let mut m = pat1.captures(&line);
        if let Some(cap) = m {
            let g1 = cap.get(1).unwrap().as_str().to_owned();
            let g2 = cap.get(2).unwrap().as_str().to_owned();
            let g3 = cap.get(3).unwrap().as_str().to_owned();
            let g4 = cap.get(4).unwrap().as_str().to_owned();
            wires.entry(g1.clone()).or_insert(None);
            wires.entry(g4.clone()).or_insert(None);
            inputs.entry(g4.clone()).or_insert( utils::HashSetFnv::default() );
            let left : Result<String, i32>;
            let right : Option<Result<String, i32>>;
            if let Ok(g1int) = g1.parse::<i32>() {
                left = Err(g1int);
            }
            else {
                left = Ok(g1.clone());
                inputs.entry(g4.clone()).or_insert( utils::HashSetFnv::default() ).insert(g1.clone());
            }
            if let Ok(g3int) = g3.parse::<i32>() {
                right = Some(Err(g3int));
            }
            else {
                right = Some(Ok(g3.clone()));
                inputs.entry(g4.clone()).or_insert( utils::HashSetFnv::default() ).insert(g3.clone());
            }
            ops.entry(g4.clone()).or_insert(Vec::new()).push(Op { op: g2.clone(), left: left, right: right});
            continue;
        }

        m = pat2.captures(&line);
        if let Some(cap) = m {
            let g1 = cap.get(1).unwrap().as_str().to_owned();
            let g2 = cap.get(2).unwrap().as_str().to_owned();
            wires.entry(g1.clone()).or_insert(None);
            wires.entry(g2.clone()).or_insert(None);
            let i = inputs.entry(g2.clone()).or_insert( utils::HashSetFnv::default() );
            i.insert(g1.clone());
            ops.entry(g2.clone()).or_insert(Vec::new()).push(Op { op: "NOT".to_owned(), left: Ok(g1.clone()), right: None});
            continue;
        }

        m = pat3.captures(&line);
        if let Some(cap) = m {
            let g1 = utils::cap_to::<i32>(cap.get(1));
            let g2 = cap.get(2).unwrap().as_str().to_owned();
            wires.insert(g2, Some(g1));
            continue;
        }

        m = pat4.captures(&line);
        if let Some(cap) = m {
            let g1 = cap.get(1).unwrap().as_str().to_owned();
            let g2 = cap.get(2).unwrap().as_str().to_owned();
            wires.entry(g1.clone()).or_insert(None);
            wires.entry(g2.clone()).or_insert(None);
            let i = inputs.entry(g2.clone()).or_insert( utils::HashSetFnv::default() );
            i.insert(g1.clone());
            ops.entry(g2.clone()).or_insert(Vec::new()).push(Op { op: "SEND".to_owned(), left: Ok(g1.clone()), right: None });
            continue;
        }

        panic!("didn't match {}", line);
    }
}

pub fn _run() 
{
    let mut wires : WireMap = utils::HashMapFnv::default();
    let mut inputs : InputMap = utils::HashMapFnv::default();
    let mut ops : OpMap = utils::HashMapFnv::default();

    read_input(&mut wires, &mut inputs, &mut ops);

    runit(&mut wires, &mut inputs, &mut ops);

    let mut wire_a = wires.get("a").unwrap().unwrap();
    println!("day07-1: {:?}", wire_a);

    wires.clear();
    inputs.clear();
    ops.clear();

    read_input(&mut wires, &mut inputs, &mut ops);

    *wires.get_mut("b").unwrap() = Some(wire_a);

    runit(&mut wires, &mut inputs, &mut ops);

    wire_a = wires.get("a").unwrap().unwrap();
    println!("day07-2: {:?}", wire_a);
}
