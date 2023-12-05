use std::{fs, str};

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let result1 = part1(&input);
    let result2 = part2(&input);

    println!("The result of part 1 is: {result1}");
    println!("The result of part 2 is: {result2}");
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|char| char.to_digit(10));

            let first = digits.next().unwrap_or(0);
            let last = digits.last().unwrap_or(first);

            10 * first + last
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().enumerate().filter_map(|(i, char)| {
                char.to_digit(10).or({
                    DIGITS
                        .iter()
                        .enumerate()
                        .find(|(_, name)| line[i..].starts_with(**name))
                        .map(|(n, _)| (n + 1) as u32)
                })
            });

            let first = digits.next().unwrap_or(0);
            let last = digits.last().unwrap_or(first);

            10 * first + last
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = fs::read_to_string("./test1.txt").unwrap();
        assert_eq!(part1(&input), 142)
    }

    #[test]
    fn test2() {
        let input = fs::read_to_string("./test2.txt").unwrap();
        assert_eq!(part2(&input), 281)
    }
}
