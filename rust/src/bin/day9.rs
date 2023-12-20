// --- Day 9: Mirage Maintenance ---

use std::env;
use std::fs;

use std::num::ParseIntError;

fn extrapolate_sequence(seq: &Vec<i32>, check_front: bool) -> i32 {
    if seq.iter().all(|&n| n == 0) {
        0
    } else {
        let diff = seq.windows(2).map(|sl| sl[1] - sl[0]).collect::<Vec<i32>>();
        if check_front {
            seq.first().unwrap_or(&0) - extrapolate_sequence(&diff, check_front)
        } else {
            seq.last().unwrap_or(&0) + extrapolate_sequence(&diff, check_front)
        }
    }
}

fn puzzle1(data: &str) -> i32 {
    data.lines()
        .map(|line| {
            let seq = line
                .split_whitespace()
                .map(str::parse::<i32>)
                .collect::<Result<Vec<i32>, ParseIntError>>()
                .unwrap();
            extrapolate_sequence(&seq, false)
        })
        .sum()
}

fn puzzle2(data: &str) -> i32 {
    data.lines()
        .map(|line| {
            let seq = line
                .split_whitespace()
                .map(str::parse::<i32>)
                .collect::<Result<Vec<i32>, ParseIntError>>()
                .unwrap();
            extrapolate_sequence(&seq, true)
        })
        .sum()
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
        assert_eq!(puzzle1("0 3 6 9 12 15"), 18);
        assert_eq!(puzzle1("1 3 6 10 15 21"), 28);
        assert_eq!(puzzle1("10 13 16 21 30 45"), 68);

        let test_data = "\
                         0 3 6 9 12 15\n\
                         1 3 6 10 15 21\n\
                         10 13 16 21 30 45";
        let res = puzzle1(test_data);
        assert_eq!(res, 114);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2("10 13 16 21 30 45"), 5);
    }
}
