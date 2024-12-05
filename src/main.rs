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

trait Solution {
    fn part1(input: &str) -> usize;
    fn part2(input: &str) -> usize;
}

struct Challenge<const Y: usize, const D: usize>;

impl Challenge<Y, D> {
    // Define a function to load data
}

impl Solution for Challenge<2024, 1> {
    fn part1(input: &str) -> usize {
        3
    }

    fn part2(input: &str) -> usize {
        5
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
            let (answer1, answer2) = match day {
                1 => (day01::part1(input), day01::part2(input)),
                2 => (day02::part1(input), day02::part2(input)),
                3 => (day03::part1(input), day03::part2(input)),
                4 => (day04::part1(input), day04::part2(input)),
                5 => (day05::part1(input), day05::part2(input)),
                6 => (day06::part1(input), day06::part2(input)),
                7 => (day07::part1(input), day07::part2(input)),
                //8 => (day08::part1(input), day00::part2(input)),
                //9 => (day09::part1(input), day00::part2(input)),
                //10 => (day10::part1(input), day00::part2(input)),
                //11 => (day11::part1(input), day00::part2(input)),
                //12 => (day12::part1(input), day00::part2(input)),
                //13 => (day13::part1(input), day00::part2(input)),
                //14 => (day14::part1(input), day00::part2(input)),
                //15 => (day15::part1(input), day00::part2(input)),
                //16 => (day16::part1(input), day00::part2(input)),
                //17 => (day17::part1(input), day00::part2(input)),
                //18 => (day18::part1(input), day00::part2(input)),
                //19 => (day19::part1(input), day00::part2(input)),
                //20 => (day20::part1(input), day00::part2(input)),
                //21 => (day21::part1(input), day00::part2(input)),
                //22 => (day22::part1(input), day00::part2(input)),
                //23 => (day23::part1(input), day00::part2(input)),
                //24 => (day24::part1(input), day00::part2(input)),
                //25 => (day25::part1(input), day00::part2(input)),
                _ => unreachable!(),
            };
            println!("Part One: {}", answer1);
            println!("Part Two: {}", answer2);
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
