#![warn(clippy::all, clippy::pedantic)]

const INPUT: &str = include_str!("input.txt");

fn main() {
    let total: u32 = INPUT
        .lines()
        .map(|line| get_first_integer(line) * 10 + get_last_integer(line))
        .sum();
    println!("Total: {total}");
}

fn get_first_integer(line: &str) -> u32 {
    let iter = line.chars();
    for c in iter {
        if c.is_ascii_digit() {
            return c.to_digit(10).unwrap();
        }
    }

    panic!("No digit found in line: {line}")
}

fn get_last_integer(line: &str) -> u32 {
    let iter = line.chars();
    for c in iter.rev() {
        if c.is_ascii_digit() {
            return c.to_digit(10).unwrap();
        }
    }

    panic!("No digit found in line: {line}")
}
