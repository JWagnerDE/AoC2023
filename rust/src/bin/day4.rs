// --- Day 4: Scratchcards ---

use std::env;
use std::fs;

use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Card {
    card_number: u32,
    winning_numbers: Vec<u32>,
    own_numbers: Vec<u32>,
}

impl Card {
    fn wins(&self) -> u32 {
        let winning_hash: HashSet<u32> = HashSet::from_iter(self.winning_numbers.clone());
        let own_hash: HashSet<u32> = HashSet::from_iter(self.own_numbers.clone());
        winning_hash.intersection(&own_hash).count() as u32
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCardError;

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once(":").ok_or(ParseCardError)?;
        let card_number = match first.split_once(" ") {
            Some(("Card", n)) => n.trim().parse::<u32>().ok(),
            _ => None,
        }
        .ok_or(ParseCardError)?;
        let (win_num, own_num) = second.split_once(" | ").ok_or(ParseCardError)?;
        let winning_numbers = win_num
            .split_whitespace()
            .map(str::trim)
            .map(str::parse::<u32>)
            .collect::<Result<Vec<u32>, ParseIntError>>()
            .map_err(|_| ParseCardError)?;

        let own_numbers = own_num
            .split_whitespace()
            .map(str::trim)
            .map(str::parse::<u32>)
            .collect::<Result<Vec<u32>, ParseIntError>>()
            .map_err(|_| ParseCardError)?;
        Ok(Self {
            card_number,
            winning_numbers,
            own_numbers,
        })
    }
}

fn puzzle1(data: &str) -> i32 {
    data.lines()
        .map(|line| Card::from_str(line).unwrap())
        .map(|card| {
            let wins = card.wins();
            if wins == 0 {
                0
            } else {
                2i32.pow(wins - 1)
            }
        })
        .sum()
}

fn puzzle2(data: &str) -> i32 {
    let mut multiplier_map: HashMap<u32, u32> = HashMap::new();
    for (i, card) in data
        .lines()
        .map(|line| Card::from_str(line).unwrap())
        .enumerate()
    {
        let id = i as u32;
        multiplier_map.insert(id, *multiplier_map.get(&id).unwrap_or(&0) + 1);
        let multiplier = match multiplier_map.get(&id) {
            Some(n) => *n,
            None => 1,
        };
        let wins = card.wins();
        for k in id + 1..id + 1 + wins {
            multiplier_map.insert(k, *multiplier_map.get(&k).unwrap_or(&0) + multiplier);
        }
    }
    multiplier_map.values().sum::<u32>() as i32
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
        assert_eq!(
            puzzle1("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            8
        );
        assert_eq!(
            puzzle1("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
            2
        );
        assert_eq!(
            puzzle1("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            1
        );
        let test_data = "\
                         Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
                         Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
                         Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
                         Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
                         Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
                         Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let res = puzzle1(test_data);
        assert_eq!(res, 13);
    }

    #[test]
    fn test_puzzle2() {
        let test_data = "\
                         Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
                         Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
                         Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
                         Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
                         Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
                         Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let res = puzzle2(test_data);
        assert_eq!(res, 30);
    }
}
