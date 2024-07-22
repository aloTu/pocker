use crate::card::Card;
use crate::hand_rank::HandRank;
use crate::utils::read_command;
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

    pub fn place_bet(&mut self, amount: u32) -> PlayerStatus {
        let current_bet = match self.status {
            PlayerStatus::Betting(s) => Some(s),
            _ => None,
        };

        let current_bet = current_bet.expect("Player is not gaming");
        // fold check call raise allin
        let self_chips = self.chips;
        let mut available_actions = vec!["fold"]; // 总是可以选择弃牌
        match current_bet.cmp(&amount) {
            std::cmp::Ordering::Equal => {
                // 如果当前下注等于上一轮的下注，可以选择检查（Check）
                available_actions.push("check");
                // 如果有筹码可以选择加注或全下
                if self_chips > 0 {
                    available_actions.push("raise");
                    available_actions.push("allin");
                }
            }
            std::cmp::Ordering::Less => {
                // 如果当前下注小于上一轮的下注，说明上家加注了
                if self_chips > amount - current_bet {
                    // 如果筹码足够，则可以跟注或加注
                    available_actions.push("call");
                    available_actions.push("raise");
                }
                // 不论筹码数，总可以选择全压
                available_actions.push("allin");
            }
            std::cmp::Ordering::Greater => {
                // 这种情况理论上不应出现在正常的扑克游戏中
                // 可以考虑记录错误或者处理异常情况
            }
        }

        PlayerStatus::Betting(amount)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Suit;

    #[test]
    fn test_receive_card() {
        let mut player = Player::new(1000);
        let card = Card::new(10, Suit::Hearts);
        player.receive_card(card);
        assert_eq!(player.hand.len(), 1);
        assert_eq!(player.hand[0], card);
    }

    #[test]
    fn test_best_hand() {
        let mut player = Player::new(1000);
        player.receive_card(Card::new(2, Suit::Clubs));
        player.receive_card(Card::new(11, Suit::Hearts));

        let community_cards = vec![
            Card::new(12, Suit::Hearts),
            Card::new(13, Suit::Hearts),
            Card::new(14, Suit::Hearts),
            Card::new(3, Suit::Clubs),
            Card::new(10, Suit::Hearts),
        ];

        let best_hand = player.best_hand(&community_cards);
        assert_eq!(best_hand, HandRank::RoyalFlush);
    }

    #[test]
    fn test_bet() {
        // let mut player = Player::new(1000);
        // player.place_bet(200);
        // let current_bet = match player.status {
        //     PlayerStatus::Betting(s) => Some(s),
        //     _ => None,
        // };

        // let current_bet = current_bet.expect("Player is not gaming");

        // assert_eq!(player.chips, 800);
        // assert_eq!(current_bet, 200);
        // player.reset_bet();
        // assert_eq!(player.chips, 800);
        // let current_bet = match player.status {
        //     PlayerStatus::Betting(s) => Some(s),
        //     _ => None,
        // };

        // let current_bet = current_bet.expect("Player is not gaming");
        // assert_eq!(current_bet, 0);
    }
}
