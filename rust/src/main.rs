use std::env;
use std::time::{Instant, Duration};
use std::collections::HashMap;

#[macro_use]
mod utils;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day25;

fn run_timer(f : fn())
{
    let start = Instant::now();

    f();

    let duration = start.elapsed();
    println!(" ==> {:?}", duration);
}

fn main() 
{
    let args: Vec<String> = env::args().collect();
    let funcs: HashMap<String, fn()> = hashmap![
        "1".to_string() => day01::_run as fn(),
        "2".to_string() => day02::_run,
        "3".to_string() => day03::_run,
        "4".to_string() => day04::_run,
        "5".to_string() => day05::_run,
        "6".to_string() => day06::_run,
        "7".to_string() => day07::_run,
        "25".to_string() => day25::_run
    ];

    if args.len() > 1 {
        if args[1] == "all" {
            let mut total: Duration = Duration::from_secs(0);
            for f in funcs.values() {
                let start = Instant::now();
                f();
                let duration = start.elapsed();
                total += duration;
                println!("  ==> {:?}", duration);
            }
            println!();
            println!("  TOTAL: {:?}", total);
        }
        else {
            let f = funcs.get(&args[1]);
            match f {
                Some(func) =>  {
                    println!("Running Day {}", args[1]);
                    run_timer(*func);
                },
                None => panic!("day not recognized: {}", args[1])
            }
        }
    }
    else {
        run_timer( day07::_run );
    }
}
