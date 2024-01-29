use std::{collections::VecDeque, fs, str};

pub fn run(input: &str) {
    let result1 = part1(&input);
    let result2 = part2(&input);

    println!("The result of part 1 is: {result1}");
    println!("The result of part 2 is: {result2}");
}

fn part1(input: &str) -> usize {
    input.lines().map(wins).map(score).sum()
}

fn part2(input: &str) -> usize {
    let mut buf = VecDeque::new();
    input.lines().map(wins).fold(0, |counter, wins| {
        let cards = 1 + buf.pop_front().unwrap_or(0);
        (0..wins).for_each(|i| match buf.get_mut(i) {
            Some(n) => *n += cards,
            None => buf.push_back(cards),
        });
        counter + cards
    })
}

fn wins(line: &str) -> usize {
    let (_, card) = line.split_once(": ").unwrap();
    let (win, have) = card.split_once(" | ").unwrap();
    let win = win
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    have.split_whitespace()
        .map(|n| n.parse().unwrap())
        .filter(|entry| win.contains(entry))
        .count()
}

fn score(wins: usize) -> usize {
    match wins as u32 {
        0 => 0,
        n => 2_usize.pow(n - 1),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = fs::read_to_string("./test/day04.txt").unwrap();
        assert_eq!(part1(&input), 13)
    }

    #[test]
    fn test2() {
        let input = fs::read_to_string("./test/day04.txt").unwrap();
        assert_eq!(part2(&input), 30)
    }
}
