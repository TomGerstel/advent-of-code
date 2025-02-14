use crate::duration_to_string;
use crate::year2023;
use std::{fmt, fs, io, path, time};

#[derive(Copy, Clone)]
pub enum Part {
    One,
    Two,
}

pub struct Puzzle {
    year: usize,
    day: usize,
    part: Part,
}

pub enum PuzzleOutcome {
    Solved {
        solution: usize,
        duration: time::Duration,
    },
    Failed,
}

impl PuzzleOutcome {
    pub fn duration(&self) -> time::Duration {
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
        match self {
            Self::Solved { solution, duration } => {
                write!(f, "{:12} ({})", solution, duration_to_string(*duration))
            }
            Self::Failed => write!(f, "N/A"),
        }
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let part_str: &str = match self.part {
            Part::One => "a",
            Part::Two => "b",
        };
        write!(f, "{}/{:02}{}", self.year, self.day, part_str)
    }
}

impl Default for PuzzleOutcome {
    fn default() -> Self {
        Self::Failed
    }
}

impl Puzzle {
    pub fn new(year: usize, day: usize, part: Part) -> Self {
        Self { year, day, part }
    }

    pub fn load_and_solve(&self) -> Result<PuzzleOutcome, io::Error> {
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
