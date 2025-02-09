use clap::Parser;
use std::{fmt, fs, io, path, time};

mod year2023;

fn duration_to_string(duration: time::Duration) -> String {
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

#[derive(Copy, Clone)]
enum Part {
    One,
    Two,
}

struct Puzzle {
    year: usize,
    day: usize,
    part: Part,
}

enum PuzzleOutcome {
    Solved {
        solution: usize,
        duration: time::Duration,
    },
    Failed,
}

impl PuzzleOutcome {
    fn duration(&self) -> time::Duration {
        match self {
            Self::Solved {
                solution: _,
                duration,
            } => *duration,
            Self::Failed => time::Duration::default(),
        }
    }
}

impl fmt::Display for PuzzleOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write preceding whitespace
        write!(f, "\n\t")?;

        // Write actual content
        match self {
            Self::Solved { solution, duration } => {
                write!(f, "{} ({})", solution, duration_to_string(*duration))?
            }
            Self::Failed => write!(f, "No solution found!")?,
        };

        // Write trailing whitespace
        writeln!(f)
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let part_str: &str = match self.part {
            Part::One => "One",
            Part::Two => "Two",
        };
        write!(f, "{} - Day {:02} - Part {}", self.year, self.day, part_str)
    }
}

impl Puzzle {
    fn load_and_solve(&self) -> Result<PuzzleOutcome, io::Error> {
        // Try to load input
        let input = self.load_input()?;

        // Trim trailing whitespace for easy splitting into lines
        let input = input.trim_end();

        // Actually solve the puzzle
        Ok(self.solve(input))
    }

    fn load_input(&self) -> Result<std::string::String, io::Error> {
        // Set the path for the given day
        let path_buf: path::PathBuf = [
            ".",
            "input",
            "data",
            self.year.to_string().as_str(),
            format!("day{:02}.txt", self.day).as_str(),
        ]
        .iter()
        .collect();

        // Try to read the input
        let input = fs::read_to_string(&path_buf)?;
        Ok(input)
    }

    fn solve(&self, input: &str) -> PuzzleOutcome {
        match self.retrieve_solver() {
            Some(solver) => {
                // Solve the puzzle and keep track of the time spent doing so
                let start_instant = time::Instant::now();
                let solution = solver(input);
                let duration = start_instant.elapsed();
                PuzzleOutcome::Solved { solution, duration }
            }
            None => PuzzleOutcome::Failed,
        }
    }

    fn retrieve_solver(&self) -> Option<fn(&str) -> usize> {
        match (self.year, self.day, self.part) {
            (2023, 1, Part::One) => Some(year2023::day01::part1),
            (2023, 1, Part::Two) => Some(year2023::day01::part2),
            (2023, 2, Part::One) => Some(year2023::day02::part1),
            (2023, 2, Part::Two) => Some(year2023::day02::part2),
            (2023, 3, Part::One) => Some(year2023::day03::part1),
            (2023, 3, Part::Two) => Some(year2023::day03::part2),
            (2023, 4, Part::One) => Some(year2023::day04::part1),
            (2023, 4, Part::Two) => Some(year2023::day04::part2),
            //(2023, 5, Part::One) => Some(year2023::day05::part1),
            //(2023, 5, Part::Two) => Some(year2023::day05::part2),
            (2023, 6, Part::One) => Some(year2023::day06::part1),
            (2023, 6, Part::Two) => Some(year2023::day06::part2),
            (2023, 7, Part::One) => Some(year2023::day07::part1),
            (2023, 7, Part::Two) => Some(year2023::day07::part2),
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
    let mut total_duration = time::Duration::default();

    // Solve both puzzles for each day
    for day in days.into_iter() {
        for part in [Part::One, Part::Two] {
            // Initialise the puzzle and solve it
            let puzzle = Puzzle { year, day, part };
            let outcome = puzzle.load_and_solve().unwrap();

            // Do some logging
            println!("{puzzle}");
            println!("{outcome}");
            println!();

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
