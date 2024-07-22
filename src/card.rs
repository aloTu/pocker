use rand::seq::SliceRandom;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suit = match self {
            Suit::Clubs => "♣",
            Suit::Diamonds => "♦",
            Suit::Hearts => "♥",
            Suit::Spades => "♠",
        };
        write!(f, "{}", suit)
    }
}

impl From<&str> for Suit {
    fn from(s: &str) -> Self {
        match s {
            "♣" => Suit::Clubs,
            "♦" => Suit::Diamonds,
            "♥" => Suit::Hearts,
            "♠" => Suit::Spades,
            &_ => panic!("Invalid suit"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub rank: u8,
    pub suit: Suit,
}

impl Card {
    pub fn new(rank: u8, suit: Suit) -> Self {
        Self { rank, suit }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rank = match self.rank {
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            14 => "A".to_string(),
            _ => self.rank.to_string(),
        };
        write!(f, "{}{}", rank, self.suit)
    }
}

impl From<&str> for Card {
    fn from(s: &str) -> Self {
        let parts = s.chars();
        if parts.count() != 2 {
            panic!("Invalid card");
        }
        let suit = s.chars().nth(0).unwrap().to_string();
        let rank = s.chars().nth(1).unwrap().to_string();

        let suit = Suit::from(suit.as_str());
        let rank = match rank.as_str() {
            "A" => 14,
            "K" => 13,
            "Q" => 12,
            "J" => 11,
            "X" => 10,
            num => num.parse().unwrap(),
        };
        Card { rank, suit }
    }
}

pub fn parse_cards(input: &str) -> Vec<Card> {
    input.split(';').map(|s| Card::from(s)).collect()
}

pub struct Deck {
    all_cards: Vec<Card>,
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);
        for suit in &[Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
            for rank in 2..=14 {
                cards.push(Card::new(rank, *suit));
            }
        }
        let all_cards = cards.clone();
        Self { all_cards, cards }
    }

    pub fn shuffle(&mut self) {
        if self.cards.len() < 52 {
            self.cards = self.all_cards.clone();
        }
        let mut rng = rand::thread_rng();
        self.cards.as_mut_slice().shuffle(&mut rng);
    }

    pub fn deal(&mut self) -> Card {
        self.cards.pop().unwrap()
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_creation() {
        let deck = Deck::new();
        assert_eq!(deck.cards.len(), 52);
    }
    #[test]
    fn test_card_creation() {
        let card = Card::new(10, Suit::Hearts);
        assert_eq!(card, Card::from("♥X"));
    }

    #[test]
    fn test_shuffle() {
        let mut deck = Deck::new();
        let original_deck = deck.cards.clone();
        deck.shuffle();
        assert_ne!(deck.cards, original_deck);
    }

    #[test]
    fn test_deal() {
        let mut deck = Deck::new();
        deck.deal();
        assert_eq!(deck.cards.len(), 51);
    }
}
