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
            let hand_card = (self.deck.deal(), self.deck.deal());
            player.receive_card(hand_card);
        }
    }

    pub fn deal_community_card(&mut self) {
        self.community_cards.push(self.deck.deal());
    }

    pub fn blind(&mut self) {}

    pub fn place_bets(&mut self, first_round: bool) {
        let mut active_players: Vec<&mut Player> = self
            .players
            .iter_mut()
            .filter(|player| matches!(player.status, PlayerStatus::Betting(_)))
            .collect();

        let mut current_rasie_position = 0;
        let mut mini_bet = match active_players[0].status {
            PlayerStatus::Betting(s) => s,
            _ => unreachable!(),
        };

        // blinds
        if first_round {
            println!("before blinds");
            for player in &active_players {
                player.show_hand()
            }

            active_players[0].chips -= SMALL_BLIND;
            active_players[0].status = PlayerStatus::Betting(SMALL_BLIND);
            active_players[1].chips -= SMALL_BLIND * 2;
            active_players[1].status = PlayerStatus::Betting(SMALL_BLIND * 2);
            current_rasie_position = 2;
            mini_bet = SMALL_BLIND * 2;
            println!("after blinds");
            for player in &active_players {
                player.show_hand()
            }
        }

        //TODO 根据current_rasie_position 判断下注轮次
        loop {
            active_players.rotate_left(current_rasie_position);
            current_rasie_position = 0;
            active_players.retain(|player| matches!(player.status, PlayerStatus::Betting(_)));
            for (i, player) in active_players.iter_mut().enumerate() {
                let num = player.place_bet(mini_bet);
                if num > mini_bet {
                    // raise
                    current_rasie_position = i;
                    mini_bet = num;
                }
            }
            if current_rasie_position == 0 {
                break;
            }
        }
        if first_round {
            println!("after first round");
            for player in &active_players {
                player.show_hand()
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
        //TODO: 余额不足需要购买筹码
        for player in &self.players {
            if player.chips == 0 {
                println!("Player {} is out of chips", player.position);
            }
        }

        self.deal_to_players();
        for player in &self.players {
            player.show_hand();
        }

        //pre-flop betting
        self.place_bets(true);

        //flop
        for _ in 0..3 {
            self.deal_community_card();
        }
        self.place_bets(false);

        // Turn
        self.deal_community_card();
        self.place_bets(false);

        // River
        self.deal_community_card();
        self.place_bets(false);

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
        println!("hhh");
        let mut game = Game::new(4, 1000);
        game.play_round();
        for player in game.players {
            player.show_hand()
        }
    }
}
