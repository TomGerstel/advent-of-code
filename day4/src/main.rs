use std::{fs, str};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let result1 = part1(&input);
    let result2 = part2(&input);

    println!("The result of part 1 is: {result1}");
    println!("The result of part 2 is: {result2}");
}

fn part1(input: &str) -> u32 {
    todo!();
}

fn part2(input: &str) -> u32 {
    todo!();
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
