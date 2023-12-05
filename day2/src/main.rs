use std::{fs, str};

struct CubeGame {
    id: u32,
    cubes: Vec<[usize; 3]>,
}

fn main() {
    let input = fs::read("./input.txt").unwrap();
    let result1 = part1(&input);
    //let result2 = part2(&input);

    println!("The result of part 1 is: {result1}");
    //println!("The result of part 2 is: {result2}");
}

fn part1(input: &[u8]) -> u32 {
    let chars = str::from_utf8(input).unwrap();

    chars
        .split('\n')
        .map(CubeGame::parse)
        .filter(|cubegame| cubegame.is_possible([12, 13, 14]))
        .map(|cubegame| cubegame.id)
        .sum()
}

impl CubeGame {
    fn parse(input: &str) -> Self {
        let (header, data) = input.split_once(": ").unwrap();

        // process header
        let (title, id) = header.split_once(' ').unwrap();
        assert_eq!(title, "Game");
        let id = id.parse::<u32>().unwrap();

        // process data
        let cubes = data
            .split("; ")
            .map(|hand| {
                let mut colours = [0, 0, 0];
                hand.split(", ").for_each(|colour_amount| {
                    let (amount, colour) = colour_amount.split_once(' ').unwrap();
                    let amount = amount.parse::<usize>().unwrap();
                    let colour_id = match colour {
                        "red" => 0,
                        "green" => 1,
                        "blue" => 2,
                        _ => panic!(),
                    };
                    colours[colour_id] = amount;
                });
                colours
            })
            .collect();

        CubeGame { id, cubes }
    }

    fn is_possible(&self, colours: [usize; 3]) -> bool {
        self.cubes.iter().all(|&[red, green, blue]| {
            red <= colours[0] && green <= colours[1] && blue <= colours[2]
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = fs::read("./test.txt").unwrap();
        assert_eq!(part1(&input), 8)
    }
}
