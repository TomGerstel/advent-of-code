use std::env;
use std::fs;
use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

fn elapsed_since(start_time: &Instant) -> String {
    let elapsed = start_time.elapsed().as_micros();
    if elapsed >= 1_000_000 {
        let elapsed = elapsed as f64 / 1_000_000.0;
        format!("{elapsed:.1}s")
    } else if elapsed >= 1000 {
        let elapsed = elapsed as f64 / 1000.0;
        format!("{elapsed:.1}ms")
    } else {
        format!("{elapsed}µs")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let days: Vec<_> = match args.len() {
        1 => (1..=25).collect(),
        _ => args.iter().skip(1).map(|d| d.parse().unwrap()).collect(),
    };
    let global_start_time = Instant::now();
    for day in &days {
        println!("Day {}:", day);
        let path = format!("./data/day{:02}.txt", day);
        let input = fs::read_to_string(&path);
        let start_time = Instant::now();
        if let Ok(input) = input {
            let input = input.trim_end();
            let day_func = match day {
                1 => day01::run,
                2 => day02::run,
                3 => day03::run,
                4 => day04::run,
                5 => day05::run,
                6 => day06::run,
                7 => day07::run,
                //8 => day08::run,
                //9 => day09::run,
                //10 => day10::run,
                //11 => day11::run,
                //12 => day12::run,
                //13 => day13::run,
                //14 => day14::run,
                //15 => day15::run,
                //16 => day16::run,
                //17 => day17::run,
                //18 => day18::run,
                //19 => day19::run,
                //20 => day20::run,
                //21 => day21::run,
                //22 => day22::run,
                //23 => day23::run,
                //24 => day24::run,
                //25 => day25::run,
                _ => unreachable!(),
            };
            day_func(input);
            println!("Time: {}", elapsed_since(&start_time));
        } else {
            println!("ERROR: no data");
        }
        println!();
    }
    if days.len() > 1 {
        println!("TOTAL TIME: {}", elapsed_since(&global_start_time));
    }
}
