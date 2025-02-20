use std::str;

pub fn part1(input: &str) -> usize {
    let input = parse1(input);
    input.map(|(t, s)| count_wins(t, s)).product()
}

pub fn part2(input: &str) -> usize {
    let (t, s) = parse2(input);
    count_wins(t, s)
}

fn parse1(input: &str) -> impl Iterator<Item = (usize, usize)> + '_ {
    let (times, distances) = input.split_once('\n').unwrap();
    let times = times
        .trim_start_matches("Time:")
        .split_whitespace()
        .map(|t| t.parse().unwrap());
    let distances = distances
        .trim_start_matches("Distance:")
        .split_whitespace()
        .map(|d| d.parse().unwrap());
    times.zip(distances)
}

fn parse2(input: &str) -> (usize, usize) {
    let (time, dist) = input.split_once('\n').unwrap();
    let mut time = time.to_string();
    let mut dist = dist.to_string();
    time.retain(|c| c.is_ascii_digit());
    dist.retain(|c| c.is_ascii_digit());
    let time = time.parse().unwrap();
    let dist = dist.parse().unwrap();
    (time, dist)
}

fn count_wins(t: usize, s: usize) -> usize {
    (0..=t).filter(|i| i * (t - i) > s).count()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, path::PathBuf};

    #[test]
    fn test1() {
        let path: PathBuf = [".", "input", "test", "2023", "day06.txt"].iter().collect();
        let input = fs::read_to_string(&path).unwrap();
        assert_eq!(part1(&input), 288)
    }

    #[test]
    fn test2() {
        let path: PathBuf = [".", "input", "test", "2023", "day06.txt"].iter().collect();
        let input = fs::read_to_string(&path).unwrap();
        assert_eq!(part2(&input), 71503)
    }
}
