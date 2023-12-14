// --- Day 7: Camel Cards ---
use std::env;
use std::fs;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum Hand {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseHandError;

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut m = HashMap::new();
        for c in s.chars() {
            let count = m.entry(c).or_insert(0);
            *count += 1;
        }
        match m.len() {
            1 => Ok(Self::FiveOfAKind),
            2 => match m.values().max().ok_or(ParseGameError) {
                Ok(amount) => match amount {
                    4 => Ok(Self::FourOfAKind),
                    3 => Ok(Self::FullHouse),
                    _ => unreachable!(),
                },
                Err(_) => Err(ParseHandError),
            },
            3 => match m.values().max().ok_or(ParseHandError)? {
                3 => Ok(Self::ThreeOfAKind),
                2 => Ok(Self::TwoPair),
                _ => Err(ParseHandError),
            },
            4 => Ok(Self::OnePair),
            5 => Ok(Self::HighCard),
            _ => Err(ParseHandError),
        }
    }
}

#[derive(Debug, Eq)]
struct Game {
    hand: Hand,
    hand_str: String,
    bid: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand_str, bid_str) = s.split_once(" ").ok_or(ParseGameError)?;
        let bid = bid_str.parse().map_err(|_| ParseGameError)?;
        let hand = Hand::from_str(hand_str).map_err(|_| ParseGameError)?;
        Ok(Self {
            hand,
            hand_str: hand_str.to_owned(),
            bid,
        })
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!()
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

fn puzzle1(data: &str) -> i32 {
    data.lines().map(|line| {
        let (hand_str, bid_str) = line.split_once(" ").unwrap();
        let bid: u32 = bid_str.parse().unwrap();
        let hand = Hand::from_str(hand_str).unwrap();
    });
    0
}

fn puzzle2(data: &str) -> i32 {
    data.len() as i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <path_to_puzzle_input>", args[0]);
        std::process::exit(-1);
    }
    let puzzle_input = fs::read_to_string(&args[1]).unwrap();

    let p1_out = puzzle1(&puzzle_input);
    println!("Answer puzzle 1: {}", p1_out);

    let p2_out = puzzle2(&puzzle_input);
    println!("Answer puzzle 2: {}", p2_out);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        let test_data = "\
                         32T3K 765\n\
                         T55J5 684\n\
                         KK677 28\n\
                         KTJJT 220\n\
                         QQQJA 483";
        let res = puzzle1(test_data);
        assert_eq!(res, 6440);
    }

    #[test]
    fn test_puzzle2() {
        let test_data = "";
        let res = puzzle2(test_data);
        assert_eq!(res, -1);
    }

    #[test]
    fn test_parse_hand() {
        assert_eq!(Hand::from_str("AAAAA"), Ok(Hand::FiveOfAKind));
        assert_eq!(Hand::from_str("88888"), Ok(Hand::FiveOfAKind));

        assert_eq!(Hand::from_str("18888"), Ok(Hand::FourOfAKind));
        assert_eq!(Hand::from_str("1KKKK"), Ok(Hand::FourOfAKind));
        assert_eq!(Hand::from_str("KK5KK"), Ok(Hand::FourOfAKind));

        assert_eq!(Hand::from_str("K55KK"), Ok(Hand::FullHouse));
        assert_eq!(Hand::from_str("83838"), Ok(Hand::FullHouse));

        assert_eq!(Hand::from_str("1K1K4"), Ok(Hand::TwoPair));
        assert_eq!(Hand::from_str("AA5TT"), Ok(Hand::TwoPair));

        assert_eq!(Hand::from_str("AA54T"), Ok(Hand::OnePair));
        assert_eq!(Hand::from_str("T354T"), Ok(Hand::OnePair));

        assert_eq!(Hand::from_str("12345"), Ok(Hand::HighCard));
    }

    #[test]
    fn test_parse_game() {
        assert_eq!(
            Game::from_str("AAAAA 123"),
            Ok(Game {
                hand: Hand::FiveOfAKind,
                hand_str: "AAAAA".to_owned(),
                bid: 123
            })
        );
        assert_eq!(
            Game::from_str("88888 234"),
            Ok(Game {
                hand: Hand::FiveOfAKind,
                hand_str: "88888".to_owned(),
                bid: 234
            })
        );

        assert_eq!(
            Game::from_str("18888 3"),
            Ok(Game {
                hand: Hand::FourOfAKind,
                hand_str: "18888".to_owned(),
                bid: 3
            })
        );
        assert_eq!(
            Game::from_str("1KKKK 4567"),
            Ok(Game {
                hand: Hand::FourOfAKind,
                hand_str: "1KKKK".to_owned(),
                bid: 4567
            })
        );

        assert_eq!(
            Game::from_str("K55KK 34"),
            Ok(Game {
                hand: Hand::FullHouse,
                hand_str: "K55KK".to_owned(),
                bid: 34
            })
        );
        assert_eq!(
            Game::from_str("1K1K4 24"),
            Ok(Game {
                hand: Hand::TwoPair,
                hand_str: "1K1K4".to_owned(),
                bid: 24
            })
        );
        assert_eq!(
            Game::from_str("AA54T 2"),
            Ok(Game {
                hand: Hand::OnePair,
                hand_str: "AA54T".to_owned(),
                bid: 2
            })
        );
        assert_eq!(
            Game::from_str("12345 93"),
            Ok(Game {
                hand: Hand::HighCard,
                hand_str: "12345".to_owned(),
                bid: 93
            })
        );
    }
}
