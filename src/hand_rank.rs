use crate::card::Card;

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
    pub fn from_cards(cards: &[Card]) -> Self {
        let mut ranks: Vec<u8> = cards.iter().map(|card| card.rank).collect();
        ranks.sort_unstable_by(|a, b| b.cmp(a));
        println!("{:?}", ranks);

        let is_flush = cards.iter().all(|card| card.suit == cards[0].suit);
        let is_straight = ranks.windows(2).all(|w| w[0] == w[1] + 1);

        if is_flush && is_straight && ranks[0] == 14 {
            return HandRank::RoyalFlush;
        }
        if is_flush && is_straight {
            return HandRank::StraightFlush(ranks[0]);
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
            return HandRank::Straight(ranks[0]);
        }

        HandRank::HighCard(ranks[0], ranks[1], ranks[2], ranks[3], ranks[4])
    }
}
