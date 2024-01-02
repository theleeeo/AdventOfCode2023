#![warn(clippy::all, clippy::pedantic)]

const INPUT: &str = include_str!("input.txt");

fn main() {
    let maps = create_maps(INPUT);

    let seeds = get_seeds(INPUT);
    let locations: Vec<u64> = seeds.iter().map(|seed| map_seed(*seed, &maps)).collect();
    println!(
        "smallest location of seeds: {}",
        locations.iter().min().unwrap()
    );

    let seed_ranges = get_seeds_as_ranges(INPUT);

    let mut location = 0;
    loop {
        let seed = map_location(location, &maps);

        let valid_seed = seed_ranges
            .iter()
            .any(|range| seed_exists_in_range(seed, range));

        if valid_seed {
            println!("smallest location: {location}");
            println! {"seed: {seed}"};
            break;
        }

        location += 1;
    }
}

fn seed_exists_in_range(seed: u64, range: &(u64, u64)) -> bool {
    seed >= range.0 && seed <= range.1
}

// Get the location of a seed
fn map_seed(seed: u64, maps: &Vec<Map>) -> u64 {
    let mut mapped_value = seed;
    for map in maps {
        mapped_value = map.map(mapped_value);
    }
    mapped_value
}

// Get the seed of a location
fn map_location(location: u64, maps: &[Map]) -> u64 {
    let mut mapped_value = location;

    let maps_reverse = maps.iter().rev().collect::<Vec<&Map>>();

    for map in maps_reverse {
        mapped_value = map.map_reverse(mapped_value);
    }
    mapped_value
}

// Returns a vector of ranges of valid seeds. The ranges are tuples of (start, end)
fn get_seeds_as_ranges(input: &str) -> Vec<(u64, u64)> {
    let seed_str = input
        .lines()
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap();

    let seed_values: Vec<u64> = seed_str
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut ranges: Vec<(u64, u64)> = Vec::new();
    for chunk in seed_values.chunks(2) {
        let seedrange_pair = match chunk.len() {
            2 => (chunk[0], chunk[1]),
            _ => panic!("Invalid number of seed values"),
        };

        let range_start = seedrange_pair.0;
        let range_length = seedrange_pair.1;

        ranges.push((range_start, range_start + range_length - 1));
    }

    ranges
}

fn get_seeds(input: &str) -> Vec<u64> {
    let seeds = input
        .lines()
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap();

    seeds
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn is_blank_line(line: &str) -> bool {
    line.trim().is_empty()
}

fn create_maps(input: &str) -> Vec<Map> {
    // Skip the seeds and the first blank line
    let iter = input.lines().skip(2);

    let mut maps = Vec::new();
    let mut map = Map { ranges: Vec::new() };
    for line in iter {
        if is_blank_line(line) {
            maps.push(map);
            map = Map { ranges: Vec::new() };
        } else if line.contains("map:") {
            continue;
        } else {
            map.ranges.push(Range::from(line));
        }
    }
    maps.push(map);

    maps
}

#[derive(Debug)]
struct Range {
    destination_start: u64,
    source_start: u64,
    range_length: u64,
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let values: Vec<u64> = value
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        Range {
            destination_start: values[0],
            source_start: values[1],
            range_length: values[2],
        }
    }
}

struct RangeIterator<'a> {
    range: &'a Range,
    current_value: u64,
}

impl<'a> Iterator for RangeIterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_value < self.range.range_length {
            let value = self.range.destination_start + self.current_value;
            self.current_value += 1;
            Some(value)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn map(&self, source: u64) -> u64 {
        let mut dest = source;
        for range in &self.ranges {
            if source >= range.source_start && source < range.source_start + range.range_length {
                dest = range.destination_start + (source - range.source_start);
                break;
            }
        }
        dest
    }

    fn map_reverse(&self, destination: u64) -> u64 {
        let mut source = destination;
        for range in &self.ranges {
            if destination >= range.destination_start
                && destination < range.destination_start + range.range_length
            {
                source = range.source_start + (destination - range.destination_start);
                break;
            }
        }
        source
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_seeds() {
        let seeds = get_seeds("seeds: 1 2 3 4 5");
        assert_eq!(seeds, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_create_maps() {
        let maps = create_maps("seeds: 1\n\nmap:\n1 2 3\n\nmap:\n7 8 9\n10 11 12");
        assert_eq!(maps.len(), 2);
        assert_eq!(maps[0].ranges.len(), 1);
        assert_eq!(maps[1].ranges.len(), 2);
        assert_eq!(maps[0].ranges[0].destination_start, 1);
        assert_eq!(maps[0].ranges[0].source_start, 2);
        assert_eq!(maps[0].ranges[0].range_length, 3);
    }

    #[test]
    fn test_map() {
        let map = Map {
            ranges: vec![
                Range {
                    destination_start: 1,
                    source_start: 2,
                    range_length: 3,
                },
                Range {
                    destination_start: 7,
                    source_start: 8,
                    range_length: 9,
                },
                Range {
                    destination_start: 10,
                    source_start: 11,
                    range_length: 12,
                },
            ],
        };
        assert_eq!(map.map(2), 1);
        assert_eq!(map.map(3), 2);
        assert_eq!(map.map(4), 3);
        assert_eq!(map.map(8), 7);
        assert_eq!(map.map(9), 8);
        assert_eq!(map.map(10), 9);
        assert_eq!(map.map(11), 10);
        assert_eq!(map.map(12), 11);
        // Out of range
        assert_eq!(map.map(99), 99);
    }

    #[test]
    fn test_range_from() {
        let range = Range::from("1 2 3");
        assert_eq!(range.destination_start, 1);
        assert_eq!(range.source_start, 2);
        assert_eq!(range.range_length, 3);
    }

    #[test]
    fn test_get_seeds_as_ranges() {
        let ranges = get_seeds_as_ranges("seeds: 1 2 3 4");
        assert_eq!(ranges, vec![(1, 2), (3, 6),]);

        let ranges = get_seeds_as_ranges("seeds: 1 2 3 4 5 6");
        assert_eq!(ranges, vec![(1, 2), (3, 6), (5, 10),]);
    }
}
