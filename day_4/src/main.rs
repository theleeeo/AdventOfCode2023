use scanf::sscanf;
use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut cards = Vec::new();
    for line in INPUT.lines() {
        cards.push(Card::new(line));
    }

    let mut total_score = 0;
    for card in cards {
        total_score += card.calculate_score();
    }

    println!("Total score: {}", total_score);
}

fn seperate_card_id(card: &str) -> (u32, &str) {
    let parts: Vec<&str> = card.split(":").collect();
    if parts.len() != 2 {
        panic!("Invalid card format");
    }

    let mut id: u32 = 0;
    sscanf!(parts[0], "Card {}", id).unwrap();
    (id, parts[1])
}

fn seperate_card_numbers(card: &str) -> (&str, &str) {
    let parts: Vec<&str> = card.split("|").collect();
    if parts.len() != 2 {
        panic!("Invalid card format");
    }

    (parts[0].trim(), parts[1].trim())
}

fn parse_winning_numbers(numbers: &str) -> HashSet<u32> {
    let mut winning_numbers = HashSet::new();

    for number in numbers.split_whitespace() {
        winning_numbers.insert(number.parse::<u32>().unwrap());
    }
    winning_numbers
}

fn parse_numbers(numbers: &str) -> Vec<u32> {
    let mut numbers_vec = Vec::new();

    for number in numbers.split_whitespace() {
        numbers_vec.push(number.parse::<u32>().unwrap());
    }
    numbers_vec
}

struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn new(s: &str) -> Self {
        let (card_number, rest) = seperate_card_id(s);
        let (winning_numbers, numbers) = seperate_card_numbers(rest);

        let winning_numbers = parse_winning_numbers(winning_numbers);
        let numbers = parse_numbers(numbers);

        Self {
            id: card_number,
            winning_numbers,
            numbers,
        }
    }

    fn is_winning_number(&self, number: u32) -> bool {
        self.winning_numbers.contains(&number)
    }

    fn calculate_score(&self) -> u32 {
        let mut score = 0;
        for number in &self.numbers {
            if self.is_winning_number(*number) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        score
    }
}
