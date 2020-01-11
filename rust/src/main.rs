use std::env;
use std::time::{Instant};

mod utils;

mod day01;
mod day02;
mod day03;
mod day25;

fn main() 
{
    let args: Vec<String> = env::args().collect();
    let funcs: Vec<fn()> = vec![
        day01::_run, 
        day02::_run, 
        day03::_run, 
        day25::_run];

    if args.len() > 1 && args[1] == "all" {
        for f in funcs {
            let start = Instant::now();
            f();
            let duration = start.elapsed();
            println!("  ==> {:?}", duration);
        }
    }
    else {
        let start = Instant::now();

        day03::_run();

        let duration = start.elapsed();
        println!(" ==> {:?}", duration);
    }
}
