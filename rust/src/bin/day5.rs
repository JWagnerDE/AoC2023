// --- Day 5: If You Give A Seed A Fertilizer ---

use std::env;
use std::fs;

use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct GardenMap {
    destination: String,
    source: String,
    _map: HashMap<u32, u32>,
}

impl GardenMap {
    fn get(&self, source: u32) -> u32 {
        *self._map.get(&source).unwrap_or(&source)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGardenMapErr;

impl FromStr for GardenMap {
    type Err = ParseGardenMapErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let (source, destination) = lines
            .next()
            .ok_or(ParseGardenMapErr)?
            .trim_end_matches(" map:")
            .split_once("-to-")
            .ok_or(ParseGardenMapErr)?;
        let mut _map: HashMap<u32, u32> = HashMap::new();
        for line in lines {
            let mut parts = line.split_whitespace();
            let dest_range_start = parts
                .next()
                .ok_or(ParseGardenMapErr)?
                .parse::<u32>()
                .map_err(|_| ParseGardenMapErr)?;
            let src_range_start = parts
                .next()
                .ok_or(ParseGardenMapErr)?
                .parse::<u32>()
                .map_err(|_| ParseGardenMapErr)?;
            let range_len = parts
                .next()
                .ok_or(ParseGardenMapErr)?
                .parse::<u32>()
                .map_err(|_| ParseGardenMapErr)?;
            for i in 0..range_len {
                _map.insert(src_range_start + i, dest_range_start + i);
            }
        }
        Ok(Self {
            destination: destination.to_owned(),
            source: source.to_owned(),
            _map,
        })
    }
}

fn puzzle1(data: &str) -> i32 {
    let mut parts = data.split("\n\n");
    let seed_line = parts.next().unwrap();
    let seeds = seed_line
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect::<Vec<u32>>();
    let garden_maps = parts
        .map(GardenMap::from_str)
        .map(Result::unwrap)
        .collect::<Vec<GardenMap>>();

    0
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
                         seeds: 79 14 55 13\n\
                         \n\
                         seed-to-soil map:\n\
                         50 98 2\n\
                         52 50 48\n\
                         \n\
                         soil-to-fertilizer map:\n\
                         0 15 37\n\
                         37 52 2\n\
                         39 0 15\n\
                         \n\
                         fertilizer-to-water map:\n\
                         49 53 8\n\
                         0 11 42\n\
                         42 0 7\n\
                         57 7 4\n\
                         \n\
                         water-to-light map:\n\
                         88 18 7\n\
                         18 25 70\n\
                         \n\
                         light-to-temperature map:\n\
                         45 77 23\n\
                         81 45 19\n\
                         68 64 13\n\
                         \n\
                         temperature-to-humidity map:\n\
                         0 69 1\n\
                         1 0 69\n\
                         \n\
                         humidity-to-location map:\n\
                         60 56 37\n\
                         56 93 4";
        let res = puzzle1(test_data);
        assert_eq!(res, 35);
    }

    #[test]
    fn test_puzzle2() {
        let test_data = "\
                         ";
        let res = puzzle2(test_data);
        assert_eq!(res, -1);
    }

    #[test]
    fn test_gardenmap_parse() {
        let input = "\
                     seed-to-soil map:\n\
                     50 98 2\n\
                     52 50 48";
        let mut _map = HashMap::new();
        _map.extend((0..2).map(|i| (98 + i, 50 + i)));
        _map.extend((0..48).map(|i| (50 + i, 52 + i)));
        let res = GardenMap {
            destination: "soil".to_owned(),
            source: "seed".to_owned(),
            _map,
        };
        assert_eq!(GardenMap::from_str(input).unwrap(), res);
        let input = "\
                     soil-to-fertilizer map:\n\
                     0 15 37\n\
                     37 52 2\n\
                     39 0 15";
        let mut _map = HashMap::new();
        _map.extend((0..37).map(|i| (15 + i, 0 + i)));
        _map.extend((0..2).map(|i| (52 + i, 37 + i)));
        _map.extend((0..15).map(|i| (0 + i, 39 + i)));
        let res = GardenMap {
            destination: "fertilizer".to_owned(),
            source: "soil".to_owned(),
            _map,
        };
        assert_eq!(GardenMap::from_str(input).unwrap(), res);
        let input = "\
                     fertilizer-to-water map:\n\
                     49 53 8\n\
                     0 11 42\n\
                     42 0 7\n\
                     57 7 4";
        let mut _map = HashMap::new();
        _map.extend((0..8).map(|i| (53 + i, 49 + i)));
        _map.extend((0..42).map(|i| (11 + i, 0 + i)));
        _map.extend((0..7).map(|i| (0 + i, 42 + i)));
        _map.extend((0..4).map(|i| (7 + i, 57 + i)));
        let res = GardenMap {
            destination: "water".to_owned(),
            source: "fertilizer".to_owned(),
            _map,
        };
        assert_eq!(GardenMap::from_str(input).unwrap(), res);
        println!("{}", res.get(3))
    }

    #[test]
    fn test_gardenmap_get() {
        let mut _map = HashMap::new();
        _map.extend((0..2).map(|i| (98 + i, 50 + i)));
        _map.extend((0..48).map(|i| (50 + i, 52 + i)));
        let gm = GardenMap {
            destination: "soil".to_owned(),
            source: "seed".to_owned(),
            _map,
        };
        assert_eq!(gm.get(0), 0);
        assert_eq!(gm.get(1), 1);
        assert_eq!(gm.get(2), 2);
        assert_eq!(gm.get(49), 49);
        assert_eq!(gm.get(50), 52);
        assert_eq!(gm.get(51), 53);
        assert_eq!(gm.get(52), 54);
        assert_eq!(gm.get(68), 70);
        assert_eq!(gm.get(97), 99);
        assert_eq!(gm.get(98), 50);
        assert_eq!(gm.get(99), 51);
        assert_eq!(gm.get(100), 100);
        assert_eq!(gm.get(101), 101);

        let mut _map = HashMap::new();
        _map.extend((0..37).map(|i| (15 + i, 0 + i)));
        _map.extend((0..2).map(|i| (52 + i, 37 + i)));
        _map.extend((0..15).map(|i| (0 + i, 39 + i)));
        let gm = GardenMap {
            destination: "fertilizer".to_owned(),
            source: "soil".to_owned(),
            _map,
        };
        assert_eq!(gm.get(0), 39);
        assert_eq!(gm.get(1), 40);
        assert_eq!(gm.get(14), 53);
        assert_eq!(gm.get(15), 0);
        assert_eq!(gm.get(20), 5);
        assert_eq!(gm.get(50), 35);
        assert_eq!(gm.get(51), 36);
        assert_eq!(gm.get(52), 37);
        assert_eq!(gm.get(53), 38);
        assert_eq!(gm.get(54), 54);
        assert_eq!(gm.get(67), 67);
    }
}
