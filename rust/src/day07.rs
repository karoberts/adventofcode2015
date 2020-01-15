use regex::Regex;
use std::collections::{HashMap,HashSet};

use super::utils;

#[derive(Clone,Debug)]
struct Op
{
    op : String,
    left : String,
    right : Option<String>
}

type WireMap = HashMap<String, Option<i32>>;
type InputMap = HashMap<String, HashSet<String>>;
type OpMap = HashMap<String, Vec<Op>>;

fn exec(wires: &WireMap, op: &Op) -> Option<i32>
{
    let ldig = op.left.parse::<i32>().ok();
    let rdig = if op.right.is_none() { None } else { op.right.clone().unwrap().parse::<i32>().ok() };

    let mut wl: Option<i32> = None;
    if ldig.is_none() {
        wl = *wires.get(&op.left).unwrap();
    }
    let mut wr: Option<i32> = None;
    if rdig.is_none() {
        wr = *wires.get(&op.right.clone().unwrap_or(String::from("--"))).unwrap_or(&Some(0));
    }

    match op.op.as_str() {
        "SEND" => return ldig.or(wl),
        "NOT" => return Some(! wl.unwrap_or(0)),
        "AND" => return Some(ldig.or(wl).unwrap_or(0) & rdig.or(wr).unwrap()),
        "OR" => return Some(ldig.or(wl).unwrap_or(0) | rdig.or(wr).unwrap()),
        "LSHIFT" => return Some(wl.unwrap_or(0) << rdig.unwrap()),
        "RSHIFT" => return Some(wl.unwrap_or(0) >> rdig.unwrap()),
        _ => panic!("bad op: {}", op.op)
    }
}

fn runit(wires: &mut WireMap, inputs: &mut InputMap, ops: &mut OpMap) 
{
    let mut done : HashSet<usize> = HashSet::new();

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

            for (i, op) in ops.get(&w).unwrap().iter().enumerate() {
                if ops.contains_key(&op.left) {
                    continue;
                }
                
                if op.right.is_none() || !ops.contains_key(&op.right.clone().unwrap()) {
                    //println!("{} exec {} {{'op': '{}', 'left': '{}' 'right': {:?}", clock, w, op.op, op.left, op.right);
                    *wires.get_mut(&w).unwrap() = exec(&wires, op);
                    done.insert(i);
                }
            }

            let mut leftover : Vec<Op> = Vec::new();
            for (i, op) in ops.get(&w).unwrap().iter().enumerate() {
                if done.contains(&i) {
                    continue;
                }
                leftover.push(op.clone());
            }

            if leftover.is_empty() {
                ops.remove(&w);
            }
            else {
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
            inputs.entry(g4.clone()).or_insert( HashSet::new() );
            if g1.parse::<i32>().is_err() {
                inputs.entry(g4.clone()).or_insert( HashSet::new() ).insert(g1.clone());
            }
            if g3.parse::<i32>().is_err() {
                inputs.entry(g4.clone()).or_insert( HashSet::new() ).insert(g3.clone());
            }
            ops.entry(g4).or_insert(Vec::new()).push(Op { op: g2.clone(), left: g1.clone(), right: Some(g3.clone())});
            continue;
        }

        m = pat2.captures(&line);
        if let Some(cap) = m {
            let g1 = cap.get(1).unwrap().as_str().to_owned();
            let g2 = cap.get(2).unwrap().as_str().to_owned();
            wires.entry(g1.clone()).or_insert(None);
            wires.entry(g2.clone()).or_insert(None);
            let i = inputs.entry(g2.clone()).or_insert( HashSet::new() );
            i.insert(g1.clone());
            ops.entry(g2.clone()).or_insert(Vec::new()).push(Op { op: "NOT".to_owned(), left: g1.clone(), right: None});
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
            let i = inputs.entry(g2.clone()).or_insert( HashSet::new() );
            i.insert(g1.clone());
            ops.entry(g2).or_insert(Vec::new()).push(Op { op: "SEND".to_owned(), left: g1.clone(), right: None });
            continue;
        }

        panic!(format!("didn't match {}", line));
    }
}

pub fn _run() 
{
    let mut wires : WireMap = HashMap::new();
    let mut inputs : InputMap = HashMap::new();
    let mut ops : OpMap = HashMap::new();

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