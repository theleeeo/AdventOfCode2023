#![warn(clippy::all, clippy::pedantic)]

const INPUT: &str = include_str!("input.txt");

fn main() {
    let histories = parse_histories(INPUT);

    let mut sum_next = 0;
    let mut sum_prev = 0;
    for history in histories {
        // diffs will be the structure containing the history first, then all of the differences
        let diffs = get_all_diffs(history);

        let prev = calculate_previous(diffs.clone());
        sum_prev += prev;

        let next = calculate_next(diffs);
        sum_next += next;
    }

    println!("Sum next: {}", sum_next);
    println!("Sum prev: {}", sum_prev);
}

fn parse_histories(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(parse_history).collect::<Vec<_>>()
}

fn parse_history(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn calculate_differences(history: &[i32]) -> Vec<i32> {
    let mut differences = Vec::new();
    for i in 0..history.len() - 1 {
        differences.push(history[i + 1] - history[i]);
    }
    differences
}

fn is_constant(differences: &[i32]) -> bool {
    differences.iter().all(|&x| x == differences[0])
}

fn get_all_diffs(history: Vec<i32>) -> Vec<Vec<i32>> {
    let mut diffs = vec![history];
    while !is_constant(diffs.last().unwrap()) {
        let diff = calculate_differences(diffs.last().unwrap());
        diffs.push(diff);
    }

    diffs
}

fn calculate_previous(diffs: Vec<Vec<i32>>) -> i32 {
    let mut diffs = diffs.clone();
    for i in 0..diffs.len() {
        diffs[i].reverse();
    }

    for i in (0..diffs.len() - 1).rev() {
        // The difference tp use when calculating the next value
        let next_diff = diffs[i + 1].last().unwrap();

        // The last element of the current vector
        let current_last = diffs[i].last().unwrap();

        let new_val = current_last - next_diff;
        diffs[i].push(new_val)
    }

    // Return the last element of the first vector
    diffs[0][diffs[0].len() - 1]
}

fn calculate_next(mut diffs: Vec<Vec<i32>>) -> i32 {
    for i in (0..diffs.len() - 1).rev() {
        // The difference tp use when calculating the next value
        let next_diff = diffs[i + 1].last().unwrap();

        // The last element of the current vector
        let current_last = diffs[i].last().unwrap();

        let new_val = current_last + next_diff;
        diffs[i].push(new_val)
    }

    // Return the last element of the first vector
    diffs[0][diffs[0].len() - 1]
}
