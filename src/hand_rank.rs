use crate::card::{parse_cards, Card};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandRank {
    HighCard(u8, u8, u8, u8, u8),
    OnePair(u8, u8, u8, u8),
    TwoPair(u8, u8, u8),
    ThreeOfAKind(u8, u8, u8),
    Straight(u8),
    Flush(u8, u8, u8, u8, u8),
    FullHouse(u8, u8),
    FourOfAKind(u8, u8),
    StraightFlush(u8),
    RoyalFlush,
}

impl HandRank {
    pub fn from_cards_str(s: &str) -> Self {
        let cards = parse_cards(s);
        HandRank::from_cards(&cards)
    }

    pub fn from_cards(cards: &[Card]) -> Self {
        let mut ranks: Vec<u8> = cards.iter().map(|card| card.rank).collect();
        ranks.sort_unstable_by(|a, b| b.cmp(a));

        let is_flush = cards.iter().all(|card| card.suit == cards[0].suit);
        let is_straight = HandRank::is_straight(&ranks);

        if is_flush && is_straight {
            return if ranks[0] == 14 {
                if ranks[1] == 13 {
                    HandRank::RoyalFlush
                } else {
                    HandRank::StraightFlush(ranks[1])
                }
            } else {
                HandRank::StraightFlush(ranks[0])
            };
        }

        let mut rank_counts = [0; 15];
        for &rank in &ranks {
            rank_counts[rank as usize] += 1;
        }

        let mut four_of_a_kind = None;
        let mut three_of_a_kind = None;
        let mut pairs = vec![];

        for (rank, &count) in rank_counts.iter().enumerate().rev() {
            match count {
                4 => four_of_a_kind = Some(rank as u8),
                3 => three_of_a_kind = Some(rank as u8),
                2 => pairs.push(rank as u8),
                _ => {}
            }
        }

        if let Some(four) = four_of_a_kind {
            return HandRank::FourOfAKind(
                four,
                ranks.iter().find(|&&r| r != four).cloned().unwrap(),
            );
        }

        if let Some(three) = three_of_a_kind {
            if !pairs.is_empty() {
                return HandRank::FullHouse(three, pairs[0]);
            }
            let the_rest = ranks
                .iter()
                .filter(|&&r| r != three)
                .cloned()
                .collect::<Vec<_>>();
            return HandRank::ThreeOfAKind(three, the_rest[0], the_rest[1]);
        }

        if pairs.len() > 1 {
            return HandRank::TwoPair(
                pairs[0],
                pairs[1],
                ranks
                    .iter()
                    .find(|&&r| r != pairs[0] && r != pairs[1])
                    .cloned()
                    .unwrap(),
            );
        }

        if pairs.len() == 1 {
            let the_rest = ranks
                .iter()
                .filter(|&&r| r != pairs[0])
                .cloned()
                .collect::<Vec<_>>();
            return HandRank::OnePair(pairs[0], the_rest[0], the_rest[1], the_rest[2]);
        }

        if is_flush {
            return HandRank::Flush(ranks[0], ranks[1], ranks[2], ranks[3], ranks[4]);
        }

        if is_straight {
            return if ranks[0] == 14 && ranks[1] == 5 {
                HandRank::Straight(ranks[1])
            } else {
                HandRank::Straight(ranks[0])
            };
        }

        HandRank::HighCard(ranks[0], ranks[1], ranks[2], ranks[3], ranks[4])
    }

    fn is_straight(ranks: &[u8]) -> bool {
        if ranks.windows(2).all(|w| w[0] == w[1] + 1) {
            return true;
        }
        if ranks == [14, 5, 4, 3, 2] {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Suit;

    #[test]
    fn test_card_type() {
        assert_eq!(
            HandRank::from_cards_str("♥X;♥J;♥Q;♥K;♥A"),
            HandRank::RoyalFlush
        );
        assert_eq!(
            HandRank::from_cards_str("♥9;♥X;♥J;♥Q;♥K"),
            HandRank::StraightFlush(13)
        );
        assert_eq!(
            HandRank::from_cards_str("♥9;♠9;♦9;♣9;♥K"),
            HandRank::FourOfAKind(9, 13)
        );
        assert_eq!(
            HandRank::from_cards_str("♥9;♠9;♦9;♣K;♥K"),
            HandRank::FullHouse(9, 13)
        );
        assert_eq!(
            HandRank::from_cards_str("♥2;♥5;♥7;♥9;♥K"),
            HandRank::Flush(13, 9, 7, 5, 2)
        );
        assert_eq!(
            HandRank::from_cards_str("♥9;♥X;♠J;♥Q;♦K"),
            HandRank::Straight(13)
        );
        assert_eq!(
            HandRank::from_cards_str("♥2;♥A;♠3;♥4;♥5"),
            HandRank::Straight(5)
        );
        assert_eq!(
            HandRank::from_cards_str("♥7;♠2;♦2;♣2;♥K"),
            HandRank::ThreeOfAKind(2, 13, 7)
        );
        assert_eq!(
            HandRank::from_cards_str("♥7;♠2;♦2;♥7;♥K"),
            HandRank::TwoPair(7, 2, 13)
        );
        assert_eq!(
            HandRank::from_cards_str("♥7;♠2;♦2;♥8;♥K"),
            HandRank::OnePair(2, 13, 8, 7)
        );
        assert_eq!(
            HandRank::from_cards_str("♥7;♠2;♦4;♥8;♦K"),
            HandRank::HighCard(13, 8, 7, 4, 2)
        );
    }

    #[test]
    fn test_rank() {
        let ranks = [
            "♥X;♥J;♥Q;♥K;♥A",
            "♦7;♦8;♦9;♦X;♦J",
            "♥2;♥3;♥4;♥5;♥6",
            "♠A;♥A;♦A;♣A;♠2",
            "♠3;♥3;♦3;♣3;♠5",
            "♠A;♥A;♦A;♣K;♠K",
            "♥2;♠2;♦2;♠5;♥5",
            "♠A;♠2;♠4;♠7;♠K",
            "♦9;♦Q;♦K;♦2;♦X",
            "♥3;♥5;♥6;♥8;♥J",
            "♠X;♥J;♦Q;♣K;♦A",
            "♠5;♥6;♦7;♣8;♠9",
            "♦A;♠2;♥3;♠4;♦5",
            "♠A;♥A;♦A;♣5;♠8",
            "♥K;♦K;♠K;♣A;♥Q",
            "♠K;♥K;♦K;♣7;♥9",
            "♠A;♥A;♦J;♣J;♠5",
            "♠Q;♦Q;♠7;♥7;♠3",
            "♥Q;♠Q;♦3;♣3;♥X",
            "♠A;♥A;♠K;♣3;♦2",
            "♣K;♦K;♠9;♥5;♠7",
            "♥Q;♠Q;♦A;♠4;♥2",
            "♥A;♦K;♠9;♣8;♥6",
            "♠A;♥7;♦3;♣4;♠5",
            "♠K;♦2;♠J;♥X;♦7",
        ];

        let mut i = 0;
        while i < ranks.len() - 1 {
            assert!(
                HandRank::from_cards_str(ranks[i]) > HandRank::from_cards_str(&ranks[i + 1]),
                "Assertion failed at index {}: {:?} is not greater than {:?}",
                i,
                ranks[i],
                ranks[i + 1]
            );
            i += 1;
        }
    }
}
