// --- Day 6: Wait For It ---
use std::env;
use std::fs;

use std::num::ParseIntError;
use std::str::FromStr;

fn solve_race_wins(race_time: u64, race_distance: u64) -> Vec<u64> {
    let mut wins = vec![];
    for t in 0..=race_time {
        let s = t;
        let d = s * (race_time - t);
        if d > race_distance {
            wins.push(d);
        }
    }
    wins
}

fn puzzle1(data: &str) -> i32 {
    let (time_str, distance_str) = data.split_once("\n").unwrap();
    let times: Vec<u64> = time_str
        .trim_start_matches("Time:")
        .trim()
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<u64>, ParseIntError>>()
        .unwrap();
    let distances: Vec<u64> = distance_str
        .trim_start_matches("Distance:")
        .trim()
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<u64>, ParseIntError>>()
        .unwrap();
    let accumulated = Iterator::zip(times.iter(), distances.iter())
        .map(|(time, dist)| solve_race_wins(*time, *dist).len())
        .reduce(|acc, wins| acc * wins)
        .unwrap();
    accumulated as i32
}

fn puzzle2(data: &str) -> i32 {
    let (time_str, distance_str) = data.split_once("\n").unwrap();
    let time: u64 = time_str
        .trim_start_matches("Time:")
        .trim()
        .split_whitespace()
        .map(String::from_str)
        .collect::<Result<String, _>>()
        .unwrap()
        .parse()
        .unwrap();
    let distance: u64 = distance_str
        .trim_start_matches("Distance:")
        .trim()
        .split_whitespace()
        .map(String::from_str)
        .collect::<Result<String, _>>()
        .unwrap()
        .parse()
        .unwrap();
    let wins = solve_race_wins(time, distance).len();
    wins as i32
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
    fn test_single_race() {
        assert_eq!(solve_race_wins(7, 9).len(), 4);
    }

    #[test]
    fn test_puzzle1() {
        let test_data = "\
                         Time:      7\n\
                         Distance:  9";
        let res = puzzle1(test_data);
        assert_eq!(res, 4);

        let test_data = "\
                         Time:      7  15   30\n\
                         Distance:  9  40  200";
        let res = puzzle1(test_data);
        assert_eq!(res, 288);
    }

    #[test]
    fn test_puzzle2() {
        let test_data = "\
                         Time:      7  15   30\n\
                         Distance:  9  40  200";
        let res = puzzle2(test_data);
        assert_eq!(res, 71503);
    }
}
