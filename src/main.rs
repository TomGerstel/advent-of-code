use clap::Parser;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::Duration;
use std::time::Instant;

mod year2023;

fn duration_to_string(duration: Duration) -> String {
    let micros = duration.as_micros();
    if micros >= 1_000_000 {
        let secs = micros as f64 / 1_000_000.0;
        format!("{secs:.1}s")
    } else if micros >= 1000 {
        let millis = micros as f64 / 1000.0;
        format!("{millis:.1}ms")
    } else {
        format!("{micros}µs")
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

// Define expected command line arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Set the year
    #[arg(short, long)]
    year: usize,

    // Set the day
    // If no day is set, all days will be run
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

    // Keep track of the total computation time
    let mut total_duration = Duration::default();

    // Compute the solution of each day
    for day in days.into_iter() {
        // Create advent day struct
        let advent_day = AdventDay { year, day };

        // Try to load input data
        if let Ok(input) = advent_day.load_input() {
            // Trim trailing whitespace for easy splitting into lines
            let input = input.trim_end();

            // Record the time of starting computations for logging
            let start_instant = Instant::now();

            // Try to solve the challenge and log the time spent doing so
            if let Some(answer1) = advent_day.solve(input, Part::One) {
                // Calculate the computation time and add to total
                let duration = start_instant.elapsed();
                total_duration += duration;

                // Print the computation time
                let duration_string = duration_to_string(duration);
                println!(
                    "{year} - Day {:02} - Part 1:\n\tanswer = {answer1}\n\ttime = {duration_string}\n",
                    day
                );
            }

            // Record the time of starting computations for logging
            let start_instant = Instant::now();

            // Try to solve the challenge and log the time spent doing so
            if let Some(answer2) = advent_day.solve(input, Part::Two) {
                // Calculate the computation time and add to total
                let duration = start_instant.elapsed();
                total_duration += duration;

                // Print the computation time
                let duration_string = duration_to_string(duration);
                println!(
                    "{year} - Day {:02} - Part 2:\n\tanswer = {answer2}\n\ttime = {duration_string}\n",
                    day
                );
            }
        }
    }

    // Print the total computation time
    let total_duration_string = duration_to_string(total_duration);
    println!("Total computation time: {total_duration_string}");
}
