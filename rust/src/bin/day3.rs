// --- Day 3: Gear Ratios ---
use std::env;
use std::fs;

use std::char;
use std::cmp;

#[derive(Debug, PartialEq, Eq)]
enum Entry {
    DOT,
    NUMBER(u8),
    SYMBOL(char),
}

#[derive(Debug, PartialEq, Eq)]
struct ParseEntryError;

impl Entry {
    fn from_char(c: char) -> Result<Self, ParseEntryError> {
        match c {
            '.' => Ok(Self::DOT),
            '0'..='9' => match c.to_digit(10) {
                Some(n) => Ok(Self::NUMBER(n as u8)),
                None => Err(ParseEntryError),
            },
            c => Ok(Self::SYMBOL(c)),
        }
    }
}

fn puzzle1(data: &str) -> i32 {
    let mut sum = 0;

    let lines: Vec<&str> = data.lines().collect();
    let max_x = lines.len() - 1;
    let max_y = lines[0].len() - 1;
    let map: Vec<Vec<Entry>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(Entry::from_char)
                .collect::<Result<Vec<Entry>, ParseEntryError>>()
                .unwrap()
        })
        .collect();
    // println!("data:\n'{}'", data);
    for (x, line) in map.iter().enumerate() {
        let mut number: u32 = 0;
        let mut number_valid = false;
        for (y, entry) in line.iter().enumerate() {
            match entry {
                Entry::NUMBER(d) => {
                    number = number * 10 + *d as u32;
                    'windowing: for dx in x.checked_sub(1).unwrap_or(0)..=cmp::min(x + 1, max_x) {
                        for dy in y.checked_sub(1).unwrap_or(0)..=cmp::min(y + 1, max_y) {
                            if let Entry::SYMBOL(_) = map[dx][dy] {
                                number_valid = true;
                                break 'windowing;
                            }
                        }
                    }
                }
                _ => {
                    if number_valid {
                        // println!("{:?} -> x:{}, y:{} -> num:{}, sum:{}", entry, x, y, number, sum);
                        sum += number;
                    }
                    number = 0;
                    number_valid = false;
                }
            }
        }
        if number_valid {
            // println!("End -> x:{} -> num:{}, sum:{}", x, number, sum);
            sum += number;
        }
    }
    // println!("END SUM: {}", sum);
    sum as i32
}

fn puzzle2(data: &str) -> i32 {
    let mut sum = 0;

    let lines: Vec<&str> = data.lines().collect();
    let max_x = lines.len() - 1;
    let max_y = lines[0].len() - 1;
    let map: Vec<Vec<Entry>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(Entry::from_char)
                .collect::<Result<Vec<Entry>, ParseEntryError>>()
                .unwrap()
        })
        .collect();
    // println!("data:\n{}", data);
    for (y, line) in map.iter().enumerate() {
        for (x, entry) in line.iter().enumerate() {
            // println!("Testing {},{}", x,y);
            match entry {
                Entry::SYMBOL('*') => {
                    let mut numbers  = Vec::new();
                    // println!("Is *. Checking window");
                    for dy in y.checked_sub(1).unwrap_or(0)..=cmp::min(y + 1, max_y) {
                        let mut x_end = 0;
                        for dx in x.checked_sub(1).unwrap_or(0)..=cmp::min(x + 1, max_x) {
                            // println!("win: {},{}", dx, dy);
                            if x_end > dx {
                                // println!("continue: {} > {}", x_end, dx);
                                continue;
                            }
                            if let Entry::NUMBER(_) = map[dy][dx] {
                                let mut x_start = dx;
                                // println!("Is number. Searching for start starting from x: {}", x_start);
                                while x_start > 0 {
                                    x_start -= 1;
                                    match map[dy][x_start] {
                                        Entry::NUMBER(_) => {},
                                        _ => {
                                            x_start += 1;
                                            break;
                                        }
                                    }
                                }
                                // println!("x_start: {}", x_start);
                                // println!("Evaluating number");
                                x_end = x_start;
                                let mut num: u32 = 0;
                                while x_end <= max_x {
                                    // println!("checking number at {}, {}", x_end, dy);
                                    match map[dy][x_end] {
                                        Entry::NUMBER(d) => {
                                            num = num*10 + d as u32;
                                            // println!("d: {}, num: {}", d, num);
                                        }
                                        _ => {
                                            numbers.push(num.clone());
                                            // println!("Number done at x {}. Num is {}. Numbers are {:?}", x_end, num, numbers);
                                            break;
                                        }
                                    }
                                    x_end += 1;
                                }
                            }
                        }
                    }
                    if numbers.len() > 1 {
                        println!("{:?}", numbers);
                        let number = numbers
                            .iter()
                            .copied()
                            .reduce(|acc, e| acc * e)
                            .unwrap();
                        // println!("number before sum {}. Sum {}", number, sum);
                        sum += number;
                    }
                }
                _ => {},
            }
        }
    }
    // println!("END SUM: {}", sum);
    sum as i32
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
        assert_eq!(puzzle1("123*..456"), 123);
        assert_eq!(puzzle1("123..*456"), 456);
        assert_eq!(puzzle1("123$.*456"), 579);
        assert_eq!(puzzle1("123#.*456"), 579);
        assert_eq!(puzzle1("123...456\n.*......."), 123);
        assert_eq!(puzzle1("123...456\n......$.."), 456);
        assert_eq!(puzzle1("123...456\n.....#..."), 456);
        assert_eq!(puzzle1("123...456\n...$.#..."), 579);
        assert_eq!(puzzle1("..34....3\n.$.......\n123...456"), 34 + 123);
        assert_eq!(
            puzzle1("..34....3\n...*...$.\n123...456\n...$.#..."),
            34 + 123 + 3 + 456
        );
        assert_eq!(
            puzzle1("..34....3\n...*.....\n123...456\n........."),
            34 + 123
        );
        assert_eq!(
            puzzle1("..348....\n...$.*...\n123...456\n........."),
            348 + 123 + 456
        );
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
