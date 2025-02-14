use clap::Parser;
use std::time;
mod solver;
mod year2023;
use solver::{Part, Puzzle};

fn main() {
    // Parse the input arguments
    let args = Args::parse();
    let year = args.year;
    let days: Vec<_> = match args.day {
        Some(day) => vec![day],
        None => (1..=25).collect(),
    };

    // Keep track of the total computation time
    let mut total_duration = time::Duration::default();

    // Solve both puzzles for each day
    for day in days.into_iter() {
        for part in [Part::One, Part::Two] {
            // Initialise the puzzle and solve it
            let puzzle = Puzzle::new(year, day, part);
            let outcome = puzzle.load_and_solve().unwrap_or_default();

            // Do some logging
            println!("{puzzle}: {outcome}");

            // Keep track of the total computation time
            total_duration += outcome.duration();
        }
    }

    // Print the total computation time
    let total_duration_string = duration_to_string(total_duration);
    println!();
    println!("Total computation time: {total_duration_string}");
    println!();
}

// Define expected command line arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Set the year
    year: usize,

    // Set the day
    // If no day is set, all days will be run
    day: Option<usize>,
}

fn duration_to_string(duration: time::Duration) -> String {
    // Extract the number of microseconds
    let micros = duration.as_micros();

    // Find the ideal division factor and corresponding SI prefix
    let (factor, prefix) = match micros {
        10_000_000.. => (1_000_000, ' '),
        10_000.. => (1_000, 'm'),
        _ => (1, 'µ'),
    };

    // Rescale the value
    let value = micros / factor;

    // Print the rescaled value with the corresponding prefix
    format!("{value:4} {prefix}s")
}
