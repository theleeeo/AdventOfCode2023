#![warn(clippy::all, clippy::pedantic)]

const INPUT: &str = include_str!("input.txt");
const ACCELERATION: u64 = 1;

fn main() {
    let races = parse_races(INPUT);

    let mut product_of_ways_to_win = 1;
    for race in &races {
        product_of_ways_to_win *= nr_of_ways_to_win(race);
    }

    println!("Product of ways to win: {product_of_ways_to_win}");

    let race = parse_race(INPUT);
    println!("Ways to win: {}", nr_of_ways_to_win(&race));
}

fn parse_race(input: &str) -> Race {
    let lines: Vec<&str> = input.lines().collect();
    assert!(lines.len() == 2, "Invalid input, expected 2 lines");

    Race {
        time: parse_line_as_digits(lines[0].strip_prefix("Time:").unwrap()),
        distance: parse_line_as_digits(lines[1].strip_prefix("Distance:").unwrap()),
    }
}

fn parse_line_as_digits(input: &str) -> u64 {
    let mut number = 0;
    let mut current_digit = 0;
    for c in input.chars().rev() {
        if c.is_ascii_whitespace() {
            continue;
        }
        number += c
            .to_digit(10)
            .expect(format!("char {c} not a digit").as_str()) as u64
            * 10_u64.pow(current_digit);
        current_digit += 1;
    }
    number
}

fn parse_races(input: &str) -> Vec<Race> {
    let lines: Vec<&str> = input.lines().collect();
    assert!(lines.len() == 2, "Invalid input, expected 2 lines");

    let times = parse_times(lines[0]);
    let distances = parse_distances(lines[1]);
    assert!(
        (times.len() == distances.len()),
        "Invalid input, expected same number of times and distances"
    );

    let iter = times.iter().zip(distances.iter());

    iter.map(|v| Race {
        time: *v.0,
        distance: *v.1,
    })
    .collect()
}

fn parse_times(input: &str) -> Vec<u64> {
    let input = input.strip_prefix("Time:").unwrap().trim();

    let mut times: Vec<u64> = Vec::new();
    for line in input.split_whitespace() {
        let time: u64 = line.parse().unwrap();
        times.push(time);
    }
    times
}

fn parse_distances(input: &str) -> Vec<u64> {
    let input = input.strip_prefix("Distance:").unwrap().trim();

    let mut distances: Vec<u64> = Vec::new();
    for line in input.split_whitespace() {
        let distance: u64 = line.parse().unwrap();
        distances.push(distance);
    }
    distances
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn nr_of_ways_to_win(race: &Race) -> u64 {
    let optimal_time = time_for_max_distance(race.time);

    let mut lower_bound = optimal_time;
    let mut upper_bound = race.time;

    // We start at the optimal time and do a binary search towards the max time to find the max winning value
    let max_winning_value;
    loop {
        let time_to_test = (lower_bound + upper_bound) / 2;

        // We have found the maximum time to hold the acceleration button
        if distance_traveled(race.time, time_to_test) <= race.distance {
            upper_bound = time_to_test;
        } else {
            lower_bound = time_to_test;
        }

        println!(
            "Time: {}, distance: {}",
            time_to_test,
            distance_traveled(race.time, time_to_test)
        );

        if upper_bound - lower_bound <= 1 {
            max_winning_value = upper_bound;
            break;
        }
    }

    lower_bound = 0;
    upper_bound = optimal_time;
    // Now we go down until we find the minumum time to hold the acceleration button
    let min_winning_value;
    loop {
        let time_to_test = (lower_bound + upper_bound) / 2;

        // We have found the maximum time to hold the acceleration button
        if distance_traveled(race.time, time_to_test) <= race.distance {
            lower_bound = time_to_test;
        } else {
            upper_bound = time_to_test;
        }

        println!(
            "Time: {}, distance: {}",
            time_to_test,
            distance_traveled(race.time, time_to_test)
        );

        if upper_bound - lower_bound <= 1 {
            min_winning_value = upper_bound;
            break;
        }
    }

    max_winning_value - min_winning_value
}

// This is the time that the acceleration button should be held down for it to travel the maximum distance
// This is carculated using the derivative of the distance function to get the maximum of the distance function
fn time_for_max_distance(max_time: u64) -> u64 {
    max_time / 2
}

fn distance_traveled(max_time: u64, time_held: u64) -> u64 {
    let speed = time_held * ACCELERATION;
    let time_left = max_time - time_held;
    speed * time_left
}
