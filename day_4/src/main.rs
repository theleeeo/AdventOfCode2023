use scanf::sscanf;
use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut cards = Vec::new();
    for line in INPUT.lines() {
        cards.push(Card::new(line));
    }

    let mut total_score = 0;
    for card in cards.iter() {
        total_score += card.calculate_score();
    }
    println!("Total score: {}", total_score);

    //
    // Part 2
    //

    // The total numbers of cards won for each card
    let mut cards_won_cache = vec![0; cards.len()];

    for (index, card) in cards.iter_mut().enumerate().rev() {
        let won_ids = card.get_won_ids();
        let mut additional_cards: u32 = 0;
        print!("Card {} won ids: {:?}", card.id, won_ids);
        for id in &won_ids {
            additional_cards += cards_won_cache[(id - 1) as usize];
        }
        cards_won_cache[index] = additional_cards + 1;
        println!(" => {}", cards_won_cache[index]);
    }

    let total_won = cards_won_cache.iter().sum::<u32>();
    println!("Total won: {}", total_won);
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

    fn get_winning_number_count(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|&n| self.is_winning_number(*n))
            .count() as u32
    }

    fn get_won_ids(&self) -> Vec<u32> {
        let mut won_ids = Vec::new();
        let won_count = self.get_winning_number_count();

        for i in 1..=won_count {
            won_ids.push(self.id + i);
        }
        won_ids
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seperate_card_id() {
        let (id, rest) = seperate_card_id("Card 1: 1 2 3 | 4 5 6");
        assert_eq!(id, 1);
        assert_eq!(rest, " 1 2 3 | 4 5 6");
    }

    #[test]
    fn test_seperate_card_numbers() {
        let (winning_numbers, numbers) = seperate_card_numbers("1 2 3 | 4 5 6");
        assert_eq!(winning_numbers, "1 2 3");
        assert_eq!(numbers, "4 5 6");
    }

    #[test]
    fn test_parse_winning_numbers() {
        let winning_numbers = parse_winning_numbers("1 2 3");
        assert_eq!(winning_numbers.len(), 3);
        assert!(winning_numbers.contains(&1));
        assert!(winning_numbers.contains(&2));
        assert!(winning_numbers.contains(&3));
        assert!(!winning_numbers.contains(&4));
    }

    #[test]
    fn test_parse_numbers() {
        let numbers = parse_numbers("1 2 3");
        assert_eq!(numbers.len(), 3);
        assert_eq!(numbers[0], 1);
        assert_eq!(numbers[1], 2);
        assert_eq!(numbers[2], 3);
    }

    #[test]
    fn test_is_winning_number() {
        let card = Card::new("Card 1: 1 2 3 | 4 5 6");
        assert!(card.is_winning_number(1));
        assert!(card.is_winning_number(2));
        assert!(card.is_winning_number(3));
        assert!(!card.is_winning_number(4));
        assert!(!card.is_winning_number(5));
        assert!(!card.is_winning_number(6));
    }

    #[test]
    fn test_calculate_score() {
        let card = Card::new("Card 1: 1 2 3 | 4 5 6");
        assert_eq!(card.calculate_score(), 0);

        let card = Card::new("Card 1: 1 2 3 | 1 2 4");
        assert_eq!(card.calculate_score(), 2);

        let card = Card::new("Card 1: 1 2 3 | 1 2 3 4");
        assert_eq!(card.calculate_score(), 4);
    }

    #[test]
    fn test_get_winning_number_count() {
        let card = Card::new("Card 1: 1 2 3 | 4 5 6");
        assert_eq!(card.get_winning_number_count(), 0);

        let card = Card::new("Card 1: 1 2 3 | 1 2 4");
        assert_eq!(card.get_winning_number_count(), 2);

        let card = Card::new("Card 1: 1 2 3 | 1 2 3 4");
        assert_eq!(card.get_winning_number_count(), 3);
    }

    #[test]
    fn test_get_won_ids() {
        let card = Card::new("Card 1: 1 2 3 | 4 5 6");
        assert_eq!(card.get_won_ids().len(), 0);

        let card = Card::new("Card 1: 1 2 3 | 1 2 4");
        assert_eq!(card.get_won_ids().len(), 1);
        assert_eq!(card.get_won_ids()[0], 2);

        let card = Card::new("Card 1: 1 2 3 | 1 2 3 4");
        assert_eq!(card.get_won_ids().len(), 2);
        assert_eq!(card.get_won_ids()[0], 2);
        assert_eq!(card.get_won_ids()[1], 3);
    }
}
