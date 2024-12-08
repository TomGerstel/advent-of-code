use std::str;

#[derive(Debug)]
struct Schematic {
    entries: Vec<Vec<Entry>>,
}

#[derive(Debug, Copy, Clone)]
enum Entry {
    Number { value: u32 },
    Gear,
    Symbol,
    Empty,
}

pub fn part1(input: &str) -> usize {
    Schematic::parse(input)
        .parts()
        .iter()
        .map(|(number, _)| number)
        .sum::<u32>() as usize
}

pub fn part2(input: &str) -> usize {
    // generate list of parts and gears
    let parts = Schematic::parse(input).parts();
    let gears = {
        let mut gears = parts
            .iter()
            .filter_map(|(_, gear)| *gear)
            .collect::<Vec<_>>();
        gears.sort();
        gears.dedup();
        gears
    };

    // iterate through the gears and calculate each gear ratio
    gears
        .iter()
        .map(|gear| {
            let gear_parts = parts
                .iter()
                .filter(|(_, part_gear)| *part_gear == Some(*gear))
                .map(|(number, _)| number)
                .collect::<Vec<_>>();
            if gear_parts.len() == 2 {
                gear_parts[0] * gear_parts[1]
            } else {
                0
            }
        })
        .sum::<u32>() as usize
}

impl Schematic {
    fn parse(input: &str) -> Self {
        let entries = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|entry| match entry {
                        '.' => Entry::Empty,
                        '*' => Entry::Gear,
                        '0'..='9' => Entry::Number {
                            value: entry.to_digit(10).unwrap(),
                        },
                        _ => Entry::Symbol,
                    })
                    .collect()
            })
            .collect();
        Schematic { entries }
    }

    fn nbrs(&self, x: usize, y: usize, dx: usize) -> Vec<(Entry, usize, usize)> {
        // generate neighbour offset coordinates
        let offsets: Vec<(isize, isize)> = {
            let mut offsets = vec![(-1, -1), (-1, 0), (-1, 1)];
            for i in 0..dx {
                offsets.append(&mut vec![(i as isize, -1), (i as isize, 1)]);
            }
            offsets.append(&mut vec![
                (dx as isize, -1),
                (dx as isize, 0),
                (dx as isize, 1),
            ]);
            offsets
        };

        // save neighbour entries if they exist
        let mut entries = Vec::new();
        for (dx, dy) in offsets {
            let x_nbr = x as isize + dx;
            let y_nbr = y as isize + dy;

            // convert coordinates to usize if possible
            if x_nbr >= 0 && y_nbr >= 0 {
                let x_nbr = x_nbr as usize;
                let y_nbr = y_nbr as usize;

                // save neighbours if they exist
                if let Some(line) = self.entries.get(y_nbr) {
                    if let Some(entry) = line.get(x_nbr) {
                        entries.push((*entry, x_nbr, y_nbr));
                    }
                }
            }
        }
        entries
    }

    fn parts(&self) -> Vec<(u32, Option<(usize, usize)>)> {
        let mut numbers = Vec::new();
        for y in 0..(self.entries.len()) {
            let mut skip = 0;
            for x in 0..(self.entries[y].len()) {
                // skip if we are halfway through a number
                if skip > 0 {
                    skip -= 1;
                } else {
                    let (number, dx) = self.construct_number(x, y);

                    // number = 0 means it is not actually a number
                    if number > 0 {
                        let nbrs = self.nbrs(x, y, dx);
                        if !nbrs.iter().all(|(nbr, _, _)| !matches!(nbr, Entry::Symbol)) {
                            numbers.push((number, None));
                        } else {
                            nbrs.iter().for_each(|(nbr, x_nbr, y_nbr)| {
                                if matches!(nbr, Entry::Gear) {
                                    numbers.push((number, Some((*x_nbr, *y_nbr))))
                                }
                            })
                        }
                    }
                    skip = dx;
                }
            }
        }
        numbers
    }

    fn construct_number(&self, x: usize, y: usize) -> (u32, usize) {
        let mut number = 0;
        let mut dx = 0;
        while let Entry::Number { value } = {
            if let Some(entry) = self.entries[y].get(x + dx) {
                entry
            } else {
                return (number, dx);
            }
        } {
            number = 10 * number + value;
            dx += 1;
        }
        (number, dx)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, path::PathBuf};

    #[test]
    fn test1() {
        let path: PathBuf = [".", "input", "test", "2023", "day03.txt"].iter().collect();
        let input = fs::read_to_string(&path).unwrap();
        assert_eq!(part1(&input), 4361)
    }

    #[test]
    fn test2() {
        let path: PathBuf = [".", "input", "test", "2023", "day03.txt"].iter().collect();
        let input = fs::read_to_string(&path).unwrap();
        assert_eq!(part2(&input), 467835)
    }
}
