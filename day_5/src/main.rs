const INPUT: &str = include_str!("input.txt");

fn main() {
    let seeds = get_seeds(INPUT);
    // println!("seeds: {:?}", seeds);

    let maps = create_maps(INPUT);
    // println!("maps: {:?}", maps);

    let locations: Vec<u64> = seeds
        .iter()
        .map(|seed| {
            let mut mapped_value = *seed;
            for map in &maps {
                mapped_value = map.map(mapped_value);
                // println!("mapped_value: {}", mapped_value);
            }
            mapped_value
        })
        .collect();

    println!("smallest location: {}", locations.iter().min().unwrap());
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
}
