// --- Day 3: Gear Ratios ---
use std::env;
use std::fs;

use std::char;

#[derive(Debug, PartialEq, Eq)]
enum Entry {
    DOT,
    NUMBER(u8),
    SYMBOL,
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
            _ => Ok(Self::SYMBOL),
        }
    }
}

fn puzzle1(data: &str) -> i32 {
    let lines: Vec<&str> = data.lines().collect();
    let y = lines.len();
    let x = lines[0].len();
    let map: Vec<Vec<Entry>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(Entry::from_char)
                .collect::<Result<Vec<Entry>, ParseEntryError>>()
                .unwrap()
        })
        .collect();
    for (x, line) in map.iter().enumerate() {
        for (y, entry) in line.iter().enumerate() {
            match entry {
                Entry::SYMBOL => {

                },
                _ => {}
            }
        }
    }
    // "123*..456"
    // "123..*456"
    // "123$.*456"
    // "123#.*456"
    // "123...456
    // ".*......."
    //
    // "123...456"
    // "......$.."
    //
    // "123...456"
    // ".....#..."
    //
    // "123...456"
    // "...$.#..."
    //
    // "..34....3"
    // ".$......."
    // "123...456"
    //
    // "..34....3"
    // "...*...$."
    // "123...456"
    // "...$.#..."
    //
    // "..34....3"
    // "...*....."
    // "123...456"
    // "........."
    //
    // "..348...."
    // "...$.*..."
    // "123...456"
    // "........."
    0
}

fn puzzle2(data: &str) -> i32 {
    0
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
                         467..114..
                         ...*......
                         ..35..633.
                         ......#...
                         617*......
                         .....+.58.
                         ..592.....
                         ......755.
                         ...$.*....
                         .664.598..";
        let res = puzzle1(test_data);
        assert_eq!(res, 4361);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(
            puzzle2("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            48
        );
        let test_data = "\
                         Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
                         Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
                         Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
                         Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
                         Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let res = puzzle2(test_data);
        assert_eq!(res, 2286);
    }
}
