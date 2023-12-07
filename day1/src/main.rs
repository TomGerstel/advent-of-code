use std::{fs, str};

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let result1 = find_digits(&input, false);
    let result2 = find_digits(&input, true);

    println!("The result of part 1 is: {result1}");
    println!("The result of part 2 is: {result2}");
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

    #[test]
    fn test1() {
        let input = fs::read_to_string("./test1.txt").unwrap();
        assert_eq!(find_digits(&input, false), 142)
    }

    #[test]
    fn test2() {
        let input = fs::read_to_string("./test2.txt").unwrap();
        assert_eq!(find_digits(&input, true), 281)
    }
}
