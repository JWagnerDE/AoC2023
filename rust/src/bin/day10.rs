// --- Day 10: Pipe Maze ---

use std::env;
use std::fs;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn step(self: &Self, y: usize, x: usize, max_y: usize, max_x: usize) -> Option<(usize, usize)> {
        if y > max_y || x > max_x {
            return None;
        }
        match self {
            Self::N => Some((y.checked_sub(1)?, x)),
            Self::E => {
                if x < max_x {
                    Some((y, x + 1))
                } else {
                    None
                }
            }
            Self::S => {
                if y < max_y {
                    Some((y + 1, x))
                } else {
                    None
                }
            }
            Self::W => Some((y, x.checked_sub(1)?)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    GROUND,
    START,
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePipeError;

impl Pipe {
    fn from_char(c: char) -> Result<Self, ParsePipeError> {
        match c {
            '|' => Ok(Self::NS),     // is a vertical pipe connecting north and south.
            '-' => Ok(Self::EW),     // is a horizontal pipe connecting east and west.
            'L' => Ok(Self::NE),     // is a 90-degree bend connecting north and east.
            'J' => Ok(Self::NW),     // is a 90-degree bend connecting north and west.
            '7' => Ok(Self::SW),     // is a 90-degree bend connecting south and west.
            'F' => Ok(Self::SE),     // is a 90-degree bend connecting south and east.
            '.' => Ok(Self::GROUND), // is ground; there is no pipe in this tile.
            'S' => Ok(Self::START),  // is the starting position of the animal;
            //  there is a pipe on this tile,
            //  but your sketch doesn't show what shape the pipe has.
            _ => Err(ParsePipeError),
        }
    }

    fn next_direction(self: &Self, dir: Direction) -> Option<Direction> {
        match dir {
            Direction::N => match self {
                Self::NS => Some(Direction::N),
                Self::SW => Some(Direction::W),
                Self::SE => Some(Direction::E),
                _ => None,
            },
            Direction::E => match self {
                Self::EW => Some(Direction::E),
                Self::NW => Some(Direction::N),
                Self::SW => Some(Direction::S),
                _ => None,
            },
            Direction::S => match self {
                Self::NS => Some(Direction::S),
                Self::NW => Some(Direction::W),
                Self::NE => Some(Direction::E),
                _ => None,
            },
            Direction::W => match self {
                Self::EW => Some(Direction::W),
                Self::NE => Some(Direction::N),
                Self::SE => Some(Direction::S),
                _ => None,
            },
        }
    }
}

fn find_starting_position(pipe_map: &Vec<Vec<Pipe>>) -> Option<(usize, usize)> {
    pipe_map.iter().enumerate().find_map(|(y, row)| {
        match row.iter().find_position(|&p| p == &Pipe::START) {
            Some((x, _)) => Some((y, x)),
            None => None,
        }
    })
}

fn parse_pipe_map(data: &str) -> Result<Vec<Vec<Pipe>>, ParsePipeError> {
    data.lines()
        .map(|line| {
            line.chars()
                .map(Pipe::from_char)
                .collect::<Result<Vec<Pipe>, ParsePipeError>>()
        })
        .collect()
}

fn puzzle1(data: &str) -> i32 {
    let pipe_map = parse_pipe_map(data).unwrap();

    let max_y = pipe_map.len();
    let max_x = pipe_map[0].len();

    // Find starting position
    let (start_y, start_x) = find_starting_position(&pipe_map).unwrap();

    // Find loops
    let directions_to_test = [Direction::N, Direction::E, Direction::S, Direction::W];

    let mut steps_per_dir = [None; 4];

    for (i, start_dir) in directions_to_test.iter().enumerate() {
        let mut steps = 0usize;
        let mut dir = *start_dir;
        let (mut y, mut x) = (start_y, start_x);
        loop {
            (y, x) = match dir.step(y, x, max_y, max_x) {
                Some((a, b)) => (a, b),
                None => break,
            };
            let next = &pipe_map[y][x];
            if next == &Pipe::START {
                steps_per_dir[i] = Some(steps);
                break;
            }
            dir = match next.next_direction(dir) {
                Some(d) => d,
                None => break,
            };
            steps += 1;
        }
    }
    let steps = steps_per_dir
        .iter()
        .filter(|&&i| i.is_some())
        .next()
        .unwrap()
        .unwrap();
    let res = (steps as f64 / 2f64).ceil() as i32;
    res
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
                         .....\n\
                         .S-7.\n\
                         .|.|.\n\
                         .L-J.\n\
                         .....";
        let res = puzzle1(test_data);
        assert_eq!(res, 4);

        let test_data = "\
                        ..F7.\n\
                        .FJ|.\n\
                        SJ.L7\n\
                        |F--J\n\
                        LJ...";
        let res = puzzle1(test_data);
        assert_eq!(res, 8);
    }

    #[test]
    #[ignore]
    fn test_puzzle2() {
        let test_data = "\
                         ";
        let res = puzzle2(test_data);
        assert_eq!(res, -1);
    }

    #[test]
    fn test_pipe_parsing() {
        assert_eq!(Pipe::from_char('|'), Ok(Pipe::NS));
        assert_eq!(Pipe::from_char('-'), Ok(Pipe::EW));
        assert_eq!(Pipe::from_char('L'), Ok(Pipe::NE));
        assert_eq!(Pipe::from_char('J'), Ok(Pipe::NW));
        assert_eq!(Pipe::from_char('7'), Ok(Pipe::SW));
        assert_eq!(Pipe::from_char('F'), Ok(Pipe::SE));
        assert_eq!(Pipe::from_char('.'), Ok(Pipe::GROUND));
        assert_eq!(Pipe::from_char('S'), Ok(Pipe::START));

        assert_eq!(Pipe::from_char('0'), Err(ParsePipeError));
        assert_eq!(Pipe::from_char('>'), Err(ParsePipeError));
        assert_eq!(Pipe::from_char('_'), Err(ParsePipeError));
    }

    #[test]
    fn test_pipe_next_dir() {
        assert_eq!(Pipe::GROUND.next_direction(Direction::N), None);
        assert_eq!(Pipe::GROUND.next_direction(Direction::S), None);
        assert_eq!(Pipe::START.next_direction(Direction::N), None);
        assert_eq!(Pipe::START.next_direction(Direction::S), None);

        assert_eq!(Pipe::NS.next_direction(Direction::N), Some(Direction::N));
        assert_eq!(Pipe::NS.next_direction(Direction::S), Some(Direction::S));
        assert_eq!(Pipe::EW.next_direction(Direction::E), Some(Direction::E));
        assert_eq!(Pipe::EW.next_direction(Direction::W), Some(Direction::W));

        assert_eq!(Pipe::NW.next_direction(Direction::N), None);
        assert_eq!(Pipe::NW.next_direction(Direction::W), None);
        assert_eq!(Pipe::NE.next_direction(Direction::N), None);
        assert_eq!(Pipe::NE.next_direction(Direction::E), None);
        assert_eq!(Pipe::SW.next_direction(Direction::S), None);
        assert_eq!(Pipe::SW.next_direction(Direction::W), None);
        assert_eq!(Pipe::SE.next_direction(Direction::S), None);
        assert_eq!(Pipe::SE.next_direction(Direction::E), None);

        assert_eq!(Pipe::NW.next_direction(Direction::S), Some(Direction::W));
        assert_eq!(Pipe::NW.next_direction(Direction::E), Some(Direction::N));
        assert_eq!(Pipe::NE.next_direction(Direction::S), Some(Direction::E));
        assert_eq!(Pipe::NE.next_direction(Direction::W), Some(Direction::N));
        assert_eq!(Pipe::SW.next_direction(Direction::N), Some(Direction::W));
        assert_eq!(Pipe::SW.next_direction(Direction::E), Some(Direction::S));
        assert_eq!(Pipe::SE.next_direction(Direction::N), Some(Direction::E));
        assert_eq!(Pipe::SE.next_direction(Direction::W), Some(Direction::S));
    }

    #[test]
    fn test_direction_step() {
        let (my, mx) = (4, 4);
        assert_eq!(Direction::N.step(0, 0, my, mx), None);
        assert_eq!(Direction::N.step(1, 0, my, mx), Some((0, 0)));
        assert_eq!(Direction::N.step(1, 6, my, mx), None);

        assert_eq!(Direction::S.step(0, 0, my, mx), Some((1, 0)));
        assert_eq!(Direction::S.step(1, 0, my, mx), Some((2, 0)));
        assert_eq!(Direction::S.step(my - 1, 0, my, mx), Some((my, 0)));
        assert_eq!(Direction::S.step(my, 0, my, mx), None);

        assert_eq!(Direction::E.step(0, 0, my, mx), Some((0, 1)));
        assert_eq!(Direction::E.step(0, 1, my, mx), Some((0, 2)));
        assert_eq!(Direction::E.step(0, mx - 1, my, mx), Some((0, mx)));
        assert_eq!(Direction::E.step(0, mx, my, mx), None);

        assert_eq!(Direction::W.step(0, 0, my, mx), None);
        assert_eq!(Direction::W.step(0, 1, my, mx), Some((0, 0)));
        assert_eq!(Direction::W.step(my + 1, 2, my, mx), None);

        assert_eq!(Direction::N.step(2, 3, my, mx), Some((1, 3)));
        assert_eq!(Direction::E.step(2, 3, my, mx), Some((2, 4)));
        assert_eq!(Direction::S.step(2, 3, my, mx), Some((3, 3)));
        assert_eq!(Direction::W.step(2, 3, my, mx), Some((2, 2)));
    }
}
