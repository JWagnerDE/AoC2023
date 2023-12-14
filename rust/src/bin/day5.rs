// --- Day 5: If You Give A Seed A Fertilizer ---

use std::env;
use std::fs;

use std::str::FromStr;
use std::ops::Range;

#[derive(Debug, PartialEq, Eq)]
struct GardenMap {
    destination: String,
    source: String,
    ranges: Vec<(u64, u64, u64)>,
}

impl GardenMap {
    fn get(&self, source: u64) -> u64 {
        for range in &self.ranges {
            if source >= range.1 && source < range.1+range.2 {
                return (range.0 + source) - range.1
            }
        }
        source
    }

    
    fn rev_get(&self, destination: u64) -> u64 {
        for range in &self.ranges {
            if destination >= range.0 && destination < range.0+range.2 {
                return (range.1 + destination) - range.0
            }
        }
        destination
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
        let mut ranges = Vec::new();
        for line in lines {
            let mut parts = line.split_whitespace();
            let dest_range_start = parts
                .next()
                .ok_or(ParseGardenMapErr)?
                .parse::<u64>()
                .map_err(|_| ParseGardenMapErr)?;
            let src_range_start = parts
                .next()
                .ok_or(ParseGardenMapErr)?
                .parse::<u64>()
                .map_err(|_| ParseGardenMapErr)?;
            let range_len = parts
                .next()
                .ok_or(ParseGardenMapErr)?
                .parse::<u64>()
                .map_err(|_| ParseGardenMapErr)?;
            ranges.push((dest_range_start, src_range_start, range_len));
        }
        Ok(Self {
            destination: destination.to_owned(),
            source: source.to_owned(),
            ranges,
        })
    }
}

fn puzzle1(data: &str) -> i32 {
    let mut parts = data.split("\n\n");
    let seed_line = parts.next().unwrap();
    let seeds = seed_line
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .collect::<Vec<u64>>();
    let garden_maps = parts
        .map(GardenMap::from_str)
        .map(Result::unwrap)
        .collect::<Vec<GardenMap>>();
    let locations: Vec<u64> = seeds
        .iter()
        .map(|seed| garden_maps
             .iter()
             .fold(*seed, |source, garden_map| garden_map.get(source))
             )
        .collect();
    let min = *locations.iter().min().unwrap();

    min as i32
}

fn puzzle2(data: &str) -> i32 {
    let mut parts = data.split("\n\n");
    let seed_line = parts.next().unwrap();
    let seed_ranges = seed_line
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|chunk| {
            let start= chunk[0];
            let len= chunk[1];
            start..start+len
        })
        .collect::<Vec<Range<u64>>>();
    let garden_maps = parts
        .map(GardenMap::from_str)
        .map(Result::unwrap)
        .collect::<Vec<GardenMap>>();
    let mut i = 0;
    loop {
        let seed_to_test = garden_maps
            .iter()
            .rev()
            .fold(i, |destination, garden_map| {
                garden_map.rev_get(destination)
            });
        for seed_range in &seed_ranges {
            if seed_to_test >= seed_range.start
            && seed_to_test < seed_range.end {
                return i as i32
            }
        }
        i += 1;
    }
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
        let res = puzzle2(test_data);
        assert_eq!(res, 46);
    }

    #[test]
    fn test_gardenmap_parse() {
        let input = "\
                     seed-to-soil map:\n\
                     50 98 2\n\
                     52 50 48";
        let mut ranges = Vec::new();
        ranges.push((50, 98, 2));
        ranges.push((52, 50, 48));
        let res = GardenMap {
            destination: "soil".to_owned(),
            source: "seed".to_owned(),
            ranges,
        };
        assert_eq!(GardenMap::from_str(input).unwrap(), res);
        let input = "\
                     soil-to-fertilizer map:\n\
                     0 15 37\n\
                     37 52 2\n\
                     39 0 15";
        let mut ranges = Vec::new();
        ranges.push((0, 15, 37));
        ranges.push((37, 52, 2));
        ranges.push((39, 0, 15));
        let res = GardenMap {
            destination: "fertilizer".to_owned(),
            source: "soil".to_owned(),
            ranges,
        };
        assert_eq!(GardenMap::from_str(input).unwrap(), res);
        let input = "\
                     fertilizer-to-water map:\n\
                     49 53 8\n\
                     0 11 42\n\
                     42 0 7\n\
                     57 7 4";
        let mut ranges = Vec::new();
        ranges.push((49, 53, 8));
        ranges.push((0, 11, 42));
        ranges.push((42, 0, 7));
        ranges.push((57, 7,4));
        let res = GardenMap {
            destination: "water".to_owned(),
            source: "fertilizer".to_owned(),
            ranges,
        };
        assert_eq!(GardenMap::from_str(input).unwrap(), res);
    }

    #[test]
    fn test_gardenmap_get() {
        let mut ranges = Vec::new();
        ranges.push((50, 98, 2));
        ranges.push((52, 50, 48));
        let gm = GardenMap {
            destination: "soil".to_owned(),
            source: "seed".to_owned(),
            ranges,
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

        let mut ranges = Vec::new();
        ranges.push((0, 15, 37));
        ranges.push((37, 52, 2));
        ranges.push((39, 0, 15));
        let gm = GardenMap {
            destination: "fertilizer".to_owned(),
            source: "soil".to_owned(),
            ranges,
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
