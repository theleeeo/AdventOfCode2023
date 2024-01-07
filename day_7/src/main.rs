#![warn(clippy::all, clippy::pedantic)]

use std::cmp;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut hands: Vec<Hand> = INPUT.lines().map(Hand::parse).collect();
    hands.sort_by(|a, b| a.order_by_cards(b));

    let mut winnings = 0;
    for (i, hand) in hands.iter().enumerate() {
        winnings += hand.bid * (i + 1) as u32;
    }

    println!("Winnings: {}", winnings);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<u32>,
    bid: u32,
    hand_type: Type,
}

impl Hand {
    fn parse(s: &str) -> Self {
        let parts: Vec<&str> = s.split_whitespace().collect();
        assert!(parts.len() == 2, "Invalid hand: {s}");

        let cards = parse_cards(parts[0]);
        let bid = parts[1].parse().unwrap();
        let hand_type = parse_type(&cards);

        Self {
            cards,
            bid,
            hand_type,
        }
    }

    fn order_by_cards(&self, other: &Self) -> cmp::Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        }

        self.cards.cmp(&other.cards)
    }
}

fn parse_type(cards: &[u32]) -> Type {
    let mut counts = [0; 15];
    for card in cards {
        counts[*card as usize] += 1;
    }

    let mut counts = counts.iter().filter(|&&c| c > 0).collect::<Vec<_>>();
    counts.sort_by(|a, b| b.cmp(a));

    match counts.as_slice() {
        [1, 1, 1, 1, 1] => Type::HighCard,
        [2, 1, 1, 1] => Type::OnePair,
        [2, 2, 1] => Type::TwoPair,
        [3, 1, 1] => Type::ThreeOfAKind,
        [3, 2] => Type::FullHouse,
        [4, 1] => Type::FourOfAKind,
        [5] => Type::FiveOfAKind,
        _ => panic!("Invalid hand: {cards:?}"),
    }
}

fn parse_cards(s: &str) -> Vec<u32> {
    s.chars()
        .map(|s| match s {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => s.to_digit(10).unwrap(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cards() {
        let input = "87A9T";
        let hand = parse_cards(input);
        assert_eq!(hand, vec![8, 7, 14, 9, 10]);

        let input = "123456789TJQKA";
        let hand = parse_cards(input);
        assert_eq!(hand, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
    }

    #[test]
    fn test_parse_type() {
        let input = "87A9T";
        let hand = parse_cards(input);
        assert_eq!(parse_type(&hand), Type::HighCard);

        let input = "55555";
        let hand = parse_cards(input);
        assert_eq!(parse_type(&hand), Type::FiveOfAKind);

        let input = "4444A";
        let hand = parse_cards(input);
        assert_eq!(parse_type(&hand), Type::FourOfAKind);

        let input = "33322";
        let hand = parse_cards(input);
        assert_eq!(parse_type(&hand), Type::FullHouse);

        let input = "22AAA";
        let hand = parse_cards(input);
        assert_eq!(parse_type(&hand), Type::FullHouse);

        let input = "12TTT";
        let hand = parse_cards(input);
        assert_eq!(parse_type(&hand), Type::ThreeOfAKind);

        let input = "225QQ";
        let hand = parse_cards(input);
        assert_eq!(parse_type(&hand), Type::TwoPair);

        let input = "22KJA";
        let hand = parse_cards(input);
        assert_eq!(parse_type(&hand), Type::OnePair);
    }

    #[test]
    fn test_parse_hand() {
        let input = "87A9T 1";
        let hand = Hand::parse(input);
        assert_eq!(
            hand,
            Hand {
                cards: vec![8, 7, 14, 9, 10],
                bid: 1,
                hand_type: Type::HighCard,
            }
        );

        let input = "88AAT 2";
        let hand = Hand::parse(input);
        assert_eq!(
            hand,
            Hand {
                cards: vec![8, 8, 14, 14, 10],
                bid: 2,
                hand_type: Type::TwoPair,
            }
        );
    }

    #[test]
    fn test_order_by_cards() {
        let hand1 = Hand::parse("87A9T 1");
        let hand2 = Hand::parse("88AAT 2");
        assert_eq!(hand1.order_by_cards(&hand2), cmp::Ordering::Less);
        assert_eq!(hand2.order_by_cards(&hand1), cmp::Ordering::Greater);

        let hand1 = Hand::parse("87A9T 1");
        let hand2 = Hand::parse("87A9T 2");
        assert_eq!(hand1.order_by_cards(&hand2), cmp::Ordering::Equal);
        assert_eq!(hand2.order_by_cards(&hand1), cmp::Ordering::Equal);

        let hand1 = Hand::parse("87A9T 1");
        let hand2 = Hand::parse("87A9J 2");
        assert_eq!(hand1.order_by_cards(&hand2), cmp::Ordering::Less);
        assert_eq!(hand2.order_by_cards(&hand1), cmp::Ordering::Greater);

        let hand1 = Hand::parse("77777 1");
        let hand2 = Hand::parse("88888 1");
        assert_eq!(hand1.order_by_cards(&hand2), cmp::Ordering::Less);
        assert_eq!(hand2.order_by_cards(&hand1), cmp::Ordering::Greater);
    }
}
