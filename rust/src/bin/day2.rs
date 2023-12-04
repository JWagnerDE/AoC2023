// --- Day 2: Cube Conundrum ---
use std::env;
use std::fs;

use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Draw {
    r: Option<u32>,
    g: Option<u32>,
    b: Option<u32>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseDrawError;

impl FromStr for Draw {
    type Err = ParseDrawError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 1 red, 2 green, 6 blue
        let mut r = None;
        let mut g = None;
        let mut b = None;

        for color_split in s.split(", ") {
            let (ammount_str, color_str) = color_split.split_once(" ").ok_or(ParseDrawError)?;
            let ammount = ammount_str.parse::<u32>().map_err(|_| ParseDrawError)?;
            match color_str {
                "red" => r = Some(ammount),
                "green" => g = Some(ammount),
                "blue" => b = Some(ammount),
                _ => return Err(ParseDrawError),
            };
        }
        Ok(Draw { r, g, b })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

        let (s1, s2) = s.split_once(":").ok_or(ParseGameError)?;
        let id = s1
            .split_once(" ")
            .ok_or(ParseGameError)?
            .1
            .parse::<u32>()
            .map_err(|_| ParseGameError)?;
        let draws: Result<Vec<Draw>, ParseGameError> = s2
            .trim()
            .split("; ")
            .map(|s| Draw::from_str(s).map_err(|_| ParseGameError))
            .collect();
        Ok(Game { id, draws: draws? })
    }
}

fn puzzle1(data: &str) -> i32 {
    let max_r = 12;
    let max_g = 13;
    let max_b = 14;

    let mut sum = 0;

    let games: Vec<Game> = data
        .lines()
        .map(Game::from_str)
        .collect::<Result<Vec<Game>, ParseGameError>>()
        .unwrap();
    for game in games {
        let mut valid_game = true;
        for draw in game.draws {
            if draw.r > Some(max_r) || draw.g > Some(max_g) || draw.b > Some(max_b) {
                valid_game = false;
                break;
            }
        }
        if valid_game {
            sum += game.id;
        }
    }
    sum as i32
}

fn puzzle2(data: &str) -> i32 {
    data.lines()
        .map(|s| Game::from_str(s).unwrap())
        .map(|game: Game| {
            let mut max_r = None;
            let mut max_g = None;
            let mut max_b = None;

            for draw in game.draws {
                if let Some(r) = draw.r {
                    max_r = Some(std::cmp::max(r, max_r.unwrap_or(std::u32::MIN)));
                }
                if let Some(g) = draw.g {
                    max_g = Some(std::cmp::max(g, max_g.unwrap_or(std::u32::MIN)));
                }
                if let Some(b) = draw.b {
                    max_b = Some(std::cmp::max(b, max_b.unwrap_or(std::u32::MIN)));
                }
            }
            max_r.unwrap_or(1) * max_g.unwrap_or(1) * max_b.unwrap_or(1)
        })
        .sum::<u32>() as i32
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
                         Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
                         Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
                         Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
                         Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
                         Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let res = puzzle1(test_data);
        assert_eq!(res, 8);
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

    #[test]
    fn test_draw_from_str() {
        assert_eq!(
            Draw::from_str("1 red, 2 blue, 3 green"),
            Ok(Draw {
                r: Some(1),
                g: Some(3),
                b: Some(2)
            })
        );
        assert_eq!(
            Draw::from_str("13 green, 4 blue"),
            Ok(Draw {
                r: None,
                g: Some(13),
                b: Some(4)
            })
        );
        assert_eq!(
            Draw::from_str("20 red"),
            Ok(Draw {
                r: Some(20),
                g: None,
                b: None
            })
        );
        assert_eq!(Draw::from_str(""), Err(ParseDrawError));
        assert_eq!(Draw::from_str("20 redd"), Err(ParseDrawError));
        assert_eq!(Draw::from_str("1 red 2 blue  3 green"), Err(ParseDrawError));
    }

    #[test]
    fn test_game_from_str() {
        assert_eq!(
            Game::from_str("Game 1: 1 red, 2 blue, 3 green"),
            Ok(Game {
                id: 1,
                draws: vec!(Draw {
                    r: Some(1),
                    g: Some(3),
                    b: Some(2)
                })
            })
        );
        assert_eq!(
            Game::from_str("Game 15: 13 green, 4 blue"),
            Ok(Game {
                id: 15,
                draws: vec!(Draw {
                    r: None,
                    g: Some(13),
                    b: Some(4)
                })
            })
        );
        assert_eq!(
            Game::from_str("Game 100: 20 red"),
            Ok(Game {
                id: 100,
                draws: vec!(Draw {
                    r: Some(20),
                    g: None,
                    b: None
                })
            })
        );
        assert_eq!(
            Game::from_str("Game 8: 1 red, 2 blue, 3 green; 12 green, 8 red"),
            Ok(Game {
                id: 8,
                draws: vec!(
                    Draw {
                        r: Some(1),
                        g: Some(3),
                        b: Some(2)
                    },
                    Draw {
                        r: Some(8),
                        g: Some(12),
                        b: None
                    }
                )
            })
        );
        assert_eq!(Game::from_str(""), Err(ParseGameError));
        assert_eq!(Game::from_str("20 redd"), Err(ParseGameError));
        assert_eq!(Game::from_str("1 red 2 blue  3 green"), Err(ParseGameError));
        assert_eq!(
            Game::from_str("Game 1 1 red, 2 blue, 3 green"),
            Err(ParseGameError)
        );
    }
}
