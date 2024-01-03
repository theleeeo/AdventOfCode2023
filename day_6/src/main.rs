#![warn(clippy::all, clippy::pedantic)]

const INPUT: &str = include_str!("input.txt");
const ACCELERATION: u32 = 1;

fn main() {
    let races = parse_races(INPUT);

    let mut product_of_ways_to_win = 1;
    for race in &races {
        product_of_ways_to_win *= nr_of_ways_to_win(race);
    }

    println!("Product of ways to win: {product_of_ways_to_win}");
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

fn parse_times(input: &str) -> Vec<u32> {
    let input = input.strip_prefix("Time:").unwrap().trim();

    let mut times: Vec<u32> = Vec::new();
    for line in input.split_whitespace() {
        let time: u32 = line.parse().unwrap();
        times.push(time);
    }
    times
}

fn parse_distances(input: &str) -> Vec<u32> {
    let input = input.strip_prefix("Distance:").unwrap().trim();

    let mut distances: Vec<u32> = Vec::new();
    for line in input.split_whitespace() {
        let distance: u32 = line.parse().unwrap();
        distances.push(distance);
    }
    distances
}

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

fn nr_of_ways_to_win(race: &Race) -> u32 {
    let optimal_time = time_for_max_distance(race.time);

    let mut time_to_test = optimal_time;
    // Based on the question i assume that the optiomal time will always win
    let mut ways_to_win = 1;

    // We start at the optimal time and go up until we find the maximum time to hold the acceleration button
    loop {
        time_to_test += 1;

        // We have found the maximum time to hold the acceleration button
        if distance_traveled(race.time, time_to_test) <= race.distance {
            break;
        }

        println!(
            "Time: {}, distance: {}",
            time_to_test,
            distance_traveled(race.time, time_to_test)
        );
        ways_to_win += 1;
    }

    time_to_test = optimal_time;
    // Now we go down until we find the minumum time to hold the acceleration button
    loop {
        time_to_test -= 1;

        // We have found the maximum time to hold the acceleration button
        if distance_traveled(race.time, time_to_test) <= race.distance {
            break;
        }

        println!(
            "Time: {}, distance: {}",
            time_to_test,
            distance_traveled(race.time, time_to_test)
        );
        ways_to_win += 1;
    }

    ways_to_win
}

// This is the time that the acceleration button should be held down for it to travel the maximum distance
// This is carculated using the derivative of the distance function to get the maximum of the distance function
fn time_for_max_distance(max_time: u32) -> u32 {
    max_time / 2
}

fn distance_traveled(max_time: u32, time_held: u32) -> u32 {
    let speed = time_held * ACCELERATION;
    let time_left = max_time - time_held;
    speed * time_left
}
