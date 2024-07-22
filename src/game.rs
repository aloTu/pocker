use itertools::Itertools;

// src/game.rs
use crate::card::{Card, Deck};
use crate::hand_rank::HandRank;
use crate::player::{Player, PlayerStatus};

pub struct Game {
    pub deck: Deck,
    pub players: Vec<Player>,
    pub community_cards: Vec<Card>,
    pub pot: u32,
    pub small_blind_position: usize,
}

const SMALL_BLIND: u32 = 10;

impl Game {
    pub fn new(player_count: usize, initial_chips: u32) -> Self {
        let mut deck = Deck::new();
        deck.shuffle();
        let mut players = Vec::with_capacity(player_count);
        for _ in 0..player_count {
            players.push(Player::new(initial_chips));
        }
        Self {
            deck,
            players,
            community_cards: Vec::with_capacity(5),
            pot: 0,
            small_blind_position: 0,
        }
    }

    pub fn deal_to_players(&mut self) {
        for player in &mut self.players {
            for _ in 0..2 {
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

    pub fn place_bets(&mut self, round: u32) {
        let mut current_rasie_position = 0;
        let mut active_players: Vec<&mut Player> = self
            .players
            .iter_mut()
            .filter(|player| matches!(player.status, PlayerStatus::Betting(_)))
            .collect();
        // blinds
        if round == 0 {
            active_players[0].place_bet(SMALL_BLIND);
            active_players[1].place_bet(SMALL_BLIND * 2);
            current_rasie_position = 2;
        }
        let mut current_bet = match active_players[0].status {
            PlayerStatus::Betting(s) => s,
            _ => panic!("Error, wrong player"),
        };
        //TODO 根据current_rasie_position 判断下注轮次
        loop {
            active_players.rotate_left(current_rasie_position);
            active_players.retain(|player| matches!(player.status, PlayerStatus::Betting(_)));
            for (i, player) in active_players.iter_mut().enumerate() {
                let hand = player.place_bet(current_bet);
                match hand {
                    PlayerStatus::Betting(_num) => {}
                    PlayerStatus::Folded(_num) => {}
                    _ => (),
                };
            }
            let noEnd = active_players.iter().any(|player| {
                if let PlayerStatus::Betting(bet) = player.status {
                    bet != self.pot
                } else {
                    false
                }
            });
            if current_rasie_position == 0 {
                break;
            }
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
        //check balance

        self.deal_to_players();
        for player in &self.players {
            player.show_hand();
        }

        // Preflop round
        self.place_bets(0);

        for _ in 0..3 {
            self.deal_community_card();
        }
        // Flop

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_creation() {
        let mut game = Game::new(4, 1000);
        game.deal_to_players();
        assert_eq!(game.players.len(), 4);
        assert_eq!(game.deck.cards.len(), 52 - 4 * 2); // 4 players, each dealt 2 cards
    }

    #[test]
    fn test_play_round() {
        let mut game = Game::new(4, 1000);
        game.play_round();
        assert_eq!(game.community_cards.len(), 5);
    }
}
