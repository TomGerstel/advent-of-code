use std::{fs, str};

struct CubeGame {
    id: u32,
    cubes: Vec<[u32; 3]>,
}

pub fn run(input: &str) {
    let result1 = part1(&input);
    let result2 = part2(&input);

    println!("The result of part 1 is: {result1}");
    println!("The result of part 2 is: {result2}");
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| CubeGame::parse(line).unwrap())
        .filter(|cubegame| cubegame.is_possible([12, 13, 14]))
        .map(|cubegame| cubegame.id)
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| CubeGame::parse(line).unwrap())
        .map(|cubegame| cubegame.power())
        .sum()
}

impl CubeGame {
    fn parse(line: &str) -> Option<Self> {
        let (id, game) = line.trim_start_matches("Game ").split_once(": ")?;
        let id = id.parse().ok()?;

        // process game
        let cubes = game
            .split("; ")
            .map(|hand| {
                let mut colours = [0, 0, 0];
                hand.split(", ").for_each(|action| {
                    let (n, colour) = action.split_once(' ').unwrap();
                    let n = n.parse().unwrap();
                    let colour_id = match colour {
                        "red" => 0,
                        "green" => 1,
                        "blue" => 2,
                        _ => panic!(),
                    };
                    colours[colour_id] = n;
                });
                colours
            })
            .collect();

        Some(CubeGame { id, cubes })
    }

    fn is_possible(&self, colours: [u32; 3]) -> bool {
        self.cubes.iter().all(|&[red, green, blue]| {
            red <= colours[0] && green <= colours[1] && blue <= colours[2]
        })
    }

    fn power(&self) -> u32 {
        let minima = self.cubes.iter().fold([0, 0, 0], |minima, colours| {
            [
                minima[0].max(colours[0]),
                minima[1].max(colours[1]),
                minima[2].max(colours[2]),
            ]
        });
        minima[0] * minima[1] * minima[2]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = fs::read_to_string("./test/day02.txt").unwrap();
        assert_eq!(part1(&input), 8)
    }

    #[test]
    fn test2() {
        let input = fs::read_to_string("./test/day02.txt").unwrap();
        assert_eq!(part2(&input), 2286)
    }
}
