use crate::card::Card;
use crate::hand_rank::HandRank;
use itertools::Itertools;

#[derive(Debug)]
pub enum PlayerStatus {
    Folded(u32),
    Allin(u32),
    Betting(u32),
    Waiting,
}

#[derive(Debug)]
pub struct Player {
    pub hand: Vec<Card>,
    pub chips: u32,
    pub status: PlayerStatus,
    pub position: u32,
}

impl Player {
    pub fn new(chips: u32) -> Self {
        Self {
            hand: Vec::with_capacity(2),
            chips,
            status: PlayerStatus::Waiting,
            position: 0,
        }
    }

    pub fn receive_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn place_bet(&mut self, amount: u32) -> u32 {
        let current_bet = match self.status {
            PlayerStatus::Betting(s) => Some(s),
            _ => None,
        };

        let current_bet = current_bet.expect("Player is not gaming");
        if amount <= self.chips {
            self.chips -= amount;
            self.status = PlayerStatus::Betting(current_bet + amount);
            amount
        } else {
            let all_in = self.chips;
            self.chips = 0;
            self.status = PlayerStatus::Betting(all_in);
            all_in
        }
    }

    pub fn reset_bet(&mut self) {
        self.status = PlayerStatus::Waiting;
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
