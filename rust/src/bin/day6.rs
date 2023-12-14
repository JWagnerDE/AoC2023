// --- Day 6: Wait For It ---
use std::env;
use std::fs;

fn solve_race_wins(race_time: u32, race_distance: u32) -> Vec<u32> {
    vec!(race_time, race_distance)
}

fn puzzle1(data: &str) -> i32 {
    solve_race_wins(0,0);
    data.len() as i32
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
    fn test_single_race() {
        let test_data = "\
                         Time:      7\n\
                         Distance:  9";
        let res = puzzle1(test_data);
        assert_eq!(res, 288);
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
        let test_data = "";
        let res = puzzle2(test_data);
        assert_eq!(res, -1);
    }
}
