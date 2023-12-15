// --- Day 7: Camel Cards ---
use std::env;
use std::fs;

use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum Hand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseHandError;

impl Hand {
    fn from_str(s: &str, with_joker: bool) -> Result<Self, ParseHandError> {
        if s.len() != 5 {
            return Err(ParseHandError);
        }
        let mut m = HashMap::new();
        for c in s.chars() {
            let count = m.entry(c).or_insert(0);
            *count += 1;
        }
        let joker_count = if with_joker {
            *m.get(&'J').unwrap_or(&0)
        } else {
            0
        };
        match m.len() {
            1 => Ok(Self::FiveOfAKind),
            2 => match m.values().max().ok_or(ParseGameError) {
                Ok(amount) => match amount {
                    4 => match joker_count {
                        0 => Ok(Self::FourOfAKind),
                        1 | 4 => Ok(Self::FiveOfAKind),
                        _ => unreachable!(),
                    },
                    3 => match joker_count {
                        0 => Ok(Self::FullHouse),
                        2 | 3 => Ok(Self::FiveOfAKind),
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                },
                Err(_) => unreachable!(),
            },
            3 => match m.values().max().ok_or(ParseHandError)? {
                3 => match joker_count {
                    0 => Ok(Self::ThreeOfAKind),
                    1 | 3 => Ok(Self::FourOfAKind),
                    _ => unreachable!(),
                },
                2 => match joker_count {
                    0 => Ok(Self::TwoPair),
                    1 => Ok(Self::FullHouse),
                    2 => Ok(Self::FourOfAKind),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            4 => match joker_count {
                0 => Ok(Self::OnePair),
                1 | 2 => Ok(Self::ThreeOfAKind),
                _ => unreachable!(),
            },
            5 => match joker_count {
                0 => Ok(Self::HighCard),
                1 => Ok(Self::OnePair),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

fn card_value(c: &char, with_joker: bool) -> Option<u8> {
    match c.to_digit(10) {
        Some(d) => {
            if d >= 2 {
                Some(d as u8)
            } else {
                None
            }
        }
        None => match c {
            'T' => Some(10),
            'J' => {
                if with_joker {
                    Some(1)
                } else {
                    Some(11)
                }
            }
            'Q' => Some(12),
            'K' => Some(13),
            'A' => Some(14),
            _ => None,
        },
    }
}

#[derive(Debug, Eq)]
struct Game {
    hand: Hand,
    hand_str: String,
    bid: u32,
    with_joker: bool,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

impl Game {
    fn from_str(s: &str, with_joker: bool) -> Result<Self, ParseGameError> {
        let (hand_str, bid_str) = s.split_once(" ").ok_or(ParseGameError)?;
        let bid = bid_str.parse().map_err(|_| ParseGameError)?;
        let hand = Hand::from_str(hand_str, with_joker).map_err(|_| ParseGameError)?;
        Ok(Self {
            hand,
            hand_str: hand_str.to_owned(),
            bid,
            with_joker,
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
        match self.hand.cmp(&other.hand) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                let zipped = Iterator::zip(self.hand_str.chars(), other.hand_str.chars());
                for (s, o) in zipped {
                    let s_val = card_value(&s, self.with_joker).unwrap();
                    let o_val = card_value(&o, self.with_joker).unwrap();
                    match s_val.cmp(&o_val) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => continue,
                    }
                }
                Ordering::Equal
            }
        }
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(&other) == Ordering::Equal
    }
}

fn puzzle1(data: &str) -> i32 {
    let mut games = data
        .lines()
        .map(|line| Game::from_str(line, false))
        .collect::<Result<Vec<Game>, ParseGameError>>()
        .unwrap();
    games.sort();
    let res: u32 = games
        .iter()
        .enumerate()
        .map(|(i, game)| (i as u32 + 1) * game.bid)
        .sum();
    res as i32
}

fn puzzle2(data: &str) -> i32 {
    let mut games = data
        .lines()
        .map(|line| Game::from_str(line, true))
        .collect::<Result<Vec<Game>, ParseGameError>>()
        .unwrap();
    games.sort();
    let res: u32 = games
        .iter()
        .enumerate()
        .map(|(i, game)| (i as u32 + 1) * game.bid)
        .sum();
    res as i32
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
        let test_data = "\
                         32T3K 765\n\
                         T55J5 684\n\
                         KK677 28\n\
                         KTJJT 220\n\
                         QQQJA 483";
        let res = puzzle2(test_data);
        assert_eq!(res, 5905);
    }

    #[test]
    fn test_parse_hand_without_joker() {
        assert_eq!(Hand::from_str("AAAAA", false), Ok(Hand::FiveOfAKind));
        assert_eq!(Hand::from_str("88888", false), Ok(Hand::FiveOfAKind));

        assert_eq!(Hand::from_str("Q8888", false), Ok(Hand::FourOfAKind));
        assert_eq!(Hand::from_str("JKKKK", false), Ok(Hand::FourOfAKind));
        assert_eq!(Hand::from_str("KK5KK", false), Ok(Hand::FourOfAKind));

        assert_eq!(Hand::from_str("K55KK", false), Ok(Hand::FullHouse));
        assert_eq!(Hand::from_str("83838", false), Ok(Hand::FullHouse));

        assert_eq!(Hand::from_str("3K3K4", false), Ok(Hand::TwoPair));
        assert_eq!(Hand::from_str("AA5TT", false), Ok(Hand::TwoPair));

        assert_eq!(Hand::from_str("AA54T", false), Ok(Hand::OnePair));
        assert_eq!(Hand::from_str("T354T", false), Ok(Hand::OnePair));

        assert_eq!(Hand::from_str("23456", false), Ok(Hand::HighCard));

        // Parsing error
        assert_eq!(Hand::from_str("2345", false), Err(ParseHandError));
        assert_eq!(Hand::from_str("234567", false), Err(ParseHandError));
    }

    #[test]
    fn test_parse_hand_with_joker() {
        assert_eq!(Hand::from_str("AAAAA", true), Ok(Hand::FiveOfAKind));
        assert_eq!(Hand::from_str("88888", true), Ok(Hand::FiveOfAKind));
        assert_eq!(Hand::from_str("JKKKK", true), Ok(Hand::FiveOfAKind));
        assert_eq!(Hand::from_str("JJJJJ", true), Ok(Hand::FiveOfAKind));

        assert_eq!(Hand::from_str("Q8888", true), Ok(Hand::FourOfAKind));
        assert_eq!(Hand::from_str("KK5KK", true), Ok(Hand::FourOfAKind));
        assert_eq!(Hand::from_str("KK5JK", true), Ok(Hand::FourOfAKind));
        assert_eq!(Hand::from_str("838J8", true), Ok(Hand::FourOfAKind));

        assert_eq!(Hand::from_str("K55KK", true), Ok(Hand::FullHouse));
        assert_eq!(Hand::from_str("83838", true), Ok(Hand::FullHouse));
        assert_eq!(Hand::from_str("8383J", true), Ok(Hand::FullHouse));

        assert_eq!(Hand::from_str("AA5AT", true), Ok(Hand::ThreeOfAKind));
        assert_eq!(Hand::from_str("AA5JT", true), Ok(Hand::ThreeOfAKind));
        assert_eq!(Hand::from_str("3KJK4", true), Ok(Hand::ThreeOfAKind));

        assert_eq!(Hand::from_str("3K3K4", true), Ok(Hand::TwoPair));
        assert_eq!(Hand::from_str("AA5TT", true), Ok(Hand::TwoPair));

        assert_eq!(Hand::from_str("JA54T", true), Ok(Hand::OnePair));
        assert_eq!(Hand::from_str("J354T", true), Ok(Hand::OnePair));

        assert_eq!(Hand::from_str("23456", true), Ok(Hand::HighCard));

        // Parsing error
        assert_eq!(Hand::from_str("2345", true), Err(ParseHandError));
        assert_eq!(Hand::from_str("234567", true), Err(ParseHandError));
    }

    #[test]
    fn test_parse_game() {
        assert_eq!(
            Game::from_str("AAAAA 123", false),
            Ok(Game {
                hand: Hand::FiveOfAKind,
                hand_str: "AAAAA".to_owned(),
                bid: 123,
                with_joker: false
            })
        );
        assert_eq!(
            Game::from_str("88888 234", false),
            Ok(Game {
                hand: Hand::FiveOfAKind,
                hand_str: "88888".to_owned(),
                bid: 234,
                with_joker: false
            })
        );

        assert_eq!(
            Game::from_str("A8888 3", false),
            Ok(Game {
                hand: Hand::FourOfAKind,
                hand_str: "A8888".to_owned(),
                bid: 3,
                with_joker: false
            })
        );
        assert_eq!(
            Game::from_str("TKKKK 4567", false),
            Ok(Game {
                hand: Hand::FourOfAKind,
                hand_str: "TKKKK".to_owned(),
                bid: 4567,
                with_joker: false
            })
        );

        assert_eq!(
            Game::from_str("K55KK 34", false),
            Ok(Game {
                hand: Hand::FullHouse,
                hand_str: "K55KK".to_owned(),
                bid: 34,
                with_joker: false
            })
        );
        assert_eq!(
            Game::from_str("2Q2Q4 24", false),
            Ok(Game {
                hand: Hand::TwoPair,
                hand_str: "2Q2Q4".to_owned(),
                bid: 24,
                with_joker: false
            })
        );
        assert_eq!(
            Game::from_str("JJ54T 2", false),
            Ok(Game {
                hand: Hand::OnePair,
                hand_str: "JJ54T".to_owned(),
                bid: 2,
                with_joker: false
            })
        );
        assert_eq!(
            Game::from_str("23456 93", false),
            Ok(Game {
                hand: Hand::HighCard,
                hand_str: "23456".to_owned(),
                bid: 93,
                with_joker: false
            })
        );
    }

    #[test]
    fn test_game_ordering_without_joker() {
        let game_higher = Game {
            hand: Hand::OnePair,
            hand_str: "AA54T".to_owned(),
            bid: 2,
            with_joker: false,
        };
        let game_lower = Game {
            hand: Hand::HighCard,
            hand_str: "23456".to_owned(),
            bid: 93,
            with_joker: false,
        };
        assert!(game_higher != game_lower);
        assert!(game_higher > game_lower);
        assert!(game_higher >= game_lower);

        let game_higher = Game {
            hand: Hand::FiveOfAKind,
            hand_str: "AAAAA".to_owned(),
            bid: 123,
            with_joker: false,
        };
        let game_lower = Game {
            hand: Hand::FiveOfAKind,
            hand_str: "88888".to_owned(),
            bid: 234,
            with_joker: false,
        };
        assert!(game_higher != game_lower);
        assert!(game_higher > game_lower);
        assert!(game_higher >= game_lower);

        let game_higher = Game {
            hand: Hand::FiveOfAKind,
            hand_str: "AAAAA".to_owned(),
            bid: 123,
            with_joker: false,
        };
        let game_lower = Game {
            hand: Hand::FiveOfAKind,
            hand_str: "JJJJJ".to_owned(),
            bid: 234,
            with_joker: false,
        };
        assert!(game_higher != game_lower);
        assert!(game_higher > game_lower);
        assert!(game_higher >= game_lower);
    }

    #[test]
    fn test_game_ordering_with_joker() {
        let game_higher = Game {
            hand: Hand::OnePair,
            hand_str: "AA54T".to_owned(),
            bid: 2,
            with_joker: true,
        };
        let game_lower = Game {
            hand: Hand::HighCard,
            hand_str: "23456".to_owned(),
            bid: 93,
            with_joker: true,
        };
        assert!(game_higher != game_lower);
        assert!(game_higher > game_lower);
        assert!(game_higher >= game_lower);

        let game_higher = Game {
            hand: Hand::FiveOfAKind,
            hand_str: "AAAAA".to_owned(),
            bid: 123,
            with_joker: true,
        };
        let game_lower = Game {
            hand: Hand::FiveOfAKind,
            hand_str: "88888".to_owned(),
            bid: 234,
            with_joker: true,
        };
        assert!(game_higher != game_lower);
        assert!(game_higher > game_lower);
        assert!(game_higher >= game_lower);

        let game_higher = Game {
            hand: Hand::FiveOfAKind,
            hand_str: "JJAJJ".to_owned(),
            bid: 123,
            with_joker: true,
        };
        let game_lower = Game {
            hand: Hand::FiveOfAKind,
            hand_str: "JJJAA".to_owned(),
            bid: 234,
            with_joker: true,
        };
        assert!(game_higher != game_lower);
        assert!(game_higher > game_lower);
        assert!(game_higher >= game_lower);

        let game_higher = Game {
            hand: Hand::FiveOfAKind,
            hand_str: "22222".to_owned(),
            bid: 123,
            with_joker: true,
        };
        let game_lower = Game {
            hand: Hand::FiveOfAKind,
            hand_str: "JJJJJ".to_owned(),
            bid: 234,
            with_joker: true,
        };
        assert!(game_higher != game_lower);
        assert!(game_higher > game_lower);
        assert!(game_higher >= game_lower);
    }
}
