use clap::Parser;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::Instant;

mod year2023;

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

enum Part {
    One,
    Two,
}

struct AdventDay {
    year: usize,
    day: usize,
}

impl AdventDay {
    fn load_input(&self) -> Result<std::string::String, io::Error> {
        // Set the path for the given day
        let path: PathBuf = [
            ".",
            "input",
            "data",
            self.year.to_string().as_str(),
            format!("day{:02}.txt", self.day).as_str(),
        ]
        .iter()
        .collect();

        // Try to read the input
        let input = fs::read_to_string(&path)?;
        Ok(input)
    }

    fn solve(&self, input: &str, part: Part) -> Option<usize> {
        match (self.year, self.day, part) {
            (2023, 1, Part::One) => Some(year2023::day01::part1(input)),
            (2023, 1, Part::Two) => Some(year2023::day01::part2(input)),
            (2023, 2, Part::One) => Some(year2023::day02::part1(input)),
            (2023, 2, Part::Two) => Some(year2023::day02::part2(input)),
            (2023, 3, Part::One) => Some(year2023::day03::part1(input)),
            (2023, 3, Part::Two) => Some(year2023::day03::part2(input)),
            (2023, 4, Part::One) => Some(year2023::day04::part1(input)),
            (2023, 4, Part::Two) => Some(year2023::day04::part2(input)),
            //(2023, 5, Part::One) => Some(year2023::day05::part1(input)),
            //(2023, 5, Part::Two) => Some(year2023::day05::part2(input)),
            (2023, 6, Part::One) => Some(year2023::day06::part1(input)),
            (2023, 6, Part::Two) => Some(year2023::day06::part2(input)),
            (2023, 7, Part::One) => Some(year2023::day07::part1(input)),
            (2023, 7, Part::Two) => Some(year2023::day07::part2(input)),
            _ => None,
        }
    }
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Set the year
    #[arg(short, long)]
    year: usize,

    /// Set the day
    /// If no day is set, all days will be run
    #[arg(short, long)]
    day: Option<usize>,
}

fn main() {
    // Parse the input arguments
    let args = Args::parse();
    let year = args.year;
    let days: Vec<_> = match args.day {
        Some(day) => vec![day],
        None => (1..=25).collect(),
    };

    for day in days.into_iter() {
        let advent_day = AdventDay { year, day };
        if let Ok(input) = advent_day.load_input() {
            let input = input.trim_end();

            let start_time = Instant::now();
            if let Some(answer1) = advent_day.solve(input, Part::One) {
                let time = elapsed_since(&start_time);
                println!(
                    "{year} - Day {:02} - Part 1:\n\tanswer = {answer1}\n\ttime = {time}\n",
                    day
                );
            }

            let start_time = Instant::now();
            if let Some(answer2) = advent_day.solve(input, Part::Two) {
                let time = elapsed_since(&start_time);
                println!(
                    "{year} - Day {:02} - Part 2:\n\tanswer = {answer2}\n\ttime = {time}\n",
                    day
                );
            }
        }
    }
}
