use std::env;
use std::time::{Instant, Duration};

mod utils;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day25;

fn main() 
{
    let args: Vec<String> = env::args().collect();
    let funcs: Vec<fn()> = vec![
        day01::_run, 
        day02::_run, 
        day03::_run, 
        day04::_run, 
        day05::_run, 
        day25::_run];

    if args.len() > 1 && args[1] == "all" {
        let mut total: Duration = Duration::from_secs(0);
        for f in funcs {
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
        let start = Instant::now();

        day05::_run();

        let duration = start.elapsed();
        println!(" ==> {:?}", duration);
    }
}
