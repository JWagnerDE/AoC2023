// --- Day 4: Scratchcards ---

use std::env;
use std::fs;

fn puzzle1(data: &str) -> i32 {
    0
}

fn puzzle2(data: &str) -> i32 {
    data.len() as i32 * 0
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
                         467..114..\n\
                         ...*......\n\
                         ..35..633.\n\
                         ......#...\n\
                         617*......\n\
                         .....+.58.\n\
                         ..592.....\n\
                         ......755.\n\
                         ...$.*....\n\
                         .664.598..";
        let res = puzzle1(test_data);
        assert_eq!(res, 4361);
    }

    #[test]
    fn test_puzzle2() {
        let test_data = "\
                         467..114..\n\
                         ...*......\n\
                         ..35..633.\n\
                         ......#...\n\
                         617*......\n\
                         .....+.58.\n\
                         ..592.....\n\
                         ......755.\n\
                         ...$.*....\n\
                         .664.598..";
        let res = puzzle2(test_data);
        assert_eq!(res, 467835);
    }
}
