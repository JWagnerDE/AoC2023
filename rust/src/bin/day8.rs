// --- Day 8: Haunted Wasteland ---
use std::env;
use std::fs;

use itertools::Itertools;
use std::collections::HashMap;

fn puzzle1(data: &str) -> i32 {
    let (instructions, documents_str) = data.split_once("\n\n").unwrap();
    let mut document_map = HashMap::new();
    for line in documents_str.split("\n").filter(|line| !line.is_empty()) {
        let (key, right_side) = line.split_once(" = ").unwrap();
        let (left_node, right_node) = right_side
            .trim_start_matches("(")
            .trim_end_matches(")")
            .split_once(", ")
            .unwrap();
        document_map.insert(key, (left_node, right_node));
    }
    // let mut document_map_precomputed = HashMap::new();
    let inner = vec!('R', 'L');
    let outer: Vec<Vec<char>> = vec![inner.clone(); 4];
    let precomputed: Vec<&str> = outer
        .iter()
        .multi_cartesian_product()
        .map(|v| v.iter().cloned().collect::<Vec<char>>())
        .map(|v| v.iter().collect::<&str>())
        .flatten()
        .collect();
    println!("{:?}", precomputed);
    return 0;
    let mut steps: u32 = 0;
    let mut current = documents_str.split_whitespace().next().unwrap();
    // TODO: Since we need to cycle the instructions untwill we reach ZZZ,
    //       This operation will take very long with the real dataset.
    //       A better soloution would be to precompute for every document
    //       The next document if we get R or L as input.
    //       Even better would be to make the computation have 2 3 or 4
    //       instructions.
    //       So compute the next document if we have RRLR on doc ABC.
    //       But think of the exit criteria.
    //       Especially if it is in the middle of the precomputed step.
    for dir in instructions.chars().cycle() {
        current = match dir {
            'L' => document_map.get(current).unwrap().0,
            'R' => document_map.get(current).unwrap().1,
            _ => unreachable!(),
        };
        steps += 1;
        if current == "ZZZ" {
            break;
        }
    }
    steps as i32
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
    #[ignore]
    fn test_puzzle2() {
        let test_data = "";
        let res = puzzle2(test_data);
        assert_eq!(res, -1);
    }
}
