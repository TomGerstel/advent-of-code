use std::{fs, str};

const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = fs::read("./input.txt").unwrap();
    let result1 = part1(&input);
    let result2 = part2(&input);

    println!("The result of part 1 is: {result1}");
    println!("The result of part 2 is: {result2}");
}

fn part1(input: &[u8]) -> u32 {
    let chars = str::from_utf8(input).unwrap();

    chars
        .split('\n')
        .map(|line| {
            let mut nums = line.chars().filter_map(|char| char.to_digit(10));

            let first = nums.next().unwrap();
            let last = match nums.last() {
                Some(last) => last,
                None => first,
            };

            10 * first + last
        })
        .sum()
}

fn part2(input: &[u8]) -> u32 {
    let chars = str::from_utf8(input).unwrap();
    chars
        .split('\n')
        .map(|line| {
            let first = find_first(line).unwrap();
            let last = find_last(line).unwrap();

            10 * first + last
        })
        .sum()
}

fn find_first(line: &str) -> Option<u32> {
    for i in 0..(line.len()) {
        let digit = line.chars().nth(i).unwrap().to_digit(10);
        if digit.is_some() {
            return digit;
        }

        for (n, name) in DIGITS.iter().enumerate() {
            let scope = line.get(i..(i + name.len()));
            if scope == Some(name) {
                return Some(n as u32);
            }
        }
    }
    None
}

fn find_last(line: &str) -> Option<u32> {
    for i in 0..(line.len()) {
        let digit = line.chars().nth(line.len() - i - 1).unwrap().to_digit(10);
        if digit.is_some() {
            return digit;
        }

        for (n, name) in DIGITS.iter().enumerate() {
            let scope = line.get((line.len() - i - name.len())..(line.len() - i));
            if scope == Some(name) {
                return Some(n as u32);
            }
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = fs::read("./test1.txt").unwrap();
        assert_eq!(part1(&input), 142)
    }

    #[test]
    fn test2() {
        let input = fs::read("./test2.txt").unwrap();
        assert_eq!(part2(&input), 281)
    }
}
