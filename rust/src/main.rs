use std::env;
use std::time::{Instant, Duration};

#[macro_use]
mod utils;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day25;

fn run_timer(f : fn()) -> Duration
{
    let start = Instant::now();

    f();

    let duration = start.elapsed();
    println!(" ==> {:?}", duration);
    return duration;
}

fn main() 
{
    let args: Vec<String> = env::args().collect();
    let funcs: Vec<Option<fn()>> = vec![
        Some(day01::_run),
        Some(day02::_run),
        Some(day03::_run),
        Some(day04::_run),
        Some(day05::_run),
        Some(day06::_run),
        Some(day07::_run),
        Some(day08::_run),
        Some(day09::_run),
        Some(day10::_run),
        Some(day11::_run),

        None, None, None, None, None, None, None, None, None, None, None, None, None, None, 

        Some(day25::_run)
    ];

    if args.len() > 1 {
        if args[1] == "all" {
            let mut total: Duration = Duration::from_secs(0);
            for f in funcs.iter().filter(|x| x.is_some()).map(|x| x.unwrap()) {
                total += run_timer(f);
            }
            println!();
            println!("  TOTAL: {:?}", total);
        }
        else {
            let f = funcs[ args[1].parse::<usize>().expect("invalid arg!") ];
            println!("Running Day {}", args[1]);
            run_timer(f.unwrap());
        }
    }
    else {
        run_timer( day11::_run );
    }
}
