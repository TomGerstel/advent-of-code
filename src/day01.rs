use std::str;

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part1(input: &str) -> usize {
    find_digits(input, false)
}

pub fn part2(input: &str) -> usize {
    find_digits(input, true)
}

fn find_digits(input: &str, part_two: bool) -> usize {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().enumerate().filter_map(|(i, char)| match char {
                '0'..='9' => char.to_digit(10).map(|n| n as usize),
                _ if part_two => DIGITS
                    .iter()
                    .enumerate()
                    .find_map(|(n, name)| line[i..].starts_with(*name).then_some(n + 1)),
                _ => None,
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
    use std::fs;

    #[test]
    fn test1() {
        let input = fs::read_to_string("./test/day01a.txt").unwrap();
        assert_eq!(find_digits(&input, false), 142)
    }

    #[test]
    fn test2() {
        let input = fs::read_to_string("./test/day01b.txt").unwrap();
        assert_eq!(find_digits(&input, true), 281)
    }
}
