// src/game.rs
use crate::card::{Card, Deck};
use crate::hand_rank::HandRank;
use crate::player::Player;

pub struct Game {
    pub deck: Deck,
    pub players: Vec<Player>,
    pub community_cards: Vec<Card>,
}

impl Game {
    pub fn new(player_count: usize) -> Self {
        let mut deck = Deck::new();
        deck.shuffle();
        let mut players = Vec::with_capacity(player_count);
        for _ in 0..player_count {
            players.push(Player::new());
        }
        Self {
            deck,
            players,
            community_cards: Vec::with_capacity(5),
        }
    }

    pub fn deal_to_players(&mut self) {
        for _ in 0..2 {
            for player in &mut self.players {
                if let Some(card) = self.deck.deal() {
                    player.receive_card(card);
                }
            }
        }
    }

    pub fn deal_community_card(&mut self) {
        if let Some(card) = self.deck.deal() {
            self.community_cards.push(card);
        }
    }

    pub fn show_community_cards(&self) {
        let result = self
            .community_cards
            .iter()
            .map(|card| card.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        println!("{}", result);
    }

    pub fn play_round(&mut self) {
        self.deal_to_players();
        for player in self.players.iter() {
            player.show_hand();
        }

        // Flop
        for _ in 0..3 {
            self.deal_community_card();
        }

        // Turn
        self.deal_community_card();

        // River
        self.deal_community_card();

        self.show_community_cards();

        let winner = self.determine_winner();
        println!("Winner is player {}", winner);
    }

    pub fn determine_winner(&self) -> usize {
        let mut best_hand = HandRank::HighCard(0, 0, 0, 0, 0);
        let mut winner = 0;

        for (i, player) in self.players.iter().enumerate() {
            let hand_rank = player.best_hand(&self.community_cards);
            println!("Player {} hand rank: {:?}", i, hand_rank);
            if hand_rank > best_hand {
                best_hand = hand_rank;
                winner = i;
            }
        }
        winner
    }
}
