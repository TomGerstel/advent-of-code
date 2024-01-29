use std::{cmp, str};

const CARD_ORDER: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const JOKER_CARD_ORDER: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Hand {
    strengths: [usize; 5],
    hand_type: HandType,
}

pub fn part1(input: &str) -> usize {
    let hands = input
        .lines()
        .map(|line| parse(line, false))
        .collect::<Vec<_>>();
    calculate_score(hands)
}

pub fn part2(input: &str) -> usize {
    let hands = input
        .lines()
        .map(|line| parse(line, true))
        .collect::<Vec<_>>();
    calculate_score(hands)
}

fn calculate_score(mut hands: Vec<(Hand, usize)>) -> usize {
    hands.sort_by(|(hand_a, _), (hand_b, _)| hand_a.cmp(hand_b));

    hands
        .iter()
        .map(|(_, bid)| bid)
        .enumerate()
        .map(|(rankm1, bid)| (rankm1 + 1) * bid)
        .sum()
}

fn parse(line: &str, jokers: bool) -> (Hand, usize) {
    let (hand, bid) = line.split_once(' ').unwrap();
    let hand = process_hand(hand, jokers);

    let bid = bid.parse().unwrap();
    (hand, bid)
}

fn process_hand(hand: &str, jokers: bool) -> Hand {
    let hand_type = hand_type(hand);
    let strengths = strengths(hand, jokers);
    let hand_type = {
        if jokers {
            let joker_count = count_jokers(hand);
            map_hand_type(hand_type, joker_count)
        } else {
            hand_type
        }
    };
    Hand {
        strengths,
        hand_type,
    }
}

fn map_hand_type(hand_type: HandType, joker_count: usize) -> HandType {
    match (hand_type, joker_count) {
        (_, 0) => hand_type,
        (HandType::HighCard, 1) => HandType::OnePair,
        (HandType::OnePair, 1 | 2) => HandType::ThreeOfAKind,
        (HandType::TwoPair, 1) => HandType::FullHouse,
        (HandType::TwoPair, 2) => HandType::FourOfAKind,
        (HandType::ThreeOfAKind, 1 | 3) => HandType::FourOfAKind,
        (HandType::FullHouse, 2 | 3) => HandType::FiveOfAKind,
        (HandType::FourOfAKind, 1 | 4) => HandType::FiveOfAKind,
        (HandType::FiveOfAKind, 5) => HandType::FiveOfAKind,
        _ => unreachable!(),
    }
}

fn count_jokers(hand: &str) -> usize {
    hand.chars().filter(|char| *char == 'J').count()
}

fn hand_type(hand: &str) -> HandType {
    let mut seen = Vec::new();
    let mut dup_count = [0, 0, 0, 0, 0];
    for card in hand.chars() {
        if let Some(i) = seen.iter().position(|&x| x == card) {
            dup_count[i] += 1;
        } else {
            dup_count[seen.len()] = 1;
            seen.push(card);
        }
    }
    dup_count.sort();

    match dup_count {
        [0, 0, 0, 0, 5] => HandType::FiveOfAKind,
        [0, 0, 0, 1, 4] => HandType::FourOfAKind,
        [0, 0, 0, 2, 3] => HandType::FullHouse,
        [0, 0, 1, 1, 3] => HandType::ThreeOfAKind,
        [0, 0, 1, 2, 2] => HandType::TwoPair,
        [0, 1, 1, 1, 2] => HandType::OnePair,
        [1, 1, 1, 1, 1] => HandType::HighCard,
        _ => unreachable!(),
    }
}

fn strengths(hand: &str, jokers: bool) -> [usize; 5] {
    hand.chars()
        .map(|card| {
            {
                if jokers {
                    JOKER_CARD_ORDER
                } else {
                    CARD_ORDER
                }
            }
            .iter()
            .position(|&x| x == card)
            .unwrap()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

impl cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for Hand {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self.hand_type).cmp(&(other.hand_type)) {
            cmp::Ordering::Equal => self.strengths.cmp(&other.strengths),
            cmp => cmp,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn test1() {
        let input = fs::read_to_string("./test/day07.txt").unwrap();
        assert_eq!(part1(&input), 6440)
    }

    #[test]
    fn test2() {
        let input = fs::read_to_string("./test/day07.txt").unwrap();
        assert_eq!(part2(&input), 5905)
    }
}
