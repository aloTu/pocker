use crate::card::Card;
use crate::hand_rank::HandRank;
use itertools::Itertools;

#[derive(Debug)]
pub struct Player {
    hand: Vec<Card>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            hand: Vec::with_capacity(2),
        }
    }

    pub fn receive_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn show_hand(&self) {
        println!("{}, {}", self.hand[0], self.hand[1]);
    }

    pub fn best_hand(&self, community_cards: &[Card]) -> HandRank {
        let mut all_cards = self.hand.clone();
        all_cards.extend_from_slice(community_cards);
        let mut best_rank = HandRank::HighCard(0, 0, 0, 0, 0);
        for combination in all_cards.iter().combinations(5) {
            let rank =
                HandRank::from_cards(&combination.iter().map(|&&card| card).collect::<Vec<Card>>());
            if rank > best_rank {
                best_rank = rank;
            }
        }
        best_rank
    }
}
