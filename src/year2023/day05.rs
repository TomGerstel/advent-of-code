use itertools::Itertools;
use std::str;

pub fn part1(input: &str) -> usize {
    let (seeds, maps) = parse(input);
    seeds
        .map(|seed| {
            maps.iter()
                .fold(seed, |source, map| lookup(source, &map[..]))
        })
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> usize {
    let (seeds, maps) = parse(input);
    seeds
        .tuples()
        .flat_map(|(start, len)| start..(start + len))
        .map(|seed| {
            maps.iter()
                .fold(seed, |source, map| lookup(source, &map[..]))
        })
        .min()
        .unwrap()
}

fn parse(input: &str) -> (impl Iterator<Item = usize> + '_, [Vec<[usize; 3]>; 7]) {
    let mut blocks = input.split("\n\n").flat_map(|s| s.split("\r\n\r\n"));
    let seeds = blocks
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|seed| seed.parse().unwrap());

    let maps: [_; 7] = blocks
        .map(|block| {
            block
                .lines()
                .skip(1)
                .map(|line| {
                    let map_line: [usize; 3] = line
                        .split_whitespace()
                        .map(|num| num.parse().unwrap())
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap();
                    map_line
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    (seeds, maps)
}

fn lookup(source: usize, map: &[[usize; 3]]) -> usize {
    map.iter()
        .find_map(|&[dest_start, source_start, len]| {
            if source >= source_start && source <= source_start + len {
                Some(source - source_start + dest_start)
            } else {
                None
            }
        })
        .unwrap_or(source)
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
