use std::{fs, str};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let result1 = part1(&input);
    //let result2 = part2(&input);

    println!("The result of part 1 is: {result1}");
    //println!("The result of part 2 is: {result2}");
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(parse)
        .map(|(win, have)| score(win, have))
        .sum()
}

fn part2(input: &str) -> usize {
    todo!();
}

fn parse(line: &str) -> (Vec<usize>, Vec<usize>) {
    let (_, card) = line.split_once(": ").unwrap();
    let (win, have) = card.split_once(" | ").unwrap();
    let win = win.split_whitespace().map(|n| n.parse().unwrap()).collect();
    let have = have
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    (win, have)
}

fn score(win: Vec<usize>, have: Vec<usize>) -> usize {
    match have.iter().filter(|entry| win.contains(entry)).count() as u32 {
        0 => 0,
        n => 2_usize.pow(n - 1),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = fs::read_to_string("./test.txt").unwrap();
        assert_eq!(part1(&input), 13)
    }

    #[test]
    fn test2() {
        let input = fs::read_to_string("./test.txt").unwrap();
        assert_eq!(part2(&input), todo!())
    }
}
