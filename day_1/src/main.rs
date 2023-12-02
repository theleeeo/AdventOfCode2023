#![warn(clippy::all, clippy::pedantic)]

const INPUT: &str = include_str!("input.txt");

const INT_NAMES: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let total: u32 = INPUT
        .lines()
        .map(|line| get_first_integer(line) * 10 + get_last_integer(line))
        .sum();
    println!("Total: {total}");
}

fn get_first_integer(line: &str) -> u32 {
    let iter = line.char_indices();
    for (i, c) in iter {
        if c.is_ascii_digit() {
            return c.to_digit(10).unwrap();
        }

        if c.is_ascii_alphabetic() {
            let name = &line[i..line.len()];
            for (j, int_name) in INT_NAMES.iter().enumerate() {
                if name.len() < int_name.len() {
                    continue;
                }
                if int_name.starts_with(&name[0..int_name.len()]) {
                    return u32::try_from(j).unwrap();
                }
            }
        }
    }

    panic!("No digit found in line: {line}")
}

fn get_last_integer(line: &str) -> u32 {
    let iter = line.char_indices();
    for (i, c) in iter.rev() {
        if c.is_ascii_digit() {
            return c.to_digit(10).unwrap();
        }

        if c.is_ascii_alphabetic() {
            let name = reverse_string(&line[0..=i]);
            for (j, int_name) in INT_NAMES.iter().enumerate() {
                let rev_name = reverse_string(int_name);
                if name.len() < rev_name.len() {
                    continue;
                }
                if rev_name.starts_with(&name[0..rev_name.len()]) {
                    return u32::try_from(j).unwrap();
                }
            }
        }
    }

    panic!("No digit found in line: {line}")
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_integer() {
        assert_eq!(get_first_integer("one"), 1);
        assert_eq!(get_first_integer("jkkjtwoghthreeghgh"), 2);
        assert_eq!(get_first_integer("jfdkj4hdj"), 4);
        assert_eq!(get_first_integer("hej1hej2hej"), 1);
    }

    #[test]
    fn test_get_last_integer() {
        assert_eq!(get_last_integer("one"), 1);
        assert_eq!(get_last_integer("jkkjtwoghthreeghgh"), 3);
        assert_eq!(get_last_integer("jfdkj4hdj"), 4);
        assert_eq!(get_last_integer("hej1hej2hej"), 2);
    }
}
