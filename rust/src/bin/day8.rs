// --- Day 8: Haunted Wasteland ---
use std::env;
use std::fs;

use num::integer::lcm;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    R,
    L,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseDirectionError;

impl Direction {
    fn from_char(s: char) -> Result<Self, ParseDirectionError> {
        match s {
            'R' => Ok(Self::R),
            'L' => Ok(Self::L),
            _ => Err(ParseDirectionError),
        }
    }
}

fn puzzle1(data: &str) -> u64 {
    let mut lines = data.lines();
    let instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(Direction::from_char)
        .collect::<Result<Vec<Direction>, ParseDirectionError>>()
        .unwrap();
    lines.next().unwrap();
    let doc_map: HashMap<&str, (&str, &str)> = HashMap::from_iter(lines.map(|line| {
        let (key, second) = line.split_once(" = ").unwrap();
        let (left, right) = second
            .trim_start_matches("(")
            .trim_end_matches(")")
            .split_once(", ")
            .unwrap();
        (key, (left, right))
    }));
    let mut steps = 0u64;
    let mut current = "AAA";

    for dir in instructions.iter().cycle() {
        if current == "ZZZ" {
            break;
        }
        current = match dir {
            Direction::L => doc_map.get(current).unwrap().0,
            Direction::R => doc_map.get(current).unwrap().1,
        };
        steps += 1;
    }
    steps
}

fn puzzle2(data: &str) -> u64 {
    let mut lines = data.lines();
    let instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(Direction::from_char)
        .collect::<Result<Vec<Direction>, ParseDirectionError>>()
        .unwrap();
    lines.next().unwrap();
    let doc_map: HashMap<&str, (&str, &str)> = HashMap::from_iter(lines.map(|line| {
        let (key, second) = line.split_once(" = ").unwrap();
        let (left, right) = second
            .trim_start_matches("(")
            .trim_end_matches(")")
            .split_once(", ")
            .unwrap();
        (key, (left, right))
    }));
    let mut start_keys = Vec::new();
    for key in doc_map.keys() {
        if key.ends_with("A") {
            start_keys.push(*key);
        }
    }

    let steps: Vec<usize> = start_keys
        .iter()
        .map(|&start_key| {
            let mut current = start_key;
            let mut current_steps = 0;
            for dir in instructions.iter().cycle() {
                if current.ends_with("Z") {
                    break;
                }
                current = match dir {
                    Direction::L => doc_map.get(current).unwrap().0,
                    Direction::R => doc_map.get(current).unwrap().1,
                };
                current_steps += 1;
            }
            current_steps
        })
        .collect();
    let mut acc = steps[0] as u64;
    for s in &steps[1..] {
        acc = lcm(acc, *s as u64);
    }
    acc
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
                         RL\n\
                         \n\
                         AAA = (BBB, CCC)\n\
                         BBB = (DDD, EEE)\n\
                         CCC = (ZZZ, GGG)\n\
                         DDD = (DDD, DDD)\n\
                         EEE = (EEE, EEE)\n\
                         GGG = (GGG, GGG)\n\
                         ZZZ = (ZZZ, ZZZ)";
        let res = puzzle1(test_data);
        assert_eq!(res, 2);

        let test_data = "\
                         LLR\n\
                         \n\
                         AAA = (BBB, BBB)\n\
                         BBB = (AAA, ZZZ)\n\
                         ZZZ = (ZZZ, ZZZ)";
        let res = puzzle1(test_data);
        assert_eq!(res, 6);
    }

    #[test]
    fn test_puzzle2() {
        let test_data = "\
                         LR\n\
                         \n\
                         11A = (11B, XXX)\n\
                         11B = (XXX, 11Z)\n\
                         11Z = (11B, XXX)\n\
                         22A = (22B, XXX)\n\
                         22B = (22C, 22C)\n\
                         22C = (22Z, 22Z)\n\
                         22Z = (22B, 22B)\n\
                         XXX = (XXX, XXX)";
        let res = puzzle2(test_data);
        assert_eq!(res, 6);
    }
}
