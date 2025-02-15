use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    let (seeds, maps) = parse(input);
    seeds
        .map(|seed| {
            maps.iter()
                .fold(seed, |source, map| lookup_single(source, &map[..]))
        })
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> usize {
    let (seeds, maps) = parse(input);

    maps.iter()
        .fold(
            seeds
                .tuples()
                .map(|(start, len)| SeedRange { start, len })
                .collect::<Vec<_>>(),
            |seed_ranges, map| lookup_range(seed_ranges, map),
        )
        .iter()
        .map(|seed_range| seed_range.start)
        .min()
        .unwrap()
}

fn parse(input: &str) -> (impl Iterator<Item = usize> + '_, [Vec<MapSection>; 7]) {
    // Split the input into blocks, one for the seeds + seven for the maps
    let mut blocks = input.split("\n\n").flat_map(|s| s.split("\r\n\r\n"));

    // Extract the one seeds block
    let seeds = blocks
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|seed| seed.parse().unwrap());

    // Extract the seven map blocks
    let maps: [_; 7] = blocks
        .map(|block| {
            // Parse a single map block
            block
                .lines()
                .skip(1)
                .map(|line| {
                    // Parse a single line of a map into a MapSection
                    let [dest, source, len]: [usize; 3] = line
                        .split_whitespace()
                        .map(|num| num.parse().unwrap())
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap();
                    MapSection { source, dest, len }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    (seeds, maps)
}

#[derive(Debug)]
struct MapSection {
    source: usize,
    dest: usize,
    len: usize,
}

#[derive(Copy, Clone, Debug)]
struct SeedRange {
    start: usize,
    len: usize,
}

fn lookup_single(value: usize, map: &[MapSection]) -> usize {
    map.iter()
        .find_map(|map_section| {
            if value >= map_section.source && value <= map_section.source + map_section.len {
                Some(value - map_section.source + map_section.dest)
            } else {
                None
            }
        })
        .unwrap_or(value)
}

fn lookup_range(mut queue: Vec<SeedRange>, map: &[MapSection]) -> Vec<SeedRange> {
    let mut output = vec![];
    'queue: while let Some(seed_range) = queue.pop() {
        for map_section in map.iter() {
            if let Some(intersection) = seed_range.intersect(map_section) {
                queue.extend(intersection.left);
                output.extend(intersection.mid);
                queue.extend(intersection.right);
                continue 'queue;
            }
            output.push(seed_range);
        }
    }
    output
}

impl SeedRange {
    fn intersect(&self, map_section: &MapSection) -> Option<Intersection> {
        // Check if there is any intersection at all
        if self.start + self.len < map_section.source
            || map_section.source + map_section.len < self.start
        {
            return None;
        }

        // Find the left part
        let left = if self.start < map_section.source && map_section.source < self.start + self.len
        {
            Some(SeedRange {
                start: self.start,
                len: map_section.source - self.start,
            })
        } else {
            None
        };

        // Find the start of the middle part
        let start = if self.start <= map_section.source {
            
        }

        // Find the middle part
        let mid = if self.start < map_section.source && map_section.source + map_section.len < self.start + self.len {
            Some(SeedRange {
                start: map_section.source,
                len: map_section.len
            });
        } else if self.start < map_section.source && self.start + self.len < map_section.source + map_section.len {

        }

        // Find the right part
        let right = if map_section.source < self.start
            && self.start < map_section.source + map_section.len
        {
            Some(SeedRange {
                start: map_section.source + map_section.len,
                len: map_section.source + map_section.len - (self.start + self.len),
            })
        } else {
            None
        };

        todo!();
    }
}

struct Intersection {
    left: Option<SeedRange>,
    mid: Option<SeedRange>,
    right: Option<SeedRange>,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, path::PathBuf};

    #[test]
    fn test1() {
        let path: PathBuf = [".", "input", "test", "2023", "day05.txt"].iter().collect();
        let input = fs::read_to_string(&path).unwrap();
        assert_eq!(part1(&input), 35)
    }

    #[test]
    fn test2() {
        let path: PathBuf = [".", "input", "test", "2023", "day05.txt"].iter().collect();
        let input = fs::read_to_string(&path).unwrap();
        assert_eq!(part2(&input), 46)
    }
}
