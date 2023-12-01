// --- Day 1: Trebuchet?! ---
use std::env;
use std::fs;


fn puzzle1(data: &str) -> i32 {
    data.lines().map(|line| {
        let first = line.find(char::is_numeric).unwrap();
        let last = line.rfind(char::is_numeric).unwrap();
        let mut num = String::new();
        num.push(line.chars().nth(first).unwrap());
        num.push(line.chars().nth(last).unwrap());
        num.parse::<i32>().unwrap()
    }).sum::<i32>()
}

const  NUMBERS: [&'static str; 18] = [
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];


fn puzzle2(data: &str) -> i32 {
    data.lines().map(|line| {
        let mut first_idx = line.len();
        let mut last_idx = 0;
        let mut first = None;
        let mut last = None;
        for (i, number) in NUMBERS.iter().enumerate() {
            if let Some(start) = line.find(number) {
                if start <= first_idx{
                    first_idx = start;
                    first = Some(i%9 + 1);
                }
            }
            if let Some(end) = line.rfind(number) {
                if end >= last_idx{
                    last_idx = end;
                    last = Some(i%9 + 1);
                }
            }
        }
        match (first, last) {
            (Some(f), Some(l)) => {
                (f*10 + l) as i32
            },
            _ => panic!("No number found in line: {}", line),
        }
    }).sum::<i32>()
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2{
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
mod tests{
    use super::*;

    #[test]
    fn test_puzzle1() {
        let test_data = "1abc2\n\
                         pqr3stu8vwx\n\
                         a1b2c3d4e5f\n\
                         treb7uchet";
        let res = puzzle1(test_data);
        assert_eq!(res, 142);
    }

    #[test]
    fn test_puzzle2() {
        let test_data = "two1nine\n\
                         eightwothree\n\
                         abcone2threexyz\n\
                         xtwone3four\n\
                         4nineeightseven2\n\
                         zoneight234\n\
                         7pqrstsixteen";
        let res = puzzle2(test_data);
        assert_eq!(res, 281);
    }
}
